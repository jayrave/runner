use crate::graphics::data;
use crate::{components, WorldData};
use sdl2::rect::Rect;
use specs::{Builder, World, WorldExt};

const PLAYER_TILE_WIDTH: u8 = 80;
const PLAYER_TILE_HEIGHT: u8 = 110;

pub struct Player;

impl Player {
    pub fn create(world: &mut World, world_data: &WorldData) {
        world
            .create_entity()
            .with(components::Player)
            .with(components::Drawable {
                tile_data: data::build_tile_data(data::Tile::Character {
                    tile: data::CharacterTile::Walk1,
                }),
                world_bounds: Rect::new(
                    world_data.world_left() + (i32::from(PLAYER_TILE_WIDTH) * 2),
                    world_data.world_surface_at() - i32::from(PLAYER_TILE_HEIGHT),
                    PLAYER_TILE_WIDTH.into(),
                    PLAYER_TILE_HEIGHT.into(),
                ),
            })
            .build();
    }
}
