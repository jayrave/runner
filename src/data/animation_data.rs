#[derive(Copy, Clone, Debug)]
pub struct GroundData {
    pub speed_in_wc_per_tick: u8,
}

impl GroundData {
    pub fn new() -> Self {
        Self {
            speed_in_wc_per_tick: 2,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct PlayerData {
    pub ticks_in_max_jump: u8,
    pub ticks_in_slide: u8,
    pub ticks_in_run_step: u8,
    pub ticks_multiplier_for_faster_running: f32,
    pub ticks_multiplier_for_slower_running: f32,
    pub max_jump_height_in_wc: u8,
    pub speed_in_wc_per_tick_fast_run: u8,
    pub speed_in_wc_per_tick_slow_run: u8,
}

impl PlayerData {
    pub fn new(ground_data: &GroundData) -> Self {
        Self {
            ticks_in_max_jump: 60,
            ticks_in_slide: 30,
            ticks_in_run_step: 16,
            ticks_multiplier_for_faster_running: 0.5,
            ticks_multiplier_for_slower_running: 2.0,
            max_jump_height_in_wc: 120,
            speed_in_wc_per_tick_fast_run: ground_data.speed_in_wc_per_tick * 2,
            speed_in_wc_per_tick_slow_run: ground_data.speed_in_wc_per_tick * 2,
        }
    }
}
