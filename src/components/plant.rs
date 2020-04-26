use specs::Component;
use specs::NullStorage;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Plant;
