use crate::resources::GameTick;
use specs::System;
use specs::WriteExpect;

pub struct GameTickUpdater;

impl<'a> System<'a> for GameTickUpdater {
    type SystemData = WriteExpect<'a, GameTick>;

    fn run(&mut self, mut game_tick: Self::SystemData) {
        game_tick.update();
    }
}
