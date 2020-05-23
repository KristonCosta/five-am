use crate::geom::{Point, Vector};
use legion::prelude::*;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TileType {
    Wall,
    Floor,
    Digging,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TilePos(pub i32, pub i32, pub i32);

impl TilePos {
    fn push_if_under_cost(
        &self,
        map: &Map,
        succ: &mut Vec<(TilePos, i32)>,
        offset: (i32, i32),
        max_cost: i32,
    ) {
        if !map.blocked[map.coord_to_index(self.0 + offset.0, self.1 + offset.1)] {
            let agg_cost = self.2 + 1;
            if agg_cost <= max_cost {
                succ.push((TilePos(self.0 + offset.0, self.1 + offset.1, agg_cost), 1));
            }
        }
    }

    pub fn successors(&self, map: &Map, max_cost: i32) -> Vec<(TilePos, i32)> {
        let (width, height) = map.size.to_tuple();
        let mut succ = vec![];

        if self.1 < height - 1 {
            self.push_if_under_cost(map, &mut succ, (0, 1), max_cost)
        }
        if self.1 > 0 {
            self.push_if_under_cost(map, &mut succ, (0, -1), max_cost)
        }
        if self.0 < width - 1 {
            self.push_if_under_cost(map, &mut succ, (1, 0), max_cost)
        }
        if self.0 > 0 {
            self.push_if_under_cost(map, &mut succ, (-1, 0), max_cost)
        }
        succ
    }
}

#[derive(Default, Clone)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub size: Vector,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
    pub blocked: Vec<bool>,
    pub depth: i32,
    pub tile_content: Vec<Option<Entity>>,
}

// Base taken from https://bfnightly.bracketproductions.com/rustbook/chapter_23.html
impl Map {
    pub fn new(size: (i32, i32), depth: i32) -> Self {
        let (width, height) = (size.0, size.1);
        let total = (width * height) as usize;
        Map {
            tiles: vec![TileType::Wall; total],
            size: (width as i32, height as i32).into(),
            revealed_tiles: vec![false; total],
            visible_tiles: vec![false; total],
            blocked: vec![true; total],
            depth,
            tile_content: vec![None; total],
        }
    }
    pub fn coord_to_index(&self, x: i32, y: i32) -> usize {
        ((y as usize) * self.size.x as usize) + x as usize
    }

    pub fn point_to_index(&self, point: Point) -> usize {
        ((point.y as usize) * self.size.x as usize) + point.x as usize
    }

    pub fn is_blocked(&self, point: Point) -> bool {
        self.blocked[self.coord_to_index(point.x, point.y)]
    }

    pub fn get_type(&self, point: Point) -> TileType {
        self.tiles[self.point_to_index(point)]
    }

    pub fn set_type(&mut self, point: Point, t: TileType) {
        let index = self.point_to_index(point);
        self.tiles[index] = t
    }

    pub fn set_visible(&mut self, point: Point) {
        let index = self.point_to_index(point);
        self.visible_tiles[index] = true;
    }

    pub fn clear_visible(&mut self) {
        for i in 0..self.tiles.len() {
            self.visible_tiles[i] = false
        }
    }

    pub fn set_revealed(&mut self, point: Point) {
        let index = self.point_to_index(point);
        self.revealed_tiles[index] = true;
    }

    pub fn refresh_blocked(&mut self) {
        for (i, tile) in self.tiles.iter_mut().enumerate() {
            self.blocked[i] = *tile == TileType::Wall;
        }
    }
    pub fn refresh_content(&mut self) {
        for i in 0..self.tile_content.len() {
            self.tile_content[i] = None
        }
    }
}
