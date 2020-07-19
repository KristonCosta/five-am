use crate::color::Color;
use crate::geom::Point;
use legion::prelude::Entity;
use crate::server::resources::trade_handler::{Trade, TradeMessage};

#[derive(Clone)]
pub enum Message {
    TradeEvent(Trade),
    LogEvent(String)
}

#[derive(Clone)]
pub enum Action {
    TradeUpdate(TradeMessage),
    Transaction {
        source: Entity,
        target: Entity,
        object: Entity,
        value: u32
    }
}
