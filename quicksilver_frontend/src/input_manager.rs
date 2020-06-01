use quicksilver::input::Event as QsEvent;
use quicksilver::input::Key;
use quicksilver::Input;
use runner_core::input::{Event, Keycode};
use runner_core::resources::EventQueue;

pub struct InputManager {
    input: Input,
}

impl InputManager {
    pub fn new(input: Input) -> InputManager {
        InputManager { input }
    }

    pub async fn reset_and_populate(&mut self, event_queue: &mut EventQueue) {
        event_queue.reset();
        while let Some(event) = self.input.next_event().await {
            if let QsEvent::KeyboardInput(keyboard_event) = event {
                if keyboard_event.is_down() {
                    match keyboard_event.key() {
                        Key::Escape => event_queue.add_event(Event::KeyDown(Keycode::Escape)),
                        Key::Up => event_queue.add_event(Event::KeyDown(Keycode::Up)),
                        Key::Down => event_queue.add_event(Event::KeyDown(Keycode::Down)),
                        Key::Left => event_queue.add_event(Event::KeyDown(Keycode::Left)),
                        Key::Right => event_queue.add_event(Event::KeyDown(Keycode::Right)),
                        _ => {}
                    }
                } else {
                    match keyboard_event.key() {
                        Key::Escape => event_queue.add_event(Event::KeyUp(Keycode::Escape)),
                        Key::Up => event_queue.add_event(Event::KeyUp(Keycode::Up)),
                        Key::Down => event_queue.add_event(Event::KeyUp(Keycode::Down)),
                        Key::Left => event_queue.add_event(Event::KeyUp(Keycode::Left)),
                        Key::Right => event_queue.add_event(Event::KeyUp(Keycode::Right)),
                        _ => {}
                    }
                }
            }
        }
    }
}
