use crate::geom::Rect;
use crate::map::{Map, TileType};
use crate::server::map_builders::{BaseMapBuilder, BuiltMap};
use rand::prelude::ThreadRng;

pub struct ShopBuilder;

impl BaseMapBuilder for ShopBuilder {
    fn build(&mut self, _: &mut ThreadRng, build_data: &mut BuiltMap) {
        let size: (i32, i32) = build_data.map.size.to_tuple();
        let map = &mut build_data.map;
        create_room(
            map,
            &Rect::new((1, 1).into(), (size.0 - 2, size.1 - 2).into()),
        );

        build_data.starting_position = Some((size.0 / 2, size.1 / 2).into());
    }
}

pub fn create_room(map: &mut Map, room: &Rect) {
    let (x_start, y_start) = room.origin.to_tuple();
    let (x_end, y_end) = (
        (room.origin.x + room.size.width),
        (room.origin.y + room.size.height),
    );
    for x in x_start..x_end {
        for y in y_start..y_end {
            let index = map.coord_to_index(x, y);
            map.tiles[index] = TileType::Floor;
            map.blocked[index] = false;
        }
    }
}
