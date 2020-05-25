#![feature(vec_remove_item)]
use crate::frontend::Atlas;
use crate::glyph::Glyph;
use crate::map::TileType;
use crate::server::map_builders::factories::{drunk_builder, random_builder, shop_builder};
use crate::server::map_builders::{BuiltMap, MapBuilder};
use crate::server::server::Server;
use gdnative::*;
use instant::Instant;
use legion::prelude::*;
use std::collections::HashMap;
use std::cell::RefCell;
use crate::Event::ClickedOn;
use std::rc::Rc;
use crate::component::Name;
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

#[derive(Debug)]
pub enum Event {
    ClickedOn(Entity)
}

#[derive(NativeClass)]
#[inherit(Area2D)]
pub struct ClickableEntity {
    entity: Option<Entity>,
    event_queue: Option<Rc<RefCell<Vec<Event>>>>
}

#[methods]
impl ClickableEntity {

    fn _init(mut _owner: Area2D) -> Self {
        ClickableEntity {
            entity: None,
            event_queue: None,
        }
    }

    pub fn set_entity(&mut self, entity: Entity) {
        self.entity = Some(entity);
    }

    pub fn set_event_queue(&mut self, queue: Rc<RefCell<Vec<Event>>>) {
        self.event_queue = Some(queue);
    }


    #[export]
    pub fn _input_event(
        &mut self,
        _owner: Area2D,
        viewport: Option<Object>,
        event: Option<InputEvent>,
        shape_idx: i64
    ) {
        if let Some(event) = event {
            let button: Option<InputEventMouseButton> = event.cast();
            if let Some(button) = button {
                if button.is_pressed() {

                    if let Some(queue) = &self.event_queue {
                        (*queue.borrow_mut()).push(ClickedOn(self.entity.unwrap()));
                    }
                }
            }
        }
    }

    #[export]
    unsafe fn notify(&mut self, mut _owner: Area2D) {
        godot_print!("Clicked on area");
    }
}


pub mod frontend {
    use std::collections::HashMap;
    use gdnative::*;

    #[derive(Clone)]
    pub struct Atlas {
        map: HashMap<char, Vector2>,
    }

    static SUPPORTED_CHARS: &str = r#"╦╩═╬╧╨╤╥╙╘╒╓╫╪┘╠┌█▄▌▐▀αßΓπΣσµτΦδ∞φ╟╚╔║╗╝╣╢╖
*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQ⌠⌡≥
RSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxy÷≈
z{|}~⌂ÇüéâäàåçêëèïîìÄÅÉæÆôöòûùÿÖÜ¢£¥₧ƒáí°∙
óúñÑªº¿⌐¬½¼¡«»░▒▓│┤╡╕╜╛┐└┴┬├─┼╞·√±≤ⁿε∩≡ΘΩ
"☺☻♥♦♣♠•◘○◙♂♀♪♫☼►◄↕‼¶§▬↨↑↓→←∟↔▲▼!#$%&'()²■"#;

    impl Atlas {
        pub fn new() -> Self {
            let mut map = HashMap::new();
            let (mut x, mut y) = (0.0, 0.0);
            for c in SUPPORTED_CHARS.chars() {
                if c == '\n' {
                    y += 1.0;
                    x = 0.0;
                } else {
                    map.insert(c, (x, y).into());
                    x += 1.0;
                }
            }
            Atlas {
                map,
            }
        }

        pub fn get(&self, c: char) -> Vector2 {
            self.map.get(&c).unwrap().clone()
        }
    }
}

type FP = f32;
const MS_PER_UPDATE: FP = 0.5;

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

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Camera;

#[methods]
impl Camera {
    fn _init(mut _owner: Node) -> Self {
        Camera
    }
    #[export]
    unsafe fn _input(&mut self, mut _owner: Node, event: Option<InputEvent>) {
        let mut _owner: Camera2D = _owner.cast().unwrap();
        if let Some(event) = event {
            let key: Option<InputEventKey> = event.cast();
            if let Some(key) = key {
                if !key.is_pressed() {
                    return
                }
                let current_offset = _owner.get_offset();
                let delta_offset: Vector2 = match key.get_scancode() {
                    GlobalConstants::KEY_W => (0.0, -20.0),
                    GlobalConstants::KEY_S => (0.0, 20.0),
                    GlobalConstants::KEY_A => (-10.0, 0.0),
                    GlobalConstants::KEY_D => (10.0, 0.0),
                    _ => (0.0, 0.0),
                }
                .into();
                if delta_offset.length() != 0.0 {
                    _owner.set_offset(current_offset + delta_offset);
                }
            }
        }
    }
    #[export]
    fn _ready(&self, _owner: Node) {
        godot_print!("Loaded Camera Controller");
    }
}

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

struct NodeInternal {
    tile_map: TileMap,
    atlas: Atlas,
    manager: EntityManager,
    server: Server,
    timestep: TimeStep,
    turns: u32,
    lag: f32,
    loaded_map: bool,
}

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register_signals)]
pub struct MapNode {
    internal: RefCell<NodeInternal>
}

const AUTO_TILE: i64 = 0;

