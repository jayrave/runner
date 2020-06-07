use runner_core::input::{Event, Keycode};
use runner_core::resources::EventQueue;
use sdl2::event::Event as SdlEvent;
use sdl2::keyboard::Keycode as SdlKeycode;
use sdl2::EventPump;

pub struct InputManager {
    event_pump: EventPump,
}

impl InputManager {
    pub fn new(event_pump: EventPump) -> InputManager {
        InputManager { event_pump }
    }

    pub fn reset_and_populate(&mut self, event_queue: &mut EventQueue) {
        event_queue.reset();
        for event in self.event_pump.poll_iter() {
            match event {
                SdlEvent::Quit { .. } => event_queue.add_event(Event::Quit),
                SdlEvent::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    SdlKeycode::Up => event_queue.add_event(Event::KeyDown(Keycode::Up)),
                    SdlKeycode::Down => event_queue.add_event(Event::KeyDown(Keycode::Down)),
                    SdlKeycode::Left => event_queue.add_event(Event::KeyDown(Keycode::Left)),
                    SdlKeycode::Right => event_queue.add_event(Event::KeyDown(Keycode::Right)),
                    SdlKeycode::Escape => event_queue.add_event(Event::KeyDown(Keycode::Escape)),
                    SdlKeycode::Space => event_queue.add_event(Event::KeyDown(Keycode::Space)),
                    _ => {}
                },
                SdlEvent::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    SdlKeycode::Up => event_queue.add_event(Event::KeyUp(Keycode::Up)),
                    SdlKeycode::Down => event_queue.add_event(Event::KeyUp(Keycode::Down)),
                    SdlKeycode::Left => event_queue.add_event(Event::KeyUp(Keycode::Left)),
                    SdlKeycode::Right => event_queue.add_event(Event::KeyUp(Keycode::Right)),
                    SdlKeycode::Escape => event_queue.add_event(Event::KeyUp(Keycode::Escape)),
                    SdlKeycode::Space => event_queue.add_event(Event::KeyUp(Keycode::Space)),
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
