use gdnative::*;
use crate::frontend::Atlas;
use crate::server::map_builders::{MapBuilder, BuiltMap};
use crate::server::map_builders::factories::{shop_builder, drunk_builder};
use crate::map::TileType;

pub mod server;
pub mod geom;
pub mod map;

pub mod frontend {
    use std::collections::HashMap;
    use gdnative::Vector2;

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
                map
            }
        }

        pub fn get(&self, c: char) -> Vector2 {
            self.map.get(&c).unwrap().clone()
        }
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
                let current_offset = _owner.get_offset();
                let delta_offset: Vector2 = match key.get_scancode() {
                    GlobalConstants::KEY_W => (0.0, -20.0),
                    GlobalConstants::KEY_S => (0.0, 20.0),
                    GlobalConstants::KEY_A => (-20.0, 0.0),
                    GlobalConstants::KEY_D => (20.0, 0.0),
                    _ => (0.0, 0.0)
                }.into();
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


#[derive(NativeClass)]
#[inherit(Node)]
pub struct MapNode {
    tile_map: TileMap,
    atlas: Atlas,
    map: BuiltMap,
    #[export]
    value: i32
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

        let mut rng = rand::thread_rng();
        let map = drunk_builder((200, 200).into(), 0, &mut rng);

        unsafe {
            tile_map.set_cell_size((20.0, 40.0).into());
            tile_map.set_tileset(Some(tile_set));

            for x in 0..map.map.size.x {
                for y in 0..map.map.size.y {
                    let texture_region = match map.map.tiles[map.map.coord_to_index(x, y)] {
                        TileType::Wall => atlas.get('#'),
                        TileType::Floor => atlas.get('.'),
                        TileType::Digging => atlas.get('>'),
                    };
                    tile_map.set_cell(x as i64, y as i64, AUTO_TILE, false, false, false, texture_region);
                }
            }
            _owner.add_child(tile_map.cast(), true);
        }


        MapNode {
            tile_map,
            atlas,
            map,
            value:1
        }
    }

    #[export]
    unsafe fn _input(&mut self, _owner: Node, event: Option<InputEvent>) {

    }
    #[export]
    fn _ready(&self, _owner: Node) {
        godot_print!("Loaded Map");
    }
}

fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<MapNode>();
    handle.add_class::<Camera>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();