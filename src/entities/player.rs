use crate::graphics::data;
use crate::{components, WorldData};
use sdl2::rect::Rect;
use specs::{Builder, World, WorldExt};

const PLAYER_TILE_WORLD_WIDTH: u8 = 40;
const PLAYER_TILE_WORLD_HEIGHT: u8 = 55;

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
                    world_data.world_left() + (i32::from(PLAYER_TILE_WORLD_WIDTH) * 2),
                    world_data.world_surface_at() - i32::from(PLAYER_TILE_WORLD_HEIGHT),
                    PLAYER_TILE_WORLD_WIDTH.into(),
                    PLAYER_TILE_WORLD_HEIGHT.into(),
                ),
            })
            .build();
    }
}
