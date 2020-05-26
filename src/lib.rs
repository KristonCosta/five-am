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
use crate::component::{Name, Inventory, Position, Renderable};
use crate::client::Serdent;

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

    type EType = u64;

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    pub struct Serdent(pub EType);

    impl From<Entity> for Serdent {
        fn from(entity: Entity) -> Self {
            let etype: EType = unsafe { std::mem::transmute(entity) };
            Serdent(etype)
        }
    }

    impl Into<Entity> for Serdent {
        fn into(self) -> Entity {
            unsafe { std::mem::transmute(self.0) }
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
            // Argument list used by the editor for GUI and generation of GDScript handlers. It can be omitted if the signal is only used from code.
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
            // Argument list used by the editor for GUI and generation of GDScript handlers. It can be omitted if the signal is only used from code.
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
            // Argument list used by the editor for GUI and generation of GDScript handlers. It can be omitted if the signal is only used from code.
            args: &[
                init::SignalArgument {
                    name: "entities",
                    default: Variant::default(),
                    export_info: init::ExportInfo::new(VariantType::VariantArray),
                    usage: init::PropertyUsage::DEFAULT,
                },
            ],
        });
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

        _owner.emit_signal(
            GodotString::from_str("map_loaded"),
            &[variant.to_variant(), Variant::from_u64(map.size.x as u64)]
        );
    }

    unsafe fn emit_entities(&self, mut _owner: Node, signal: &str, entities: Vec<u64>) {
        _owner.emit_signal(
            GodotString::from_str(signal),
            &[entities.to_variant()]
        );
    }

    #[export]
    fn _ready(&self, mut _owner: Node) {
        unsafe {
            self.emit_map(_owner);
        }
    }

    #[export]
    unsafe fn _physics_process(&mut self, _owner: Node, delta: f64) {
        self.server.tick();
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
    unsafe fn try_move(&mut self, _owner: Node, variant: Variant) {
        match variant.get_type() {
            VariantType::Vector2 => {
                let delta = variant.to_vector2();
                self.server.try_move_player(delta.x as i32, delta.y as i32);
            }
            _ => ()
        }
    }
}

fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<LogicController>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
