use crate::components::player::data::Action;
use crate::jump_physics::JumpPhysics;
use specs::Component;
use specs::VecStorage;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Player {
    pub jump_physics: Option<JumpPhysics>,
    pub most_recent_max_jump_end_at: u64,
    pub most_recent_max_slide_end_at: u64,
    pub current_action: Action,
    pub is_hit: bool,
}

impl Player {
    pub fn new() -> Player {
        Player {
            jump_physics: None,
            most_recent_max_jump_end_at: 0,
            most_recent_max_slide_end_at: 0,
            current_action: Action::Run,
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
