use sdl2::event::Event;
use sdl2::EventPump;
use std::collections::VecDeque;

pub struct EventQueue {
    pub queue: VecDeque<Event>,
}

impl EventQueue {
    pub fn new() -> EventQueue {
        EventQueue {
            queue: VecDeque::new(),
        }
    }

    pub fn populate(&mut self, event_pump: &mut EventPump) {
        // We are doing this since `EventPump` is not send & so can't be
        // used directly in a system
        for event in event_pump.poll_iter() {
            self.queue.push_back(event.clone());
        }
    }
}