#[methods]
impl MapNode {
    fn _init(mut _owner: Node) -> Self {
        let str = GodotString::from_str("test.png");
        let mut texture = ImageTexture::new();
        texture.load(str);
        let mut tile_set = TileSet::new();
        tile_set.create_tile(AUTO_TILE);
        tile_set.tile_set_texture(AUTO_TILE, texture.cast());
        let region = Rect2::new((0.0, 0.0).into(), (860.0, 240.0).into());
        tile_set.tile_set_region(AUTO_TILE, region);
        tile_set.tile_set_tile_mode(AUTO_TILE, 2);
        tile_set.autotile_set_size(AUTO_TILE, (20.0, 40.0).into());

        let atlas = Atlas::new();
        let mut tile_map = TileMap::new();
        let mut timestep = TimeStep::new();
        unsafe {
            tile_map.set_cell_size((20.0, 40.0).into());
            tile_map.set_tileset(Some(tile_set));
            tile_map.set_quadrant_size(32);
            _owner.add_child(tile_map.cast(), true);
        }

        let manager = EntityManager {
            atlas_texture: texture.cast().unwrap(),
            atlas: atlas.clone(),
            entity_to_node: HashMap::new(),
            owner: _owner.clone(),
            event_queue: Rc::new(RefCell::new(Vec::new()))
        };

        let mut server = Server::new();
        MapNode {
            internal: RefCell::new(
                NodeInternal {
                    tile_map,
                    atlas,
                    server,
                    timestep,
                    manager,
                    turns: 0,
                    lag: 0.0,
                    loaded_map: false,
                }
            )
        }
    }

    fn register_signals(builder: &init::ClassBuilder<Self>) {
        builder.add_signal(init::Signal {
            name: "clicked_on_entity",
            // Argument list used by the editor for GUI and generation of GDScript handlers. It can be omitted if the signal is only used from code.
            args: &[init::SignalArgument {
                name: "entity",
                default: Variant::from_i64(-1),
                export_info: init::ExportInfo::new(VariantType::I64),
                usage: init::PropertyUsage::DEFAULT,
            }],
        });
    }

    #[export]
    unsafe fn get_name(&self, _owner:Node, entity: Variant) -> GodotString {
        godot_print!("Trying to get name");
        let internal = match self.internal.try_borrow() {
            Ok(internal) => internal,
            Err(err) => {
                godot_print!("Failed to borrow! {:?}", err);
                panic!("GAK");
            }
        };
        let world = &internal.server.world;
        match entity.get_type() {
            VariantType::I64 =>
                {
                    godot_print!("Matched type");
                    let entity: Entity = Serdent(entity.to_u64()).into();
                    godot_print!("Serialized {:?}", entity);
                    world.get_component::<Name>(entity).map_or(GodotString::from_str(""),|name| {
                        GodotString::from_str(&name.name)
                    })
                },
            _ => GodotString::from_str("")
        }

    }

    #[export]
    unsafe fn ping(&self, _owner:Node) -> GodotString {
        godot_print!("Pong.");
        GodotString::from_str("Test")
    }

    #[export]
    unsafe fn _physics_process(&self, _owner: Node, delta: f64) {
        self.internal.borrow_mut().server.tick();
    }

    #[export]
    unsafe fn _process(&self, mut _owner: Node, delta: f64) {
        let mut internal = &mut *self.internal.borrow_mut();
        internal.timestep.delta();
        if let Some(fps) = internal.timestep.frame_rate() {
            godot_print!("FPS {}", fps);
        }
        self.process_queue(_owner, &mut internal);
        let server = &mut internal.server;
        let tile_map = &mut internal.tile_map;
        let resources = & server.resources;
        let map = resources.get::<crate::map::Map>().unwrap();


            for x in 0..map.size.x {
                for y in 0..map.size.y {
                    let texture_region = match map.tiles[map.coord_to_index(x, y)] {
                        TileType::Wall => internal.atlas.get('#'),
                        TileType::Floor => internal.atlas.get('.'),
                        TileType::Digging => internal.atlas.get('>'),
                    };
                    internal.tile_map.set_cell(
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
        let world = &mut server.world;

        let query = <(Read<component::Position>, Read<component::Renderable>)>::query();
        for (entity, (position, renderable)) in query.iter_entities(world) {
            if  internal.manager.entity_exists(entity) {
                continue;
            }
            let glyph: Glyph = renderable.glyph;
            internal.manager.new_sprite(entity, glyph.ch);
        }

        internal.manager.sync(world);
    }

    fn process_queue(&self, mut _owner: Node, internal:  &mut NodeInternal) {
        for event in  internal.manager.event_queue.borrow_mut().drain(..) {
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

    #[export]
    unsafe fn _input(&self, _owner: Node, event: Option<InputEvent>) {
        let internal = &mut *self.internal.borrow_mut();
        if let Some(event) = event {
            let key: Option<InputEventKey> = event.cast();
            if let Some(key) = key {
                if !key.is_pressed() {
                    return
                }
                match key.get_scancode() {
                    GlobalConstants::KEY_W =>  internal.server.try_move_player(0, -1),
                    GlobalConstants::KEY_S =>  internal.server.try_move_player(0, 1),
                    GlobalConstants::KEY_A =>  internal.server.try_move_player(-1, 0),
                    GlobalConstants::KEY_D =>  internal.server.try_move_player(1, 0),
                    _ => false,
                };
            }
        }
    }
    #[export]
    fn _ready(&self, _owner: Node) {
        godot_print!("Loaded Map");
    }
}

fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<MapNode>();
    handle.add_class::<Camera>();
    handle.add_class::<ClickableEntity>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
