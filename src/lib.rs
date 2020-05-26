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
use crate::Event::ClickedOn;
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


type FP = f32;
const MS_PER_UPDATE: FP = 0.5;

#[derive(Debug)]
pub enum Event {
    ClickedOn(Entity)
}

#[derive(Debug)]
pub struct TimeStep {
    last_time: Instant,
    delta_time: FP,
    frame_count: u32,
    frame_time: FP,
}

impl TimeStep {
    // https://gitlab.com/flukejones/diir-doom/blob/master/game/src/main.rs
    // Grabbed this from here
    pub fn new() -> TimeStep {
        TimeStep {
            last_time: Instant::now(),
            delta_time: 0.0,
            frame_count: 0,
            frame_time: 0.0,
        }
    }

    pub fn delta(&mut self) -> FP {
        let current_time = Instant::now();
        let delta = current_time.duration_since(self.last_time).as_micros() as FP * 0.001;
        self.last_time = current_time;
        self.delta_time = delta;
        delta
    }

    // provides the framerate in FPS
    pub fn frame_rate(&mut self) -> Option<u32> {
        self.frame_count += 1;
        self.frame_time += self.delta_time;
        let tmp;
        // per second
        if self.frame_time >= 1000.0 {
            tmp = self.frame_count;
            self.frame_count = 0;
            self.frame_time = 0.0;
            return Some(tmp);
        }
        None
    }
}
/*
pub struct EntityManager {
    atlas_texture: Texture,
    atlas: Atlas,
    entity_to_node: HashMap<Entity, Sprite>,
    owner: Node,
    event_queue: Rc<RefCell<Vec<Event>>>
}

impl EntityManager {
    pub fn new_sprite(&mut self, entity: Entity, c: char) -> Sprite {
        let mut sprite = Sprite::new();

        let region = self.atlas.get(c);
        let position = (region.x * 20.0 , region.y * 40.0).into();
        unsafe {
            sprite.set_texture(self.atlas_texture.cast());
            sprite.set_region(true);

            sprite.set_region_rect(Rect2::new(position, (20.0, 40.0).into()));
            let mut shape: RectangleShape2D = RectangleShape2D::new();
            shape.set_extents((10.0, 20.0).into());

            let mut clickable: Instance<ClickableEntity> = Instance::<ClickableEntity>::new();
            clickable.map_mut(|c, owner| {
                c.set_entity(entity);
                c.set_event_queue(self.event_queue.clone())
            });

            let mut clickable = clickable.into_base();
            clickable.set_pickable(true);
            let owner_clone = clickable.clone();
            let id = clickable.create_shape_owner(owner_clone.cast());
            clickable.shape_owner_add_shape(id, shape.cast());
            sprite.add_child(clickable.cast(), true);
            self.owner.add_child(sprite.cast(), true);
            self.entity_to_node.insert(entity, sprite);
        }
        sprite
    }

    pub fn entity_exists(&self, entity: Entity) -> bool {
        self.entity_to_node.contains_key(&entity)
    }

    pub fn sync(&mut self, world: &mut legion::prelude::World) {
        let query = <(Read<component::Position>, Read<component::Renderable>)>::query();
        for (entity, (position, _)) in query.iter_entities(world) {
            if let Some(sprite ) = self.entity_to_node.get_mut(&entity) {
                unsafe {
                    sprite.set_position((position.x as f32 * 20.0 + 10.0, position.y as f32 * 40.0 + 20.0).into());
                }
            }
        }
    }
}
*/

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

}



#[derive(NativeClass)]
#[inherit(Node)]
pub struct MapNode {
    server: Server,
}

const AUTO_TILE: i64 = 0;

#[methods]
impl MapNode {
    fn _init(mut _owner: Node) -> Self {
        let mut server = Server::new();
        MapNode {
                    server,

        }
    }

    #[export]
    unsafe fn ping(&self, _owner:Node) -> GodotString {
        godot_print!("Pong.");
        GodotString::from_str("Test")
    }

    #[export]
    unsafe fn _physics_process(&mut self, _owner: Node, delta: f64) {
        self.server.tick();
    }
/*
    #[export]
    unsafe fn _process(&mut self, mut _owner: Node, delta: f64) {
        self.timestep.delta()
        if let Some(fps) = self.timestep.frame_rate() {
            godot_print!("FPS {}", fps);
        }
        self.process_queue(_owner);
        let map = self.server.resources.get::<crate::map::Map>().unwrap();
        if !self.loaded_map {
            self.loaded_map = true;
            for x in 0..map.size.x {
                for y in 0..map.size.y {
                    let texture_region = match map.tiles[map.coord_to_index(x, y)] {
                        TileType::Wall => self.atlas.get('#'),
                        TileType::Floor => self.atlas.get('.'),
                        TileType::Digging => self.atlas.get('>'),
                    };
                    self.tile_map.set_cell(
                        x as i64,
                        y as i64,
                        AUTO_TILE,
                        false,
                        false,
                        false,
                        texture_region,
                    );
                }
            }
        }

        let world = &mut self.server.world;
        let query = <(Read<component::Position>, Read<component::Renderable>)>::query();
        for (entity, (position, renderable)) in query.iter_entities(world) {
            if  self.manager.entity_exists(entity) {
                continue;
            }
            let glyph: Glyph = renderable.glyph;
            self.manager.new_sprite(entity, glyph.ch);
        }

        self.manager.sync(world);
    }

    fn process_queue(&self, mut _owner: Node) {
        for event in  self.manager.event_queue.borrow_mut().drain(..) {
            match event {
                ClickedOn(entity) => {
                    let serialized: Serdent = entity.into();
                    unsafe {
                        _owner.emit_signal(
                            GodotString::from_str("clicked_on_entity"),
                            &[Variant::from_u64(serialized.0)]
                        );
                    }
                }
            }
        }
    }
    */

}

fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<MapNode>();


    handle.add_class::<LogicController>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
