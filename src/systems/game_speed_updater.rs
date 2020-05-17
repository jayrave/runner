use crate::data::enemy_data::EnemyData;
use crate::data::{GroundData, PlayerData};
use crate::resources::GameTick;
use specs::shred::ResourceId;
use specs::SystemData;
use specs::World;
use specs::WriteExpect;
use specs::{ReadExpect, System};

// Speed up once every 10 seconds
const SPEED_UP_TICK_INTERVAL: u16 =
    (10 * 1000) / (crate::resources::game_tick::MILLISECONDS_IN_A_TICK as u16);

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
    game_tick: ReadExpect<'a, GameTick>,
    ground_data: WriteExpect<'a, GroundData>,
    enemy_data: WriteExpect<'a, EnemyData>,
    player_data: WriteExpect<'a, PlayerData>,
}

impl<'a> System<'a> for GameSpeedUpdater {
    type SystemData = SpeedSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        let tick_animated = data.game_tick.ticks_animated();
        if self.last_speed_up_at_tick + u64::from(SPEED_UP_TICK_INTERVAL) <= tick_animated
        {
            let multiplier = self.last_multiplier * 2.0;
            let ground_data = GroundData::new(multiplier);
            let enemy_data = EnemyData::new(&ground_data);
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
