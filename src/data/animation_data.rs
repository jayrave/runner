#[derive(Copy, Clone, Debug)]
pub struct GroundData {
    pub speed_in_wc_per_tick: u16,
}

impl GroundData {
    pub fn new(speed_multiplier: f32) -> Self {
        Self {
            speed_in_wc_per_tick: (2.0 * speed_multiplier) as u16,
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
    pub speed_in_wc_per_tick_fast_run: u16,
    pub speed_in_wc_per_tick_slow_run: u16,
}

impl PlayerData {
    pub fn new() -> Self {
        Self {
            ticks_in_max_jump: 60,
            ticks_in_slide: 30,
            ticks_in_run_step: 16,
            ticks_multiplier_for_faster_running: 0.5,
            ticks_multiplier_for_slower_running: 2.0,
            max_jump_height_in_wc: 120,
            speed_in_wc_per_tick_fast_run: 4,
            speed_in_wc_per_tick_slow_run: 4,
        }
    }
}
