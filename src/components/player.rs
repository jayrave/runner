use specs::Component;
use specs::VecStorage;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Player {
    pub current_step_started_at_tick: u64,
}
