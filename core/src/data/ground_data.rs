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
