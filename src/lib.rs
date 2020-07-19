#![feature(vec_remove_item)]

use crate::glyph::Glyph;
use crate::map::TileType;
use crate::server::map_builders::factories::{drunk_builder, random_builder, shop_builder};
use crate::server::map_builders::{BuiltMap, MapBuilder};
use crate::server::server::Server;
use gdnative::*;
use instant::Instant;
use legion::prelude::*;
use legion::query::DefaultFilter;
use std::collections::{HashMap, HashSet};
use std::cell::RefCell;
use std::rc::Rc;
use crate::component::{Name, Inventory, Position, Renderable, DisplayCabinet};
use crate::client::{Serdent, entity_to_u64};
use crate::message::{Message, Action};
use crate::server::resources::trade_handler::{Trade, TradeState, TradeMessage, TradeRequest};

pub mod color;
pub mod component;
pub mod geom;
pub mod glyph;
pub mod map;
pub mod message;
pub mod server;

pub mod client {
    use legion::entity::Entity;
    use serde::{Deserialize, Serialize};
    use std::convert::{From, Into};
    use gdnative::*;
    type EType = u64;

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    pub struct Serdent(pub EType);

    impl From<Entity> for Serdent {
        fn from(entity: Entity) -> Self {
            
            let etype: EType = unsafe { std::mem::transmute(entity) };
            Serdent(etype)
        }
    }

    impl ToVariant for Serdent {
        fn to_variant(&self) -> Variant {
            Variant::from_u64(self.0)
        }
    }

    impl FromVariant for Serdent {
        fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> {
            Ok(Serdent(variant.to_u64()))
        }
    }

    impl Into<Entity> for Serdent {
        fn into(self) -> Entity {
            unsafe { std::mem::transmute(self.0) }
        }
    }

    pub fn entity_to_u64(entity: Entity) -> u64 {
        let serd: Serdent = entity.into();
        serd.0
    }
}

#[derive(ToVariant, FromVariant)]
enum TradeStateDTO {
    Pending,
    Start,
    Offer(u32),
    CounterOffer(u32),
    Rejected,
    Accepted,
    Final(u32),
}

#[derive(ToVariant)]
struct TradeDTO {
    pub request: u64,
    pub target: Serdent,
    pub buyer: Serdent,
    pub seller: Serdent,
    pub last_response: Serdent,
    pub trade_state: TradeStateDTO
}

impl Into<TradeDTO> for Trade {
    fn into(self) -> TradeDTO {
        TradeDTO {
            request: self.request.id,
            target: self.target.into(),
            buyer: self.buyer.into(),
            seller: self.seller.into(),
            last_response: self.last_response.into(),
            trade_state: match self.trade_state {
                TradeState::Pending => TradeStateDTO::Pending,
                TradeState::Start => TradeStateDTO::Start,
                TradeState::Offer(val) => TradeStateDTO::Offer(val),
                TradeState::CounterOffer(val) => TradeStateDTO::CounterOffer(val),
                TradeState::Rejected => TradeStateDTO::Rejected,
                TradeState::Accepted => TradeStateDTO::Accepted,
                TradeState::Final(val) => TradeStateDTO::Final(val),
            }
        }
    }
}

#[derive(FromVariant)]
struct TradeMessageDTO {
    pub origin: Serdent,
    pub request: u64,
    pub state_change: TradeStateDTO
}

impl Into<TradeMessage> for TradeMessageDTO {
    fn into(self) -> TradeMessage {
        TradeMessage {
            origin: self.origin.into(),
            request: TradeRequest {
                id: self.request
            },
            state_change: match self.state_change {
                TradeStateDTO::Pending => TradeState::Pending,
                TradeStateDTO::Start => TradeState::Start,
                TradeStateDTO::Offer(val) => TradeState::Offer(val),
                TradeStateDTO::CounterOffer(val) => TradeState::CounterOffer(val),
                TradeStateDTO::Rejected => TradeState::Rejected,
                TradeStateDTO::Accepted => TradeState::Accepted,
                TradeStateDTO::Final(val) => TradeState::Final(val),
            }
        }
    }
}

struct TrackerResult {
    created: Vec<u64>,
    deleted: Vec<u64>,
}

struct EntityTracker {
    prev_entities: HashSet<u64>
}

