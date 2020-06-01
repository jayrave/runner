use crate::images::Images;
use crate::input_manager::InputManager;
use crate::renderer::Renderer;
use quicksilver::geom::Vector;
use quicksilver::graphics::{Color, Image};
use quicksilver::{Graphics, Input, Result, Settings, Window};
use runner_core::components::Drawable;
use runner_core::data::WorldData;
use runner_core::game_loop;
use runner_core::game_loop::{GameLoop, GameLoopResult};
use runner_core::resources::EventQueue;
use specs::ReadStorage;

mod color;
mod images;
mod input_manager;
mod renderer;

fn main() {
    let world_data = WorldData::new();
    quicksilver::run(
        Settings {
            title: "Runner",
            size: Vector::new(world_data.bounds().width(), world_data.bounds().height()),
            ..Settings::default()
        },
        app,
    );
}

async fn app(window: Window, mut graphics: Graphics, mut input: Input) -> Result<()> {
    let world_data = WorldData::new();
    let images = Images::load_from_files(&graphics).await;

    let mut game_loop = GameLoop::new(world_data);
    let mut renderer = Renderer::new(world_data, window, graphics, images);
    let mut input_manager = InputManager::new(input);

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
    }

    Ok(())
}
