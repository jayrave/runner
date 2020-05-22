use crate::resources::{GamePlay, GamePlayTick};
use specs::shred::ResourceId;
use specs::SystemData;
use specs::World;
use specs::WriteExpect;
use specs::{ReadExpect, System};

pub struct GamePlayTickUpdater;

#[derive(SystemData)]
pub struct TickUpdaterSystemData<'a> {
    game_play: ReadExpect<'a, GamePlay>,
    game_play_tick: WriteExpect<'a, GamePlayTick>,
}

impl<'a> System<'a> for GamePlayTickUpdater {
    type SystemData = TickUpdaterSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        if !data.game_play.is_over() {
            data.game_play_tick.update();
        }
    }
}
