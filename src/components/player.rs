use specs::Component;
use specs::VecStorage;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Player {
    pub current_action: data::Action,
}

impl Player {
    pub fn new() -> Player {
        Player {
            current_action: data::Action::Run,
        }
    }
}

pub mod data {
    #[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
    pub enum Action {
        FastRun,
        Jump,
        Run,
        Slide,
        SlowRun,
    }
}
