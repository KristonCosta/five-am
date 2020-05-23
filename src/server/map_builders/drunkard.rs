use crate::geom::Point;
use crate::map::TileType;
use crate::server::map_builders::{BaseMapBuilder, BuiltMap};
use rand::prelude::ThreadRng;
use rand::Rng;

pub struct DrunkardsWalkBuilder {
    pub lifetime: u32,
    pub floor_percent: f32,
    pub brush_size: i32,
}
// https://bfnightly.bracketproductions.com/rustbook/chapter_36.html
impl BaseMapBuilder for DrunkardsWalkBuilder {
    fn build(&mut self, rng: &mut ThreadRng, build_data: &mut BuiltMap) {
        let starting_position: Point = ((build_data.map.size) / 2).to_tuple().into();
        let total_tiles = build_data.map.size.x * build_data.map.size.y;
        let desired_floor = (self.floor_percent * total_tiles as f32) as usize;
        let mut digger_count = 0;
        let mut floor_tile_count = build_data
            .map
            .tiles
            .iter()
            .filter(|tile| **tile == TileType::Floor)
            .count();
        build_data.starting_position = Some(starting_position.clone());
        while floor_tile_count < desired_floor {
            let mut did_something = false;
            let mut position = match digger_count {
                0 => starting_position.clone(),
                _ => (
                    rng.gen_range(1, build_data.map.size.x - 3) + 1,
                    rng.gen_range(1, build_data.map.size.y - 3) + 1,
                )
                    .into(),
            };
            let mut current_life = self.lifetime;
            while current_life > 0 {
                if build_data.map.get_type(position) == TileType::Wall {
                    did_something = true;
                }
                build_data.map.set_type(position, TileType::Digging);
                let stagger = rng.gen_range(0, 4);
                match stagger {
                    0 => {
                        if position.x > 1 {
                            position.x -= 1
                        }
                    }
                    1 => {
                        if position.x < build_data.map.size.x - 2 {
                            position.x += 1
                        }
                    }
                    2 => {
                        if position.y > 1 {
                            position.y -= 1
                        }
                    }
                    _ => {
                        if position.y < build_data.map.size.y - 2 {
                            position.y += 1
                        }
                    }
                }
                current_life -= 1;
            }
            if did_something {
                build_data.take_snapshot();
            }

            digger_count += 1;
            for t in build_data.map.tiles.iter_mut() {
                if *t == TileType::Digging {
                    *t = TileType::Floor;
                }
            }
            floor_tile_count = build_data
                .map
                .tiles
                .iter()
                .filter(|tile| **tile == TileType::Floor)
                .count();
        }
    }
}

impl DrunkardsWalkBuilder {}
