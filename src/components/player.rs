use specs::Component;
use specs::VecStorage;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Player {
    pub most_recent_max_jump_end_at: u64,
    pub most_recent_max_slide_end_at: u64,
    pub current_action: data::Action,
    pub is_hit: bool,
}

impl Player {
    pub fn new() -> Player {
        Player {
            most_recent_max_jump_end_at: 0,
            most_recent_max_slide_end_at: 0,
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
