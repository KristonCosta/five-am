
use legion::prelude::*;
use crate::server::resources::trade_handler::{TradeHandler, TradeState};
use crate::server::resources::message_queue::MessageQueue;
use crate::server::resources::action_queue::ActionQueue;
use crate::message::{Action, Message};
use crate::component::{Tradeable, Inventory};
use crate::message::Action::Transaction;


pub fn transaction_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("transaction_system")
        .write_resource::<MessageQueue>()
        .write_resource::<ActionQueue>()
        .with_query(<(Write<Inventory>)>::query())
        .build(move |command_buffer, mut world, (message_queue, action_queue), _| {
            let action_queue: &mut ActionQueue = action_queue;
            let message_queue: &mut MessageQueue = message_queue;
            for action in action_queue.get_actions() {
                match action {
                    Action::Transaction {
                        source,
                        target,
                        object,
                        value
                    } => {
                        message_queue.push(Message::LogEvent("Committing transaction".to_string()));
                        command_buffer.exec_mut(move |world| {
                            let contents = {
                                let mut inv = world
                                    .get_component_mut::<Inventory>(source)
                                    .unwrap();
                                let index = inv.contents.iter().position(|i| *i == object);
                                index.map(|i| inv.contents.swap_remove(i))
                            };
                            if let Some(contents) = contents {
                                world
                                    .get_component_mut::<Inventory>(target)
                                    .unwrap()
                                    .contents
                                    .push(contents.clone());
                            }
                        });
                    },
                    _ => {}
                }
            }
        })
}
