use crate::components::data;
use crate::components::UserControlled;
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

pub struct EventSystem;

#[derive(SystemData)]
pub struct EventSystemData<'a> {
    event_queue: WriteExpect<'a, EventQueue>,
    game_finisher: WriteExpect<'a, GameFinisher>,
    user_controlled_storage: WriteStorage<'a, UserControlled>,
}

impl<'a> System<'a> for EventSystem {
    type SystemData = EventSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        let mut should_finish_game = false;
        let mut user_input = None;
        for event in data.event_queue.queue.drain(..) {
            match event {
                Event::Quit { .. } => should_finish_game = true,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::Escape => should_finish_game = true,
                    Keycode::Space | Keycode::Up => user_input = Some(data::Input::Up),
                    Keycode::Down => user_input = Some(data::Input::Down),
                    _ => {}
                },
                _ => {}
            }
        }

        if should_finish_game {
            data.game_finisher.finish();
        } else {
            for user_controlled in (&mut data.user_controlled_storage).join() {
                if let Some(input) = user_input {
                    user_controlled.update_input_if_required(input);
                }
            }
        }
    }
}
