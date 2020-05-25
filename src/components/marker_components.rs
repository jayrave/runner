use specs::Component;
use specs::NullStorage;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Cloud;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Ground;