impl EntityTracker {
    pub fn track(&mut self, world: &legion::prelude::World) -> TrackerResult {
        let query = <(Read<component::Position>)>::query();
        let current: HashSet<u64> = query.iter_entities(world).map(
            |(entity, (_))| {
                let ser: Serdent = entity.into();
                ser.0
            }
        ).collect();
        let deleted: Vec<u64> = self.prev_entities.difference(&current).cloned().collect();
        let created: Vec<u64> = current.difference(&self.prev_entities).cloned().collect();
        self.prev_entities = current;
        TrackerResult {
            created,
            deleted
        }
    }
}


#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register_signals)]
pub struct LogicController {
    server: Server,
    tracker: EntityTracker
}

#[methods]
impl LogicController {
    fn _init(mut _owner: Node) -> Self {
        let server = Server::new();
        let query = <(Read<component::Position>)>::query();
        godot_print!("Connected to server.");
        LogicController {
            server,
            tracker: EntityTracker {
                prev_entities: HashSet::new()
            }
        }
    }

    fn register_signals(builder: &init::ClassBuilder<Self>) {
        builder.add_signal(init::Signal {
            name: "map_loaded",
            args: &[
                init::SignalArgument {
                    name: "tiles",
                    default: Variant::default(),
                    export_info: init::ExportInfo::new(VariantType::StringArray),
                    usage: init::PropertyUsage::DEFAULT,
                },
                init::SignalArgument {
                    name: "width",
                    default: Variant::default(),
                    export_info: init::ExportInfo::new(VariantType::I64),
                    usage: init::PropertyUsage::DEFAULT,
                },
            ],
        });
        builder.add_signal(init::Signal {
            name: "created_entities",
            args: &[
                init::SignalArgument {
                    name: "entities",
                    default: Variant::default(),
                    export_info: init::ExportInfo::new(VariantType::VariantArray),
                    usage: init::PropertyUsage::DEFAULT,
                },
            ],
        });
        builder.add_signal(init::Signal {
            name: "deleted_entities",
            args: &[
                init::SignalArgument {
                    name: "entities",
                    default: Variant::default(),
                    export_info: init::ExportInfo::new(VariantType::VariantArray),
                    usage: init::PropertyUsage::DEFAULT,
                },
            ],
        });
        builder.add_signal(init::Signal {
            name: "trade_event",
            args: &[
                init::SignalArgument {
                    name: "trade",
                    default: Variant::default(),
                    export_info: init::ExportInfo::new(VariantType::Dictionary),
                    usage: init::PropertyUsage::DEFAULT,
                },
            ],
        });
        builder.add_signal(init::Signal {
            name: "trade_request",
            args: &[

            ]
        })
    }

    unsafe fn emit_map(&self, mut _owner: Node) {
        let map = self.server.resources.get::<crate::map::Map>().unwrap();
        let variant: Vec<String> = map.tiles.iter().map(
            |tile| match tile {
                TileType::Wall => "#".to_string(),
                TileType::Floor => ".".to_string(),
                TileType::Digging => ">".to_string(),
            }
        ).collect();
        godot_print!("Emitting map");
        _owner.emit_signal(
            GodotString::from_str("map_loaded"),
            &[variant.to_variant(), Variant::from_u64(map.size.x as u64)]
        );
        godot_print!("Emitted map");
    }

    unsafe fn emit_entities(&self, mut _owner: Node, signal: &str, entities: Vec<u64>) {
        _owner.emit_signal(
            GodotString::from_str(signal),
            &[entities.to_variant()]
        );
    }

    #[export]
    fn _ready(&self, mut _owner: Node) {
        godot_print!("1: Emitting map");
        unsafe {
            self.emit_map(_owner);
        }
    }

    unsafe fn process_messages(&mut self, mut _owner: Node, messages: Vec<Message>) {
        for message in messages {
            match message {
                Message::TradeEvent(trade) => {
                    godot_print!("Processing trade event");
                    let trade_dto: TradeDTO = trade.into();
                    _owner.emit_signal(
                        GodotString::from_str("trade_event"),
                        &[trade_dto.to_variant()]
                    );
                    godot_print!("Processed");
                },
                Message::LogEvent(event) => {
                    godot_print!("{}", event)
                }
            }
        }
    }

    #[export]
    unsafe fn _physics_process(&mut self, mut _owner: Node, delta: f64) {
        let messages = self.server.tick();
        self.process_messages(_owner, messages);
    }

    #[export]
    fn _process(&mut self, mut _owner: Node, delta: f64) {
        unsafe {
            let result = self.tracker.track(&self.server.world);
            if !result.created.is_empty() {
                self.emit_entities(_owner, "created_entities", result.created);
            }
            if !result.deleted.is_empty() {
                self.emit_entities(_owner, "deleted_entities", result.deleted);
            }
        }
    }

