use crate::message::Action;

pub struct ActionQueue {
    queue: Vec<Action>,
    future: Vec<Action>,
}

impl ActionQueue {
    pub fn new() -> Self {
        Self {
            queue: Vec::new(),
            future: Vec::new()
        }
    }

    pub fn get_actions(&self) -> Vec<Action> {
        self.queue.clone()
    }

    pub fn step(&mut self) {
        self.queue.clear();
        let mut current = Vec::new();
        std::mem::swap(&mut self.future, &mut current);
        self.queue = current;
    }

    pub fn push(&mut self, action: Action) {
        self.queue.push(action)
    }

    pub fn push_future(&mut self, action: Action) {
        self.future.push(action)
    }
}