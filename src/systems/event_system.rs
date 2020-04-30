use crate::components::input::data::Direction;
use crate::components::input::InputControlled;
use crate::resources::EventQueue;
use crate::resources::GameFinisher;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use specs::join::Join;
use specs::shred::ResourceId;
use specs::SystemData;
use specs::World;
use specs::WriteExpect;
use specs::{System, WriteStorage};
use std::collections::VecDeque;

pub struct EventSystem {
    queue: VecDeque<Event>,
}

impl EventSystem {
    pub fn new() -> EventSystem {
        EventSystem {
            queue: VecDeque::new(),
        }
    }
}

#[derive(SystemData)]
pub struct EventSystemData<'a> {
    event_queue: WriteExpect<'a, EventQueue>,
    game_finisher: WriteExpect<'a, GameFinisher>,
    input_controlled_storage: WriteStorage<'a, InputControlled>,
}

impl<'a> System<'a> for EventSystem {
    type SystemData = EventSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        // Should start with a clean slate every time. We are going to
        // handle all the events from the event queue & each
        // `InputControlled` component must be informed about all the
        // events. Since it would be inefficient to loop over the components
        // for every event, we are collecting the event as the first step
        // & replaying them separately for each component
        self.queue.clear();
        for event in data.event_queue.queue.drain(..) {
            self.queue.push_back(event)
        }

        // Replay events for all components. Instead of finding one up &
        // one down, we want all events to be directly delivered to the
        // components to be true to user input
        let mut should_finish_game = false;
        for input_cntl in (&mut data.input_controlled_storage).join() {
            for event in self.queue.iter() {
                match event {
                    Event::Quit { .. } => should_finish_game = true,
                    Event::KeyDown {
                        keycode: Some(keycode),
                        ..
                    } => match keycode {
                        Keycode::Escape => should_finish_game = true,
                        Keycode::Space | Keycode::Up => input_cntl.update_key_down(Direction::Up),
                        Keycode::Down => input_cntl.update_key_down(Direction::Down),
                        Keycode::Left => input_cntl.update_key_down(Direction::Left),
                        Keycode::Right => input_cntl.update_key_down(Direction::Right),
                        _ => {}
                    },
                    Event::KeyUp {
                        keycode: Some(keycode),
                        ..
                    } => match keycode {
                        Keycode::Space | Keycode::Up => input_cntl.update_key_up(Direction::Up),
                        Keycode::Down => input_cntl.update_key_up(Direction::Down),
                        Keycode::Left => input_cntl.update_key_up(Direction::Left),
                        Keycode::Right => input_cntl.update_key_up(Direction::Right),
                        _ => {}
                    },
                    _ => {}
                }
            }
        }

        if should_finish_game {
            data.game_finisher.finish();
        }
    }
}
