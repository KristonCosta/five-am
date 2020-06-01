use crate::message::Action;

pub struct ActionQueue {
    queue: Vec<Action>
}

impl ActionQueue {
    pub fn new() -> Self {
        Self {
            queue: Vec::new()
        }
    }

    pub fn get_actions(&self) -> Vec<Action> {
        self.queue.clone()
    }

    pub fn clear(&mut self) {
        self.queue.clear()
    }

    pub fn push(&mut self, action: Action) {
        self.queue.push(action)
    }
}