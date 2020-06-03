use crate::components::Drawable;
use crate::data::WorldData;
use crate::ecs::Ecs;
use crate::input::{Event, Keycode};
use crate::resources::{EventQueue, GamePlay};
use specs::shred::FetchMut;
use specs::{ReadStorage, WorldExt};

enum HandleInputResult {
    Continue,
    StartNewGame,
    Quit,
}

pub enum GameLoopResult {
    Continue,
    Quit,
}

pub struct GameLoop<'a, 'b> {
    world_data: WorldData,
    ecs: Ecs<'a, 'b>,
}

impl<'a, 'b> GameLoop<'a, 'b> {
    pub fn new(world_data: WorldData) -> GameLoop<'a, 'b> {
        GameLoop {
            world_data,
            ecs: Ecs::setup(world_data),
        }
    }

    pub fn event_queue(&mut self) -> FetchMut<EventQueue> {
        self.ecs.world.fetch_mut::<EventQueue>()
    }

    pub fn drawables_storage(&self) -> ReadStorage<Drawable> {
        self.ecs.world.read_storage()
    }

    pub fn execute(&mut self) -> GameLoopResult {
        // Check & finish the game or start a new game if required
        let mut game_loop_result = GameLoopResult::Continue;
        let handle_input_result =
            GameLoop::handle_input(&self.ecs.world.fetch(), &mut self.ecs.world.fetch_mut());

        match handle_input_result {
            HandleInputResult::Quit => game_loop_result = GameLoopResult::Quit,
            HandleInputResult::Continue => {}
            HandleInputResult::StartNewGame => self.ecs = Ecs::setup(self.world_data),
        }

        // Work the systems
        if self.ecs.world.fetch::<GamePlay>().is_allowed() {
            self.ecs.dispatch()
        }

        game_loop_result
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
}
