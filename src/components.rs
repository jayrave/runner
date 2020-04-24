use crate::graphics::data;
use sdl2::rect::Rect;
use specs::Component;
use specs::VecStorage;
use specs::NullStorage;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Drawable {
    pub tile_data: data::TileData,
    pub world_bounds: Rect,
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Ground;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Player;