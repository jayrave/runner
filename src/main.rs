extern crate sdl2;

use sdl2::render::WindowCanvas;

use crate::data::WorldData;
use sdl2::EventPump;
use specs::Dispatcher;
use specs::DispatcherBuilder;
use specs::World;
use specs::WorldExt;

mod components;
mod data;
mod entities;
mod graphics;
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
    run_game_loop(&mut event_pump, setup_ecs(world_data, canvas, textures));
}

fn setup_splash_screen(world_data: &WorldData, canvas: &mut WindowCanvas) {
    // Setup start-up color to prevent showing empty window until
    // the rendering loop starts
    canvas.set_draw_color(world_data.sky_color());
    canvas.clear();
    canvas.present();
}

fn setup_ecs<'a, 'b>(
    world_data: WorldData,
    canvas: WindowCanvas,
    textures: graphics::textures::Textures<'b>,
) -> (World, Dispatcher<'a, 'b>) {
    let mut world = World::new();

    // Insert resources
    world.insert(resources::EventQueue::new());
    world.insert(resources::GameTick::new());
    world.insert(resources::GameFinisher::new());

    // Register components
    world.register::<components::Animatable>();
    world.register::<components::Drawable>();
    world.register::<components::Enemy>();
    world.register::<components::Ground>();
    world.register::<components::Plant>();
    world.register::<components::Player>();
    world.register::<components::input::InputControlled>();

    // Create entities
    entities::Ground::create_all_tiles(&mut world, &world_data);
    entities::Player::create(&mut world, &world_data);

    // Orchestrate systems
    let animation_data = data::AnimationData::new();
    let dispatcher = DispatcherBuilder::new()
        .with(systems::GameTickUpdater, "game_tick_updater", &[])
        .with_barrier()
        .with(systems::EventSystem, "event_system", &[])
        .with_barrier() // To let event system to work before any other system
        .with(
            systems::GroundSystem::new(animation_data, world_data),
            "ground_system",
            &[],
        )
        .with(
            systems::PlantSystem::new(animation_data, world_data),
            "plant_system",
            &[],
        )
        .with(
            systems::PlayerSystem::new(animation_data, world_data),
            "player_system",
            &[],
        )
        .with(
            systems::EnemySystem::new(animation_data, world_data),
            "enemy_system",
            &[],
        )
        .with_thread_local(systems::RenderingSystem::new(world_data, canvas, textures))
        .build();

    (world, dispatcher)
}

fn run_game_loop(event_pump: &mut EventPump, inputs: (World, Dispatcher)) {
    let (mut world, mut dispatcher) = inputs;
    'running: loop {
        // Drain event pump to event queue
        world
            .fetch_mut::<resources::EventQueue>()
            .populate(event_pump);

        // Work the systems
        dispatcher.dispatch(&world);
        world.maintain();

        // Check & finish the game if required
        if world.fetch::<resources::GameFinisher>().should_finish() {
            break 'running;
        }
    }
}
