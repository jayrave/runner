use crate::graphics;
use sdl2::rect::Rect;
use specs::Component;
use specs::NullStorage;
use specs::VecStorage;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Drawable {
    pub tile_data: graphics::data::TileData,
    pub world_bounds: Rect,
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Ground;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Player;

#[derive(Component, Default)]
#[storage(VecStorage)]
pub struct UserControlled {
    pub input: Option<data::Input>,
}

pub mod data {
    #[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
    pub enum Input {
        Up,
        Down,
    }
}
