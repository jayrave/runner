#[derive(Copy, Clone, Debug)]
pub struct AnimationData {
    ground_speed_in_wc_per_tick: u8,
}

impl Default for AnimationData {
    fn default() -> Self {
        AnimationData {
            ground_speed_in_wc_per_tick: 2,
        }
    }
}

impl AnimationData {
    pub fn new() -> AnimationData {
        AnimationData::default()
    }

    pub fn ground_speed_in_wc_per_tick(&self) -> u8 {
        self.ground_speed_in_wc_per_tick
    }
}
