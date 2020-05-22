use crate::data::GroundData;

#[derive(Copy, Clone, Debug)]
pub struct EnemyData {
    pub min_ticks_between_enemies: u64,
    pub randomness_factor: u8,
    pub bat_animation: Animation,
    pub bee_animation: Animation,
    pub bug_animation: Animation,
    pub mouse_animation: Animation,
    pub spider_animation: Animation,
}

impl EnemyData {
    pub fn new(ground_data: GroundData) -> Self {
        Self {
            min_ticks_between_enemies: 180,
            randomness_factor: 6,

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
