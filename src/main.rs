extern crate sdl2;

use sdl2::render::WindowCanvas;

use crate::data::WorldData;
use crate::resources::{EventQueue, GamePlay};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;
use specs::Dispatcher;
use specs::DispatcherBuilder;
use specs::World;
use specs::WorldExt;

mod components;
mod data;
mod entities;
mod frame_limiter;
mod graphics;
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
    let textures = graphics::textures::Textures::load_from_files(&texture_creator);

    setup_splash_screen(&world_data, &mut canvas);
    run_game_loop(
        &mut event_pump,
        setup_ecs(world_data),
        renderer::Renderer::new(world_data, canvas, textures),
        frame_limiter::FrameLimiter::new(60),
    );
}

fn setup_splash_screen(world_data: &WorldData, canvas: &mut WindowCanvas) {
    // Setup start-up color to prevent showing empty window until
    // the rendering loop starts
    canvas.set_draw_color(world_data.sky_color());
    canvas.clear();
    canvas.present();
}

fn setup_ecs<'a, 'b>(world_data: WorldData) -> (World, Dispatcher<'a, 'b>) {
    let mut world = World::new();

    // Insert resources
    let ground_data = data::GroundData::new(1.0);
    world.insert(data::enemy_data::EnemyData::new(&ground_data));
    world.insert(data::PlayerData::new());
    world.insert(ground_data);
    world.insert(resources::EventQueue::new());
    world.insert(resources::GamePlay::new());
    world.insert(resources::GamePlayTick::new());

    // Register components
    world.register::<components::Animatable>();
    world.register::<components::Drawable>();
    world.register::<components::Enemy>();
    world.register::<components::Ground>();
    world.register::<components::player::Player>();
    world.register::<components::input::InputControlled>();

    // Create entities
    entities::Ground::create_all_tiles(&mut world, &world_data);
    entities::Player::create(&mut world, &world_data);

    // Orchestrate systems
    let game_play_tick_updater = "game_play_tick_updater";
    let dispatcher = DispatcherBuilder::new()
        .with(systems::GamePlayTickUpdater, game_play_tick_updater, &[])
        .with(
            systems::EventSystem,
            "event_system",
            &[game_play_tick_updater],
        )
        .with(systems::GameSpeedUpdater::new(), "game_speed_updater", &[])
        .with_barrier() // To let event system & game updaters to work before any other systems
        .with(systems::GroundSystem::new(world_data), "ground_system", &[])
        .with(systems::PlayerSystem::new(world_data), "player_system", &[])
        .with(systems::EnemySystem::new(world_data), "enemy_system", &[])
        .with_barrier()
        .with(systems::CollisionSystem, "collision_system", &[])
        .build();

    (world, dispatcher)
}

fn run_game_loop(
    event_pump: &mut EventPump,
    inputs: (World, Dispatcher),
    mut renderer: renderer::Renderer,
    mut frame_limiter: frame_limiter::FrameLimiter,
) {
    let (mut world, mut dispatcher) = inputs;
    'running: loop {
        // Drain event pump to event queue
        world
            .fetch_mut::<resources::EventQueue>()
            .reset_and_populate(event_pump);

        // Check & finish the game if required
        if should_quit_on_handling_input(&world.fetch(), &mut world.fetch_mut()) {
            break 'running;
        }

        // Work the systems
        if world.fetch::<resources::GamePlay>().should_allow() {
            dispatcher.dispatch(&world);
            world.maintain();
        }

        // Display whatever we have
        renderer.draw(world.read_storage());

        // We don't want to drink up too much power
        frame_limiter.limit_as_required();
    }
}

fn should_quit_on_handling_input(event_queue: &EventQueue, game_play: &mut GamePlay) -> bool {
    let mut should_finish_game = false;
    for event in event_queue.iter() {
        match event {
            Event::Quit { .. } => should_finish_game = true,
            Event::KeyDown {
                keycode: Some(keycode),
                ..
            } => match keycode {
                Keycode::Escape => should_finish_game = true,
                Keycode::Space => game_play.mark_started(),
                _ => {}
            },
            _ => {}
        }
    }

    should_finish_game
}
