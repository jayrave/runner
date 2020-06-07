use crate::resources::{Score, GamePlay};
use specs::shred::ResourceId;
use specs::{System, ReadExpect};
use specs::SystemData;
use specs::World;
use specs::WriteExpect;

pub struct ScoreUpdater;

#[derive(SystemData)]
pub struct ScoreUpdaterSystemData<'a> {
    game_play: ReadExpect<'a, GamePlay>,
    score: WriteExpect<'a, Score>,
}

impl<'a> System<'a> for ScoreUpdater {
    type SystemData = ScoreUpdaterSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        data.score.update(data.game_play.ticks_animated())
    }
}
