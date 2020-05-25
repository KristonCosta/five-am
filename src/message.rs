use crate::color::Color;
use crate::geom::Point;
use legion::prelude::Entity;

pub enum Message {
    GameEvent(String, Option<Color>, Option<Color>),
}

pub enum Action {
    Give {
        source: Entity,
        destination: Entity,
        item: Entity,
    },
    Take {
        source: Entity,
        destination: Entity,
        item: Entity,
    },
    Move {
        entity: Entity,
        delta: Point,
    },
}
