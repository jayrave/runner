use crate::components;
use crate::components::Drawable;
use crate::data::{PlayerData, WorldData};
use crate::graphics::data;

use sdl2::rect::Rect;
use specs::{Builder, World, WorldExt};
use std::convert::TryFrom;

const TILE_TO_WORLD_DIVIDER: u32 = 2;

pub struct PlayerEntity;

impl PlayerEntity {
    pub fn create(world: &mut World, world_data: &WorldData) {
        world
            .create_entity()
            .with(components::player::Player::new())
            .with(components::input::InputControlled::new())
            .with(components::Animatable {
                current_step_started_at_tick: 0,
            })
            .with(PlayerEntity::build_drawable_with_left_bottom(
                data::CharacterTile::Still,
                world_data.bounds().left() + (world_data.bounds().width() / 8) as i32,
                world_data.world_surface_at(),
            ))
            .build();
    }

    pub fn build_drawable_with_left_bottom(
        tile: data::CharacterTile,
        world_left: i32,
        world_bottom: i32,
    ) -> Drawable {
        let tile_data = data::build_tile_data(data::Tile::Character { tile });
        let width_in_world = tile_data.bounds_in_tile_sheet.width() / TILE_TO_WORLD_DIVIDER;
        let height_in_world = tile_data.bounds_in_tile_sheet.height() / TILE_TO_WORLD_DIVIDER;

        components::Drawable {
            tile_data,
            world_bounds: Rect::new(
                world_left,
                world_bottom - i32::try_from(height_in_world).expect("u32 too big for i32"),
                width_in_world,
                height_in_world,
            ),
        }
    }

    pub fn bottom_when_max_jumping(world_data: &WorldData, player_data: &PlayerData) -> i32 {
        world_data.world_surface_at() - i32::from(player_data.max_jump_height_in_wc)
    }

    pub fn top_when_sliding(world_data: &WorldData) -> i32 {
        PlayerEntity::build_drawable_with_left_bottom(
            data::CharacterTile::Slide,
            world_data.bounds().right(),
            world_data.world_surface_at(),
        )
        .world_bounds
        .top()
    }
}
