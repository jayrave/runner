extern crate sdl2;

use crate::input_manager::InputManager;
use crate::renderer::Renderer;
use crate::textures::Textures;
use runner_core::components::Drawable;
use runner_core::data::WorldData;
use runner_core::game_loop;
use runner_core::game_loop::{GameLoop, GameLoopResult};
use runner_core::resources::EventQueue;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;
use specs::ReadStorage;

mod color;
mod input_manager;
mod renderer;
mod textures;

pub fn main() {
    let world_data = WorldData::new();
    let (sdl, mut canvas) = build_canvas(world_data);

    setup_splash_screen(world_data, &mut canvas);
    run_game_loop(world_data, sdl, canvas);
}

fn build_canvas(world_data: WorldData) -> (Sdl, WindowCanvas) {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let window = video_subsystem
        .window(
            "Runner",
            world_data.bounds().width(),
            world_data.bounds().height(),
        )
        .position_centered()
        .build()
        .unwrap();

    (sdl, window.into_canvas().build().unwrap())
}

fn setup_splash_screen(world_data: WorldData, canvas: &mut WindowCanvas) {
    // Setup start-up color to prevent showing empty window until
    // the rendering loop starts
    canvas.set_draw_color(color::sdl_color_from(world_data.sky_color()));
    canvas.clear();
    canvas.present();
}

fn run_game_loop(world_data: WorldData, sdl: Sdl, canvas: WindowCanvas) {
    let texture_creator = canvas.texture_creator();
    let textures = Textures::load_from_files(&texture_creator);

    let mut game_loop = GameLoop::new(world_data);
    let mut renderer = Renderer::new(world_data, canvas, textures);
    let mut input_manager = InputManager::new(sdl.event_pump().unwrap());

    'running: loop {
        {
            // Drain event pump to event queue. Separate scope as
            // to drop `event_queue` which will let us borrow
            // GameLoop again
            let mut event_queue = game_loop.event_queue();
            input_manager.reset_and_populate(&mut event_queue);
        }

        // Execute loop once
        match game_loop.execute() {
            GameLoopResult::Continue => {}
            GameLoopResult::Quit => break 'running,
        }

        // Display whatever we have
        renderer.draw(game_loop.drawables_storage());
    }
}
