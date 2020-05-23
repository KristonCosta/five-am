use crate::geom::Rect;
use crate::map::{Map, TileType};
use crate::server::map_builders::{BaseMapBuilder, BuiltMap};
use rand::prelude::ThreadRng;
use rand::Rng;
use std::cmp::{max, min};

pub struct RoomMapBuilder;

impl BaseMapBuilder for RoomMapBuilder {
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

pub struct SimpleMapBuilder;

impl BaseMapBuilder for SimpleMapBuilder {
    fn build(&mut self, rng: &mut ThreadRng, build_data: &mut BuiltMap) {
        SimpleMapBuilder::rooms_and_corridors(rng, build_data);
    }
}

impl SimpleMapBuilder {
    pub fn rooms_and_corridors(rng: &mut ThreadRng, build_data: &mut BuiltMap) {
        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;
        let mut rooms: Vec<Rect> = vec![];
        let size = build_data.map.size.clone();
        for _i in 0..MAX_ROOMS {
            let w = rng.gen_range(MIN_SIZE, MAX_SIZE);
            let h = rng.gen_range(MIN_SIZE, MAX_SIZE);
            let x = rng.gen_range(1, size.x - w - 1);
            let y = rng.gen_range(1, size.y - h - 1);
            let new_room = Rect::new((x, y).into(), (w, h).into());
            let mut ok = true;
            for other_room in rooms.iter() {
                if new_room.intersects(other_room) {
                    ok = false;
                    break;
                }
            }

            if ok {
                create_room(&mut build_data.map, &new_room);
                build_data.take_snapshot();
                if !rooms.is_empty() {
                    let center = new_room.center();
                    let prev = rooms[rooms.len() - 1].center();
                    if rng.gen_range(0, 2) == 1 {
                        dig_horizontal(
                            &mut build_data.map,
                            prev.x as i32,
                            center.x as i32,
                            prev.y as i32,
                        );
                        dig_vertical(
                            &mut build_data.map,
                            prev.y as i32,
                            center.y as i32,
                            center.x as i32,
                        );
                    } else {
                        dig_horizontal(
                            &mut build_data.map,
                            prev.x as i32,
                            center.x as i32,
                            center.y as i32,
                        );
                        dig_vertical(
                            &mut build_data.map,
                            prev.y as i32,
                            center.y as i32,
                            prev.x as i32,
                        );
                    }
                }
                rooms.push(new_room);
                build_data.take_snapshot();
            }
        }
        let center = rooms[0].center();
        build_data.starting_position = Some((center.x as i32, center.y as i32).into());
        build_data.rooms = Some(rooms);
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

pub fn dig_horizontal(map: &mut Map, start: i32, end: i32, y: i32) {
    for x in min(start, end)..=max(start, end) {
        let index = map.coord_to_index(x, y);
        map.tiles[index] = TileType::Floor;
        map.blocked[index] = false;
    }
}

pub fn dig_vertical(map: &mut Map, start: i32, end: i32, x: i32) {
    for y in min(start, end)..=max(start, end) {
        let index = map.coord_to_index(x, y);
        map.tiles[index] = TileType::Floor;
        map.blocked[index] = false;
    }
}
