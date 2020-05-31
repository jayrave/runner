extern crate sdl2;

use crate::input_manager::InputManager;
use crate::renderer::Renderer;
use crate::textures::Textures;
use runner_core::data::WorldData;
use runner_core::ecs::Ecs;
use runner_core::frame_limiter::FrameLimiter;
use runner_core::input::{Event, Keycode};
use runner_core::resources::{EventQueue, GamePlay};
use sdl2::render::WindowCanvas;
use specs::WorldExt;

mod color;
mod input_manager;
mod renderer;
mod textures;

pub fn main() {
    let world_data = WorldData::new();
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(
            "Runner",
            world_data.bounds().width(),
            world_data.bounds().height(),
        )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();
    let textures = Textures::load_from_files(&texture_creator);

    setup_splash_screen(&world_data, &mut canvas);
    run_game_loop(
        world_data,
        InputManager::new(sdl_context.event_pump().unwrap()),
        Renderer::new(world_data, canvas, textures),
        FrameLimiter::new(60),
    );
}

fn setup_splash_screen(world_data: &WorldData, canvas: &mut WindowCanvas) {
    // Setup start-up color to prevent showing empty window until
    // the rendering loop starts
    canvas.set_draw_color(color::sdl_color_from(world_data.sky_color()));
    canvas.clear();
    canvas.present();
}

fn run_game_loop(
    world_data: WorldData,
    mut input_manager: InputManager,
    mut renderer: Renderer,
    mut frame_limiter: FrameLimiter,
) {
    let mut ecs = Ecs::setup(world_data);
    'running: loop {
        // Drain event pump to event queue
        input_manager.reset_and_populate(&mut ecs.world.fetch_mut::<EventQueue>());

        // Check & finish the game or start a new game if required
        let result = handle_input(&ecs.world.fetch(), &mut ecs.world.fetch_mut());
        match result {
            HandleInputResult::Quit => break 'running,
            HandleInputResult::Continue => {}
            HandleInputResult::StartNewGame => ecs = Ecs::setup(world_data),
        }

        // Work the systems
        if ecs.world.fetch::<GamePlay>().is_allowed() {
            ecs.dispatch()
        }

        // Display whatever we have
        renderer.draw(ecs.world.read_storage());

        // We don't want to drink up too much power
        frame_limiter.limit_as_required();
    }
}

enum HandleInputResult {
    Continue,
    StartNewGame,
    Quit,
}

fn handle_input(event_queue: &EventQueue, game_play: &mut GamePlay) -> HandleInputResult {
    for event in event_queue.iter() {
        match event {
            Event::Quit => return HandleInputResult::Quit,
            Event::KeyDown(keycode) => match keycode {
                Keycode::Escape => return HandleInputResult::Quit,
                _ => {
                    if !game_play.is_started() {
                        game_play.mark_started();
                        return HandleInputResult::Continue;
                    } else if game_play.is_over() {
                        return HandleInputResult::StartNewGame;
                    }
                }
            },
            _ => {}
        }
    }

    HandleInputResult::Continue
}
