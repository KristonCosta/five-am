use crate::component::{ActiveTurn, Priority, TurnState};
use legion::prelude::*;

pub struct PendingMoves {
    list: Vec<Entity>,
}

impl PendingMoves {
    pub fn new() -> Self {
        Self { list: vec![] }
    }
}

pub fn turn_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("turn_system")
        .write_resource::<PendingMoves>()
        .with_query(<Read<ActiveTurn>>::query())
        .with_query(<Read<Priority>>::query())
        .build(
            move |command_buffer, world, pending_moves, (turn_query, priority_query)| {
                let mut itr = turn_query.iter_entities(world);
                let active_entity = itr.next().clone();
                std::mem::drop(itr);
                let still_active = match &active_entity {
                    Some((entity, active_turn)) => {
                        if active_turn.state == TurnState::DONE {
                            command_buffer.remove_component::<ActiveTurn>(*entity);
                            false
                        } else {
                            true
                        }
                    }
                    None => false,
                };
                std::mem::drop(active_entity);
                if !still_active {
                    if pending_moves.list.is_empty() {
                        let mut priority_tuple = vec![];

                        for (entity, priority) in priority_query.iter_entities_mut(world) {
                            priority_tuple.push((priority.value, entity));
                        }
                        priority_tuple.sort_by_key(|k| k.0);
                        pending_moves.list = priority_tuple.iter().map(|v| v.1).collect();
                    }

                    let next_turn = pending_moves
                        .list
                        .pop()
                        .expect("No entites could take a turn");
                    command_buffer.add_component(
                        next_turn,
                        ActiveTurn {
                            state: TurnState::PENDING,
                        },
                    );
                }
            },
        )
}
