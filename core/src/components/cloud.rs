use specs::Component;
use specs::VecStorage;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Cloud {
    pub speed_in_wc_per_tick: u16,
}

impl Cloud {
    pub fn new(speed_in_wc_per_tick: u16) -> Self {
        Self {
            speed_in_wc_per_tick,
        }
    }
}
