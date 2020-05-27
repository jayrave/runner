use crate::data::{GroundData, WorldData};

#[derive(Copy, Clone, Debug)]
pub struct CloudData {
    pub cloud_wave_ticks_count: u16,
    pub cloud_count_in_wave: u8,
}

impl CloudData {
    pub fn new(world_data: WorldData, ground_data: GroundData) -> Self {
        // Let's say that one enemy wave is one world length & we want
        // a particular number of enemies in a wave
        let wave_length_in_wc = world_data.bounds().width();
        let wave_ticks_count = (wave_length_in_wc / ground_data.speed_in_wc_per_tick as u32) as u16;
        Self {
            cloud_wave_ticks_count: wave_ticks_count,
            cloud_count_in_wave: 1,
        }
    }
}
