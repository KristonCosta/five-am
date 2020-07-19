
use legion::prelude::*;
use crate::server::resources::trade_handler::{TradeHandler, TradeState};
use crate::server::resources::message_queue::MessageQueue;
use crate::server::resources::action_queue::ActionQueue;
use crate::message::{Action, Message};
use crate::component::Tradeable;
use crate::message::Action::Transaction;


pub fn trade_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("trade_system")
        .write_resource::<TradeHandler>()
        .write_resource::<MessageQueue>()
        .write_resource::<ActionQueue>()
        .with_query(<(Write<Tradeable>)>::query())
        .build(move |command_buffer, mut world, (trade_handler, message_queue, action_queue), query| {
            let trade_handler: &mut TradeHandler = trade_handler;
            let action_queue: &mut ActionQueue = action_queue;
            let message_queue: &mut MessageQueue = message_queue;
            for action in action_queue.get_actions() {
                match action {
                    Action::TradeUpdate(message) => {
                        trade_handler.handle_message(message).map(
                            |trade| {
                                match &trade.trade_state {
                                    TradeState::Final(val) => action_queue.push_future(Transaction {
                                        source: trade.seller,
                                        target: trade.buyer,
                                        object: trade.target,
                                        value: val.clone()
                                    }),
                                    _ => {}
                                }
                                if std::mem::discriminant(&TradeState::Final(1)) == std::mem::discriminant(&trade.trade_state) ||
                                    TradeState::Rejected == trade.trade_state {
                                    let trade = trade.clone();
                                    command_buffer.exec_mut(move |world| {
                                        world.get_component_mut::<Tradeable>(trade.buyer).map(|mut tradeable| tradeable.request.take());
                                        world.get_component_mut::<Tradeable>(trade.seller).map(|mut tradeable| tradeable.request.take());
                                    });
                                }
                                message_queue.push(Message::TradeEvent(trade))
                            }
                        );
                    },
                    _ => {}
                }
            }
        })
}
