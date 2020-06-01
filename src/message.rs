use crate::color::Color;
use crate::geom::Point;
use legion::prelude::Entity;
use crate::server::resources::trade_handler::{Trade, TradeMessage};

#[derive(Copy, Clone)]
pub enum Message {
    TradeEvent(Trade)
}

#[derive(Copy, Clone)]
pub enum Action {
    TradeUpdate(TradeMessage)
}
