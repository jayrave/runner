use crate::components;
use crate::data::WorldData;
use crate::graphics::data;
use sdl2::rect::Rect;
use specs::{Builder, World, WorldExt};
use std::convert::TryFrom;

const PLAYER_TILE_WORLD_WIDTH: u8 = 48;
const PLAYER_TILE_WORLD_HEIGHT: u8 = 64;

pub struct Player;

impl Player {
    pub fn create(world: &mut World, world_data: &WorldData) {
        world
            .create_entity()
            .with(components::player::Player::new())
            .with(components::input::InputControlled::new())
            .with(components::Animatable {
                current_step_started_at_tick: 0,
            })
            .with(components::Drawable {
                tile_data: data::build_tile_data(data::Tile::Character {
                    tile: data::CharacterTile::Run1,
                }),
                world_bounds: Rect::new(
                    world_data.bounds().left()
                        + i32::try_from(world_data.bounds().width() / 8).unwrap(),
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
