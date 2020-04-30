use specs::Component;
use specs::NullStorage;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Enemy;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Ground;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Plant;
