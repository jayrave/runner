use specs::Component;
use specs::VecStorage;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Player {
    pub current_action: data::Action,
    pub is_hit: bool,
}

impl Player {
    pub fn new() -> Player {
        Player {
            current_action: data::Action::Run,
            is_hit: false,
        }
    }
}

pub mod data {
    #[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
    pub enum Action {
        Jump,
        Run,
        Slide,
    }
}
