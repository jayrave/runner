use crate::resources::GamePlay;
use specs::shred::ResourceId;
use specs::System;
use specs::SystemData;
use specs::World;
use specs::WriteExpect;

pub struct GamePlayTickUpdater;

#[derive(SystemData)]
pub struct TickUpdaterSystemData<'a> {
    game_play: WriteExpect<'a, GamePlay>,
}

impl<'a> System<'a> for GamePlayTickUpdater {
    type SystemData = TickUpdaterSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        if data.game_play.should_allow() {
            data.game_play.update();
        }
    }
}
