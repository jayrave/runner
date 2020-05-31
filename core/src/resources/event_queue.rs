use crate::input::Event;
use std::collections::VecDeque;
use std::iter::Iterator;

/// We are using `EventQueue` instead of straight up using `EventPump`
/// since it is not `Send` & so can't be used directly in a specs `System`
pub struct EventQueue {
    queue: VecDeque<Event>,
}

impl EventQueue {
    pub fn new() -> EventQueue {
        EventQueue {
            queue: VecDeque::new(),
        }
    }

    pub fn reset(&mut self) {
        self.queue.clear()
    }

    pub fn add_event(&mut self, event: Event) {
        self.queue.push_back(event)
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a Event> + 'a {
        self.queue.iter()
    }
}
