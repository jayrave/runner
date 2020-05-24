use crate::data::enemy_data::EnemyData;
use crate::data::{GroundData, PlayerData};
use crate::resources::GamePlay;
use specs::shred::ResourceId;
use specs::SystemData;
use specs::World;
use specs::WriteExpect;
use specs::{ReadExpect, System};

// Speed up once every 10 seconds
const SPEED_UP_MULTIPLIER: f32 = 1.1;
const SPEED_UP_INTERVAL_IN_SECONDS: u8 = 10;
const SPEED_UP_TICK_INTERVAL: u16 = (SPEED_UP_INTERVAL_IN_SECONDS as u16 * 1000)
    / (crate::resources::game_play::MILLISECONDS_IN_A_TICK as u16);

pub struct GameSpeedUpdater {
    last_speed_up_at_tick: u64,
    last_multiplier: f32,
}

impl GameSpeedUpdater {
    pub fn new() -> Self {
        Self {
            last_speed_up_at_tick: 0,
            last_multiplier: 1.0,
        }
    }
}

#[derive(SystemData)]
pub struct SpeedSystemData<'a> {
    game_play: ReadExpect<'a, GamePlay>,
    ground_data: WriteExpect<'a, GroundData>,
    enemy_data: WriteExpect<'a, EnemyData>,
    player_data: WriteExpect<'a, PlayerData>,
}

impl<'a> System<'a> for GameSpeedUpdater {
    type SystemData = SpeedSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        let tick_animated = data.game_play.ticks_animated();
        if self.last_speed_up_at_tick + u64::from(SPEED_UP_TICK_INTERVAL) <= tick_animated {
            let multiplier = self.last_multiplier * SPEED_UP_MULTIPLIER;
            let ground_data = GroundData::new(multiplier);
            let enemy_data = EnemyData::new(ground_data, multiplier);
            let player_data = PlayerData::new();

            *data.ground_data = ground_data;
            *data.enemy_data = enemy_data;
            *data.player_data = player_data;

            // Update local data for next run
            self.last_multiplier = multiplier;
            self.last_speed_up_at_tick = tick_animated;
        }
    }
}
