
use legion::prelude::*;
use crate::server::resources::trade_handler::TradeHandler;
use crate::server::resources::message_queue::MessageQueue;
use crate::server::resources::action_queue::ActionQueue;
use crate::message::{Action, Message};


pub fn trade_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("trade_system")
        .write_resource::<TradeHandler>()
        .write_resource::<MessageQueue>()
        .read_resource::<ActionQueue>()
        .build(move |_, mut world, (trade_handler, message_queue, action_queue), _| {
            let trade_handler: &mut TradeHandler = trade_handler;
            let action_queue: &ActionQueue = action_queue;
            let message_queue: &mut MessageQueue = message_queue;

            for action in action_queue.get_actions() {
                match action {
                    Action::TradeUpdate(message) => {
                        trade_handler.handle_message(message).map(
                            |trade| message_queue.push(Message::TradeEvent(trade))
                        );
                    },
                }
            }
        })
}
