use crate::geom::Vector;
use crate::server::map_builders::basic_builders::SimpleMapBuilder;
use crate::server::map_builders::drunkard::DrunkardsWalkBuilder;
use crate::server::map_builders::{BuiltMap, MapBuilder};
use rand::prelude::ThreadRng;
use super::shop_builder::ShopBuilder;

pub fn random_builder(size: Vector, depth: i32, rng: &mut ThreadRng) -> BuiltMap {
    MapBuilder::new(size, depth, SimpleMapBuilder)
        //    .keep_history()
        .build(rng)
}

pub fn drunk_builder(size: Vector, depth: i32, rng: &mut ThreadRng) -> BuiltMap {
    MapBuilder::new(
        size,
        depth,
        DrunkardsWalkBuilder {
            lifetime: 400,
            floor_percent: 0.6,
            brush_size: 1,
        },
    )
    // .keep_history()
    .build(rng)
}

pub fn shop_builder(size: Vector, rng: &mut ThreadRng) -> BuiltMap {
    MapBuilder::new(size, 0, ShopBuilder).build(rng)
}