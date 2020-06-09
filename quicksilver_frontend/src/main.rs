use crate::images::Images;
use crate::input_manager::InputManager;
use crate::renderer::Renderer;
use quicksilver::geom::Vector;
use quicksilver::{Graphics, Input, Result, Settings, Window};
use runner_core::data::WorldData;
use runner_core::game_loop::{GameLoop, GameLoopResult};
use runner_frontend_utils::Fps;

mod color;
mod images;
mod input_manager;
mod renderer;

fn main() {
    // Initiate log before anything else
    init_logger();

    // Start game
    let world_data = WorldData::new();
    quicksilver::run(
        Settings {
            title: "Runner",
            size: Vector::new(
                world_data.bounds().width() as f32,
                world_data.bounds().height() as f32,
            ),
            ..Settings::default()
        },
        app,
    );
}

async fn app(window: Window, mut graphics: Graphics, input: Input) -> Result<()> {
    // Setup splash screen & give time to load the resources
    let world_data = WorldData::new();
    setup_splash_screen(world_data, &mut graphics, &window);

    // Time to load some resources
    let images = Images::load_from_files(&graphics).await;

    // Build everything required for the loop
    let mut game_loop = GameLoop::new(world_data);
    let mut renderer = Renderer::new(world_data, window, graphics, images);
    let mut input_manager = InputManager::new(input);
    let mut fps = Fps::new();

    // Note: unlike other front-ends, we are not using any `FrameLimiter`s
    // for quicksilver. Looks like awaiting on the event queue automatically
    // sets the FPS to 60. This can be seen from the logs from our Fps struct
    'running: loop {
        {
            // Drain event pump to event queue. Separate scope as
            // to drop `event_queue` which will let us borrow
            // GameLoop again
            let mut event_queue = game_loop.event_queue();
            input_manager.reset_and_populate(&mut event_queue).await;
        }

        // Execute loop once
        match game_loop.execute() {
            GameLoopResult::Continue => {}
            GameLoopResult::Quit => break 'running,
        }

        // Display whatever we have
        renderer.draw(game_loop.drawables_storage());
        fps.track();
    }

    Ok(())
}

fn setup_splash_screen(world_data: WorldData, graphics: &mut Graphics, window: &Window) {
    // Setup start-up color to prevent showing empty window until
    // the rendering loop starts
    graphics.clear(color::qs_color_from(world_data.sky_color()));
    graphics
        .present(&window)
        .expect("Not able to present the window");
}

#[cfg(feature = "desktop")]
fn init_logger() {
    use simplelog::{Config, LevelFilter, SimpleLogger};
    SimpleLogger::init(LevelFilter::Debug, Config::default()).expect("log couldn't be initiated")
}

#[cfg(feature = "web")]
fn init_logger() {
    use log::Level;
    stdweb_logger::init_with_level(Level::Debug);
}

#[cfg(not(feature = "desktop"))]
#[cfg(not(feature = "web"))]
fn init_logger() {
    panic!("You should either select the desktop or the web feature")
}
