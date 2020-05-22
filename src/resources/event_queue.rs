use sdl2::event::Event;
use sdl2::EventPump;
use std::collections::VecDeque;
use std::iter::Iterator;

/// We are using `EventQueue` instead of straight up using `EventPump`
/// since it is not `Send` & so can't be used directly in a specs `System`
pub struct EventQueue {
    pub queue: VecDeque<Event>,
}

impl EventQueue {
    pub fn new() -> EventQueue {
        EventQueue {
            queue: VecDeque::new(),
        }
    }

    pub fn clear(&mut self) {
        self.queue.clear()
    }

    pub fn populate(&mut self, event_pump: &mut EventPump) {
        for event in event_pump.poll_iter() {
            self.queue.push_back(event.clone());
        }
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a Event> + 'a {
        self.queue.iter()
    }
}
