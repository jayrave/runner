use crate::data::WorldData;
use crate::ecs::Ecs;
use crate::frame_limiter::FrameLimiter;
use crate::input::{Event, Keycode};
use crate::resources::{EventQueue, GamePlay};
use specs::{WorldExt, ReadStorage};
use crate::components::Drawable;

pub trait LoopActions {
    fn pump_events(&mut self, event_queue: &mut EventQueue);
    fn render(&mut self, drawables_storage: ReadStorage<Drawable>);
}

pub fn run_game_loop(
    world_data: WorldData,
    loop_actions: &mut dyn LoopActions,
) {
    let mut frame_limiter = FrameLimiter::new(60);
    let mut ecs = Ecs::setup(world_data);
    'running: loop {
        // Drain event pump to event queue
        loop_actions.pump_events(&mut ecs.world.fetch_mut::<EventQueue>());

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
        loop_actions.render(ecs.world.read_storage());

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
