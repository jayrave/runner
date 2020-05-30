use specs::Component;
use specs::VecStorage;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Animatable {
    pub current_step_started_at_tick: u64,
}
