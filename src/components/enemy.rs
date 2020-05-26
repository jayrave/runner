use crate::components::enemy::data::Position;
use specs::Component;
use specs::VecStorage;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Enemy {
    pub speed_in_wc_per_tick: u16,
    pub ticks_in_movement: u8,
    pub position: Position,
}

impl Enemy {
    pub fn new(speed_in_wc_per_tick: u16, ticks_in_movement: u8, position: Position) -> Self {
        Self {
            speed_in_wc_per_tick,
            ticks_in_movement,
            position,
        }
    }
}

pub mod data {
    #[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
    pub enum Position {
        Low,
        Mid,
        High,
    }
}
