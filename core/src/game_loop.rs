use crate::components::Drawable;
use crate::data::WorldData;
use crate::ecs::Ecs;
use crate::input::{Event, Keycode};
use crate::resources::{EventQueue, GamePlay};
use specs::shred::FetchMut;
use specs::{ReadStorage, WorldExt};

enum HandleInputResult {
    NoAction,
    StartGamePlay,
    RestartGame,
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
        let mut ecs = Ecs::setup(world_data);
        ecs.show_instructions();

        GameLoop { world_data, ecs }
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
            GameLoop::handle_input(&self.ecs.world.fetch(), &self.ecs.world.fetch_mut());

        match handle_input_result {
            HandleInputResult::NoAction => {}
            HandleInputResult::Quit => game_loop_result = GameLoopResult::Quit,
            HandleInputResult::StartGamePlay => {
                &mut self.ecs.world.fetch_mut::<GamePlay>().mark_started();
                self.ecs.start_game_play()
            }
            HandleInputResult::RestartGame => {
                self.ecs = Ecs::setup(self.world_data);

                // Needn't show instructions again & can directly start playing
                &mut self.ecs.world.fetch_mut::<GamePlay>().mark_started();
                self.ecs.start_game_play()
            }
        }

        // Work the systems
        let is_game_play_allowed = self.ecs.world.fetch::<GamePlay>().is_allowed();
        if is_game_play_allowed {
            self.ecs.dispatch()
        }

        // If game came to an end, reflect that correctly
        if is_game_play_allowed && self.ecs.world.fetch::<GamePlay>().is_over() {
            self.ecs.show_game_end()
        }

        game_loop_result
    }

    fn handle_input(event_queue: &EventQueue, game_play: &GamePlay) -> HandleInputResult {
        for event in event_queue.iter() {
            match event {
                Event::Quit => return HandleInputResult::Quit,
                Event::KeyDown(keycode) => match keycode {
                    Keycode::Escape => return HandleInputResult::Quit,
                    Keycode::Space => {
                        if !game_play.is_started() {
                            return HandleInputResult::StartGamePlay;
                        } else if game_play.is_over() {
                            return HandleInputResult::RestartGame;
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        HandleInputResult::NoAction
    }
}
