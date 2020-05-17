use specs::Component;
use specs::VecStorage;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Enemy {
    pub speed_in_wc_per_tick: u16,
    pub ticks_in_movement: u8,
}

impl Enemy {
    pub fn new(speed_in_wc_per_tick: u16, ticks_in_movement: u8) -> Self {
        Self {
            speed_in_wc_per_tick,
            ticks_in_movement,
        }
    }
}
