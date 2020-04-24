use crate::components;
use crate::constants;
use crate::graphics::data;
use sdl2::rect::Rect;
use specs::{Builder, World, WorldExt};

const PLAYER_TILE_WIDTH: u8 = 80;
const PLAYER_TILE_HEIGHT: u8 = 110;
pub struct Player;
impl Player {
    pub fn create(world: &mut World) {
        world
            .create_entity()
            .with(components::Player)
            .with(components::Drawable {
                tile_data: data::build_tile_data(data::Tile::Character { tile: data::CharacterTile::Walk1 }),
                world_bounds: Rect::new(
                    -300,
                    150i32 - PLAYER_TILE_HEIGHT as i32,
                    PLAYER_TILE_WIDTH.into(),
                    PLAYER_TILE_HEIGHT.into(),
                ),
            })
            .build();
    }
}
