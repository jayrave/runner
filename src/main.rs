extern crate sdl2;

use sdl2::render::WindowCanvas;

use crate::data::WorldData;
use crate::game_world::GameWorld;
use crate::resources::{EventQueue, GamePlay};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

use crate::frame_limiter::FrameLimiter;
use crate::graphics::textures::Textures;
use crate::renderer::Renderer;
use specs::WorldExt;

mod components;
mod data;
mod entities;
mod frame_limiter;
mod game_world;
mod graphics;
mod jump_physics;
mod rect;
mod renderer;
mod resources;
mod systems;

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
    let mut event_pump = sdl_context.event_pump().unwrap();

    let texture_creator = canvas.texture_creator();
    let textures = Textures::load_from_files(&texture_creator);

    setup_splash_screen(&world_data, &mut canvas);
    run_game_loop(
        world_data,
        &mut event_pump,
        Renderer::new(world_data, canvas, textures),
        FrameLimiter::new(60),
    );
}

fn setup_splash_screen(world_data: &WorldData, canvas: &mut WindowCanvas) {
    // Setup start-up color to prevent showing empty window until
    // the rendering loop starts
    canvas.set_draw_color(world_data.sky_color());
    canvas.clear();
    canvas.present();
}

fn run_game_loop(
    world_data: WorldData,
    event_pump: &mut EventPump,
    mut renderer: Renderer,
    mut frame_limiter: FrameLimiter,
) {
    let mut game_world = GameWorld::setup(world_data);
    'running: loop {
        // Drain event pump to event queue
        game_world
            .world
            .fetch_mut::<EventQueue>()
            .reset_and_populate(event_pump);

        // Check & finish the game or start a new game if required
        let result = handle_input(&game_world.world.fetch(), &mut game_world.world.fetch_mut());
        match result {
            HandleInputResult::Quit => break 'running,
            HandleInputResult::Continue => {}
            HandleInputResult::StartNewGame => game_world = GameWorld::setup(world_data),
        }

        // Work the systems
        if game_world.world.fetch::<GamePlay>().is_allowed() {
            game_world.dispatch()
        }

        // Display whatever we have
        renderer.draw(game_world.world.read_storage());

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
            Event::Quit { .. } => return HandleInputResult::Quit,
            Event::KeyDown {
                keycode: Some(keycode),
                ..
            } => match keycode {
                Keycode::Escape => return HandleInputResult::Quit,
                Keycode::Space => {
                    if !game_play.is_started() {
                        game_play.mark_started();
                        return HandleInputResult::Continue;
                    } else if game_play.is_over() {
                        return HandleInputResult::StartNewGame;
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    HandleInputResult::Continue
}
