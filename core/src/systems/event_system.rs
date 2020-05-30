use crate::components::input::data::Direction;
use crate::components::input::InputControlled;
use crate::input::{Event, Keycode};
use crate::resources::EventQueue;
use specs::join::Join;
use specs::shred::ResourceId;
use specs::SystemData;
use specs::World;
use specs::WriteExpect;
use specs::{System, WriteStorage};

pub struct EventSystem;

#[derive(SystemData)]
pub struct EventSystemData<'a> {
    event_queue: WriteExpect<'a, EventQueue>,
    input_controlled_storage: WriteStorage<'a, InputControlled>,
}

impl<'a> System<'a> for EventSystem {
    type SystemData = EventSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        // Replay events for all components. Instead of finding one up &
        // one down, we want all events to be directly delivered to the
        // components to be true to user input
        for input_cntl in (&mut data.input_controlled_storage).join() {
            for event in data.event_queue.iter() {
                match event {
                    Event::KeyDown(keycode) => match keycode {
                        Keycode::Up => input_cntl.update_key_down(Direction::Up),
                        Keycode::Down => input_cntl.update_key_down(Direction::Down),
                        Keycode::Left => input_cntl.update_key_down(Direction::Left),
                        Keycode::Right => input_cntl.update_key_down(Direction::Right),
                        _ => {}
                    },
                    Event::KeyUp(keycode) => match keycode {
                        Keycode::Up => input_cntl.update_key_up(Direction::Up),
                        Keycode::Down => input_cntl.update_key_up(Direction::Down),
                        Keycode::Left => input_cntl.update_key_up(Direction::Left),
                        Keycode::Right => input_cntl.update_key_up(Direction::Right),
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
    }
}
