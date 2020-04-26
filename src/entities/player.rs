use crate::components;
use crate::data::WorldData;
use crate::graphics::data;
use sdl2::rect::Rect;
use specs::{Builder, World, WorldExt};

const PLAYER_TILE_WORLD_WIDTH: u8 = 48;
const PLAYER_TILE_WORLD_HEIGHT: u8 = 64;

pub struct Player;

impl Player {
    pub fn create(world: &mut World, world_data: &WorldData) {
        world
            .create_entity()
            .with(components::Player {
                current_step_started_at_tick: 0,
            })
            .with(components::input::InputControlled::new())
            .with(components::Drawable {
                tile_data: data::build_tile_data(data::Tile::Character {
                    tile: data::CharacterTile::Run1,
                }),
                world_bounds: Rect::new(
                    world_data.world_left() + (i32::from(PLAYER_TILE_WORLD_WIDTH) * 2),
                    Player::running_y(world_data),
                    PLAYER_TILE_WORLD_WIDTH.into(),
                    PLAYER_TILE_WORLD_HEIGHT.into(),
                ),
            })
            .build();
    }

    pub fn running_y(world_data: &WorldData) -> i32 {
        world_data.world_surface_at() - i32::from(PLAYER_TILE_WORLD_HEIGHT)
    }
}
