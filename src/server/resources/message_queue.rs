use crate::message::Message;

pub struct MessageQueue {
    queue: Vec<Message>
}

impl MessageQueue {
    pub fn new() -> Self {
        Self {
            queue: Vec::new()
        }
    }

    pub fn get_messages(&self) -> Vec<Message> {
        self.queue.clone()
    }

    pub fn clear(&mut self) {
        self.queue.clear()
    }

    pub fn push(&mut self, message: Message) {
        self.queue.push(message)
    }
}