use crate::graphics;
use sdl2::rect::Rect;
use specs::Component;
use specs::VecStorage;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Drawable {
    pub tile_data: graphics::data::TileData,
    pub world_bounds: Rect,
}
