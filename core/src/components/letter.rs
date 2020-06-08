use specs::Component;
use specs::NullStorage;

#[derive(Component, Clone, Copy, Default)]
#[storage(NullStorage)]
pub struct Letter;
