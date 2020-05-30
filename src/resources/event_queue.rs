use crate::input::{Event, Keycode};
use sdl2::event::Event as SdlEvent;
use sdl2::keyboard::Keycode as SdlKeycode;
use sdl2::EventPump;
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

    pub fn reset_and_populate(&mut self, event_pump: &mut EventPump) {
        self.queue.clear();
        for event in event_pump.poll_iter() {
            match event {
                SdlEvent::Quit { .. } => self.queue.push_back(Event::Quit),
                SdlEvent::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    SdlKeycode::Up => self.queue.push_back(Event::KeyDown(Keycode::Up)),
                    SdlKeycode::Down => self.queue.push_back(Event::KeyDown(Keycode::Down)),
                    SdlKeycode::Left => self.queue.push_back(Event::KeyDown(Keycode::Left)),
                    SdlKeycode::Right => self.queue.push_back(Event::KeyDown(Keycode::Right)),
                    SdlKeycode::Escape => self.queue.push_back(Event::KeyDown(Keycode::Escape)),
                    _ => {}
                },
                SdlEvent::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    SdlKeycode::Up => self.queue.push_back(Event::KeyUp(Keycode::Up)),
                    SdlKeycode::Down => self.queue.push_back(Event::KeyUp(Keycode::Down)),
                    SdlKeycode::Left => self.queue.push_back(Event::KeyUp(Keycode::Left)),
                    SdlKeycode::Right => self.queue.push_back(Event::KeyUp(Keycode::Right)),
                    SdlKeycode::Escape => self.queue.push_back(Event::KeyUp(Keycode::Escape)),
                    _ => {}
                },
                _ => {}
            }
        }
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a Event> + 'a {
        self.queue.iter()
    }
}