    fn get_entity(variant: Variant) -> Option<Entity> {
        match variant.get_type() {
            VariantType::I64 => {
                let entity: Entity = Serdent(variant.to_u64()).into();
                Some(entity)
            },
            _ => None,
        }
    }

    #[export]
    unsafe fn get_name(&self, _owner:Node, entity: Variant) -> GodotString {
        let world = &self.server.world;
        match Self::get_entity(entity) {
            Some(entity) => world.get_component::<Name>(entity).map_or(GodotString::from_str(""),|name| {
                GodotString::from_str(&name.name)
            }),
            None => GodotString::from_str("")
        }
    }

    #[export]
    unsafe fn get_inventory(&self, _owner: Node, variant: Variant) -> VariantArray {
        let mut res: VariantArray = VariantArray::new();
        let world = &self.server.world;
        match Self::get_entity(variant) {
            Some(entity) => {
                world.get_component::<Inventory>(entity).map(|inventory| {
                    inventory.contents.iter().for_each(|item| {
                        let name = world.get_component::<Name>(*item);
                        let ser: Serdent = (*item).into();
                        match name {
                            Some(name) => {
                                let mut dictionary: Dictionary = Dictionary::new();
                                dictionary.set(&Variant::from_str("name"), &Variant::from_str(&name.name));
                                dictionary.set(&Variant::from_str("entity"), &Variant::from_u64(ser.0));
                                res.push(&dictionary.to_variant());
                            },
                            None => ()
                        };
                    });
                });
            },
            None => ()
        }
        res
    }

    #[export]
    unsafe fn get_position(&self, _owner: Node, variant: Variant) -> Variant {
        let world = &self.server.world;
        let res: Vector2 = match Self::get_entity(variant) {
            Some(entity) => {
                world.get_component::<Position>(entity).map_or((-1.0 as f32, -1.0 as f32).into(),
                  |position| {
                        (position.x as f32, position.y as f32).into()
                  }
                )
            },
            None => (-1.0 as f32, -1.0 as f32).into()
        };
        res.to_variant()
    }

    #[export]
    unsafe fn get_renderable(&self, _owner: Node, variant: Variant) -> GodotString {
        let world = &self.server.world;
        let res: String = match Self::get_entity(variant) {
            Some(entity) => {
                world.get_component::<Renderable>(entity).map_or("".to_string(),
                                                               |renderable| {
                                                                   renderable.glyph.ch.to_string()
                                                               }
                )
            },
            None => "".to_string()
        };
        GodotString::from_str(res)
    }

    #[export]
    unsafe fn is_display_case(&self, _owner: Node, variant: Variant) -> Variant {
        let world = &self.server.world;
        let res: bool = match Self::get_entity(variant) {
            Some(entity) => {
                world.get_tag::<DisplayCabinet>(entity).map_or(false, |_| { true })
            },
            None => false
        };
        res.to_variant()
    }

    #[export]
    unsafe fn try_move(&mut self, _owner: Node, variant: Variant) {
        match variant.get_type() {
            VariantType::Vector2 => {
                let delta = variant.to_vector2();
                self.server.try_move_player(delta.x as i32, delta.y as i32);
            }
            _ => ()
        }
    }

    #[export]
    unsafe fn try_take(&mut self, _owner: Node, variant: Variant) {
        match Self::get_entity(variant) {
            Some(entity) => {
                self.server.try_player_take(entity);
            },
            None => ()
        };
    }

    #[export]
    unsafe fn try_trade(&mut self, _owner: Node) {
        self.server.try_start_trade();
    }

    #[export]
    unsafe fn try_put(&mut self, _owner: Node, target: Variant, item: Variant) {
        if let Some(target) = Self::get_entity(target) {
            if let Some(item) = Self::get_entity(item) {
                self.server.try_player_put(target, item);
            }
        }
    }

    #[export]
    unsafe fn try_trade_handle(&mut self, _owner: Node, request: Variant) {
        let dto: TradeMessageDTO = TradeMessageDTO::from_variant(&request).unwrap();
        let message: TradeMessage = dto.into();
        godot_print!("Handling trade {:?}", message);
        self.server.add_action(Action::TradeUpdate(message));
        godot_print!("Trade pushed to queue");
    }

    #[export]
    unsafe fn get_player(&mut self, _owner: Node) -> Variant {
        let ser: Serdent = self.server.get_player().into();
        ser.0.to_variant()
    }
}

fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<LogicController>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
