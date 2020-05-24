use crate::data::{GroundData, WorldData};

#[derive(Copy, Clone, Debug)]
pub struct EnemyData {
    pub enemy_wave_ticks_count: u16,
    pub enemy_count_in_wave: u8,
    pub bat_animation: Animation,
    pub bee_animation: Animation,
    pub bug_animation: Animation,
    pub mouse_animation: Animation,
    pub spider_animation: Animation,
}

impl EnemyData {
    /// Shouldn't use [speed_multiplier] to change the animation speed or
    /// the movement speed of the enemies since it is just the player who
    /// is speeding up & not the enemies
    pub fn new(world_data: WorldData, ground_data: GroundData) -> Self {
        // Let's say that one enemy wave is one world length & we want
        // a particular number of enemies in a wave
        let wave_length_in_wc = world_data.bounds().width();
        let wave_ticks_count = (wave_length_in_wc / ground_data.speed_in_wc_per_tick as u32) as u16;
        Self {
            enemy_wave_ticks_count: wave_ticks_count,
            enemy_count_in_wave: 4,

            // Bats fly fast but since they have sizable wings, needn't
            // animate that fast
            bat_animation: Animation {
                speed_in_wc_per_tick: (ground_data.speed_in_wc_per_tick + 4) as u16,
                ticks_in_movement: 20,
            },

            // Bees are slow than bats but owing to their wing size is
            // going to have a lot more flaps
            bee_animation: Animation {
                speed_in_wc_per_tick: (ground_data.speed_in_wc_per_tick + 2) as u16,
                ticks_in_movement: 10,
            },

            // Bug needn't move. It just stays put on the ground
            bug_animation: Animation {
                speed_in_wc_per_tick: (ground_data.speed_in_wc_per_tick) as u16,
                ticks_in_movement: std::u8::MAX,
            },

            // Mouse should move
            mouse_animation: Animation {
                speed_in_wc_per_tick: (ground_data.speed_in_wc_per_tick + 1) as u16,
                ticks_in_movement: 10,
            },

            // Spider should move. Our spider is pretty big. So, it can walk
            // a bit slower than the mouse
            spider_animation: Animation {
                speed_in_wc_per_tick: (ground_data.speed_in_wc_per_tick + 2) as u16,
                ticks_in_movement: 14,
            },
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Animation {
    pub speed_in_wc_per_tick: u16,
    pub ticks_in_movement: u8,
}
