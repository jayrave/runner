use crate::resources::EventQueue;
use crate::resources::GameFinisher;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use specs::System;
use specs::WriteExpect;

pub struct EventSystem;

impl<'a> System<'a> for EventSystem {
    type SystemData = (WriteExpect<'a, EventQueue>, WriteExpect<'a, GameFinisher>);

    fn run(&mut self, mut data: Self::SystemData) {
        for event in data.0.queue.drain(..) {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => data.1.finish(),
                _ => {}
            }
        }
    }
}
