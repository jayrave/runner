use crate::graphics;
use crate::graphics::data::TileData;
use sdl2::rect::Rect;
use specs::Component;
use specs::VecStorage;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Drawable {
    pub tile_data: TileData,
    pub world_bounds: Rect,
}
