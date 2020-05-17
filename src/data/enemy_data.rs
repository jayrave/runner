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
    pub fn new() -> Self {
        Self {
            min_ticks_between_enemies: 180,
            randomness_factor: 6,
            bat_animation: Animation {
                speed_in_wc_per_tick: 6,
                ticks_in_movement: 8,
            },
            bee_animation: Animation {
                speed_in_wc_per_tick: 4,
                ticks_in_movement: 2,
            },
            bug_animation: Animation {
                speed_in_wc_per_tick: 2,
                ticks_in_movement: 2,
            },
            mouse_animation: Animation {
                speed_in_wc_per_tick: 3,
                ticks_in_movement: 8,
            },
            spider_animation: Animation {
                speed_in_wc_per_tick: 4,
                ticks_in_movement: 10,
            },
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Animation {
    pub speed_in_wc_per_tick: u8,
    pub ticks_in_movement: u8,
}
