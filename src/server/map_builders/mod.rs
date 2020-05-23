use crate::geom::{Point, Rect, Vector};
use crate::map::Map;
use rand::prelude::ThreadRng;

pub mod basic_builders;
pub mod drunkard;
pub mod factories;
pub mod shop_builder;

// Most of this taken from https://bfnightly.bracketproductions.com/rustbook/chapter_36.html
pub trait BaseMapBuilder {
    fn build(&mut self, rng: &mut ThreadRng, build_data: &mut BuiltMap);
}

pub trait MetaMapBuilder {
    fn mutate(&mut self, rng: &mut ThreadRng, build_data: &mut BuiltMap);
}

pub struct MapBuilder {
    base: Box<dyn BaseMapBuilder>,
    builders: Vec<Box<dyn MetaMapBuilder>>,
    build_data: BuiltMap,
}

impl MapBuilder {
    pub fn new(size: Vector, depth: i32, base: impl BaseMapBuilder + 'static) -> Self {
        MapBuilder {
            base: Box::new(base),
            builders: vec![],
            build_data: BuiltMap::new(size, depth),
        }
    }

    pub fn keep_history(mut self) -> Self {
        self.build_data.with_history = true;
        self
    }

    pub fn with(mut self, builder: impl MetaMapBuilder + 'static) -> Self {
        self.builders.push(Box::new(builder));
        self
    }

    pub fn build(mut self, rng: &mut ThreadRng) -> BuiltMap {
        self.base.build(rng, &mut self.build_data);
        for mut metabuilder in self.builders.drain(..) {
            metabuilder.mutate(rng, &mut self.build_data)
        }
        self.build_data
    }
}

pub struct BuiltMap {
    pub spawn_list: Vec<(usize, String)>,
    pub map: Map,
    pub starting_position: Option<Point>,
    pub rooms: Option<Vec<Rect>>,
    pub history: Vec<Map>,
    pub with_history: bool,
}

impl BuiltMap {
    pub fn new(size: Vector, depth: i32) -> Self {
        BuiltMap {
            spawn_list: vec![],
            map: Map::new(size.to_tuple(), depth),
            starting_position: None,
            rooms: None,
            history: vec![],
            with_history: false,
        }
    }
}

impl BuiltMap {
    fn take_snapshot(&mut self) {
        if self.with_history {
            let mut snapshot = self.map.clone();
            for tile in snapshot.revealed_tiles.iter_mut() {
                *tile = true;
            }
            self.history.push(snapshot);
        }
    }
}
