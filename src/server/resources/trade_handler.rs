use legion::prelude::*;
use std::collections::HashMap;
use crate::server::resources::trade_handler::TradeState::{Rejected, Final};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TradeMessage {
    pub origin: Entity,
    pub request: TradeRequest,
    pub state_change: TradeState
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TradeRequest {
    pub(crate) id: u64
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TradeState {
    Pending,
    Start,
    Offer(u32),
    CounterOffer(u32),
    Rejected,
    Accepted,
    Final(u32),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Trade {
    pub request: TradeRequest,
    pub target: Entity,
    pub buyer: Entity,
    pub seller: Entity,
    pub last_response: Entity,
    pub trade_state: TradeState
}

pub struct TradeHandler {
    next_id: u64,
    active_requests: HashMap<TradeRequest, Trade>,
    finished_requests: HashMap<TradeRequest, Trade>
}

impl TradeHandler {
    pub fn new() -> Self {
        TradeHandler {
            next_id: 0,
            active_requests: HashMap::new(),
            finished_requests: HashMap::new(),
        }
    }

    pub fn start(&mut self, target: Entity, buyer: Entity, seller: Entity, origin: Entity) -> TradeRequest {
        let request = TradeRequest{
            id: self.increment()
        };
        let trade = Trade {
            request,
            target,
            buyer,
            seller,
            last_response: origin,
            trade_state: TradeState::Pending
        };
        self.active_requests.insert(request, trade.clone());
        request
    }

    fn increment(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id

    }

    pub fn get_trade(&self, request: TradeRequest) -> Option<Trade> {
        self.active_requests.get(&request).map(
            |value| value.clone()
        )
    }

    pub fn handle_message(&mut self, message: TradeMessage) -> Option<Trade> {
        let trade = match self.active_requests.get_mut(&message.request) {
            Some(trade) => trade,
            None => return None
        };

        let is_seller = message.origin == trade.seller;
        let is_buyer = message.origin == trade.buyer;
        let not_last_responder = message.origin != trade.last_response;
        let complete = match message.state_change {
            TradeState::Pending => false,
            TradeState::Start => {
                if not_last_responder && trade.trade_state == TradeState::Pending {
                    trade.trade_state = message.state_change;
                    trade.last_response = message.origin;
                    return Some(trade.clone())
                }
                false
            },
            TradeState::Offer(value) => {
                if is_buyer && not_last_responder {
                    if TradeState::Start == trade.trade_state
                        || (std::mem::discriminant(&TradeState::CounterOffer(1)) == std::mem::discriminant(&trade.trade_state)) {
                        trade.trade_state = TradeState::Offer(value);
                        trade.last_response = message.origin;
                        return Some(trade.clone())
                    }
                }
                false
            },
            TradeState::CounterOffer(value) => {
                if is_seller && not_last_responder {
                    if TradeState::Start == trade.trade_state
                        || (std::mem::discriminant(&TradeState::Offer(1)) == std::mem::discriminant(&trade.trade_state)) {
                        trade.trade_state = TradeState::CounterOffer(value);
                        trade.last_response = message.origin;
                        return Some(trade.clone())
                    }
                }
                false
            },
            TradeState::Rejected => {
                trade.trade_state = TradeState::Rejected;
                true
            },
            TradeState::Accepted => {
                if not_last_responder {
                    match trade.trade_state {
                        TradeState::Offer(val) => {
                            trade.trade_state = Final(val);
                            trade.last_response = message.origin;
                            true
                        },
                        TradeState::CounterOffer(val) => {
                            trade.trade_state = Final(val);
                            trade.last_response = message.origin;
                            true
                        },
                        _ => false
                    }
                } else {
                    false
                }
            },
            TradeState::Final(_) => false
        };
        if complete {
            let trade = trade.clone();
            self.finished_requests.insert(message.request, trade.clone());
            self.active_requests.remove(&message.request);
            return Some(trade)
        }
        None
    }
}