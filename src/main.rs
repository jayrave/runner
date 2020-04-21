extern crate sdl2;

use specs::World;
use specs::WorldExt;
use specs::Dispatcher;
use specs::DispatcherBuilder;
use sdl2::render::WindowCanvas;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

mod constants;
mod systems;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("runner", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    setup_splash_screen(&mut canvas);
    let (world, mut dispatcher) = setup_ecs(canvas);

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        dispatcher.dispatch(&world);
    }
}

fn setup_splash_screen(canvas: &mut WindowCanvas) {
    // Setup start-up color to prevent showing empty window until
    // the rendering loop starts
    canvas.set_draw_color(constants::SKY_COLOR);
    canvas.clear();
    canvas.present();
}

fn setup_ecs<'a, 'b>(canvas: WindowCanvas) -> (World, Dispatcher<'a, 'b>) {
    let world = World::new();
    let dispatcher = DispatcherBuilder::new()
        .with_thread_local(systems::RenderingSystem::new(canvas))
        .with_thread_local(systems::PowerOptimizerSystem::new())
        .build();

    (world, dispatcher)
}
