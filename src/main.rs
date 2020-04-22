extern crate sdl2;

use sdl2::render::WindowCanvas;

use sdl2::EventPump;
use specs::Dispatcher;
use specs::DispatcherBuilder;
use specs::World;
use specs::WorldExt;

mod constants;
mod graphics;
mod resources;
mod systems;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("runner", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    setup_splash_screen(&mut canvas);
    run_game_loop(&mut event_pump, setup_ecs(canvas));
}

fn setup_splash_screen(canvas: &mut WindowCanvas) {
    // Setup start-up color to prevent showing empty window until
    // the rendering loop starts
    canvas.set_draw_color(constants::SKY_COLOR);
    canvas.clear();
    canvas.present();
}

fn setup_ecs<'a, 'b>(canvas: WindowCanvas) -> (World, Dispatcher<'a, 'b>) {
    let mut world = World::new();

    // Insert resources
    world.insert(resources::EventQueue::new());
    world.insert(resources::GameFinisher::new());

    // Orchestrate systems
    let dispatcher = DispatcherBuilder::new()
        .with(systems::EventSystem, "event_system", &[])
        .with_barrier() // To let event system to work before any other system
        .with_thread_local(systems::RenderingSystem::new(canvas))
        .with_thread_local(systems::PowerOptimizerSystem::new())
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
