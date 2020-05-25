use crate::geom::Point;
use crate::glyph::Glyph;
use legion::prelude::Entity;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Into<Point> for Position {
    fn into(self) -> Point {
        (self.x, self.y).into()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Renderable {
    pub glyph: Glyph,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;

#[derive(Clone, Debug, PartialEq)]
pub struct Name {
    pub name: String,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TurnState {
    PENDING,
    ACTIVE,
    DONE,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ActiveTurn {
    pub state: TurnState,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TileBlocker;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Priority {
    pub value: u8,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DisplayCabinet;

#[derive(Clone, Debug, PartialEq)]
pub struct Inventory {
    pub contents: Vec<Entity>,
    pub capacity: u8,
}
