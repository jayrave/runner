use crate::data::WorldData;

#[derive(Copy, Clone, Debug)]
pub struct CloudData {
    pub cloud_wave_ticks_count: u16,
    pub cloud_count_in_wave: u8,
    pub speed_in_wc_per_tick: u16,
}

impl CloudData {
    pub fn new(world_data: WorldData, ground_data: GroundData) -> Self {
        // Let's say that one enemy wave is one world length & we want
        // a particular number of enemies in a wave
        let wave_length_in_wc = world_data.bounds().width();
        let wave_ticks_count = (wave_length_in_wc / ground_data.speed_in_wc_per_tick as u32) as u16;
        Self {
            cloud_wave_ticks_count: wave_ticks_count,
            cloud_count_in_wave: 4,
            speed_in_wc_per_tick: (i32::from(ground_data.speed_in_wc_per_tick) - 6).max(1) as u16,
        }
    }
}

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
