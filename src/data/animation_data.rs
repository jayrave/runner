#[derive(Copy, Clone, Debug)]
pub struct GroundData {
    pub speed_in_wc_per_tick: u8,
}

impl GroundData {
    pub fn new() -> Self {
        Self {
            speed_in_wc_per_tick: 4,
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
    pub jump_height_in_wc: u8,
    pub extra_input_speed_in_wc_per_tick: u8,
}

impl PlayerData {
    pub fn new() -> Self {
        Self {
            ticks_in_max_jump: 60,
            ticks_in_slide: 40,
            ticks_in_run_step: 12,
            ticks_multiplier_for_faster_running: 0.5,
            ticks_multiplier_for_slower_running: 2.0,
            jump_height_in_wc: 120,
            extra_input_speed_in_wc_per_tick: 4,
        }
    }
}
