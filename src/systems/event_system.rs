use crate::resources::EventQueue;
use crate::resources::GameFinisher;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use specs::shred::ResourceId;
use specs::System;
use specs::SystemData;
use specs::World;
use specs::WriteExpect;

pub struct EventSystem;

#[derive(SystemData)]
pub struct EventSystemData<'a> {
    event_queue: WriteExpect<'a, EventQueue>,
    game_finisher: WriteExpect<'a, GameFinisher>,
}

impl<'a> System<'a> for EventSystem {
    type SystemData = EventSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        for event in data.event_queue.queue.drain(..) {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => data.game_finisher.finish(),
                _ => {}
            }
        }
    }
}
