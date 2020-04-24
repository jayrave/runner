extern crate sdl2;

use sdl2::render::WindowCanvas;

use sdl2::EventPump;
use specs::Dispatcher;
use specs::DispatcherBuilder;
use specs::World;
use specs::WorldExt;

mod components;
mod constants;
mod entities;
mod graphics;
mod resources;
mod systems;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(
            constants::GAME_NAME,
            constants::WORLD_WIDTH,
            constants::WORLD_HEIGHT,
        )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let texture_creator = canvas.texture_creator();
    let textures = graphics::textures::Textures::load_from_files(&texture_creator);

    setup_splash_screen(&mut canvas);
    run_game_loop(&mut event_pump, setup_ecs(canvas, textures));
}

fn setup_splash_screen(canvas: &mut WindowCanvas) {
    // Setup start-up color to prevent showing empty window until
    // the rendering loop starts
    canvas.set_draw_color(constants::SKY_COLOR);
    canvas.clear();
    canvas.present();
}

fn setup_ecs<'a, 'b>(
    canvas: WindowCanvas,
    textures: graphics::textures::Textures<'b>,
) -> (World, Dispatcher<'a, 'b>) {
    let mut world = World::new();

    // Insert resources
    world.insert(resources::EventQueue::new());
    world.insert(resources::FrameStepper::new());
    world.insert(resources::GameFinisher::new());

    // Register components
    world.register::<components::Drawable>();
    world.register::<components::Ground>();
    world.register::<components::Player>();

    // Create entities
    entities::Ground::create_all_tiles(&mut world);
    entities::Player::create(&mut world);

    // Orchestrate systems
    let dispatcher = DispatcherBuilder::new()
        .with(systems::FrameStepperUpdater, "frame_stepper_updater", &[])
        .with_barrier()
        .with(systems::EventSystem, "event_system", &[])
        .with_barrier() // To let event system to work before any other system
        .with(systems::physics::GroundSystem, "ground_system", &[])
        .with(systems::physics::PlayerSystem, "player_system", &[])
        .with_thread_local(systems::RenderingSystem::new(canvas, textures))
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
