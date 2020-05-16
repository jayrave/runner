use crate::components;
use crate::data::WorldData;
use crate::graphics::data;
use sdl2::rect::Rect;
use specs::{Builder, World, WorldExt};
use std::convert::TryFrom;
use std::iter::Iterator;

const GROUND_TILE_WORLD_DIMENSION: u8 = 50;

pub struct Ground;

impl Ground {
    pub fn create_all_tiles(world: &mut World, world_data: &WorldData) {
        let total_row_count = u8::try_from(
            (world_data.bounds().bottom() - world_data.world_surface_at())
                / i32::from(GROUND_TILE_WORLD_DIMENSION),
        )
        .expect("Too many ground tiles to draw!");

        for row_number in 1..=total_row_count {
            Ground::create_ground_row(
                world,
                world_data,
                world_data.bounds().bottom() - i32::from(GROUND_TILE_WORLD_DIMENSION * row_number),
                if row_number == total_row_count {
                    data::PlatformTile::GrassyGround
                } else {
                    data::PlatformTile::Ground
                },
            )
        }
    }

    fn create_ground_row(
        world: &mut World,
        world_data: &WorldData,
        tile_top_at_world_y: i32,
        with_tile: data::PlatformTile,
    ) {
        // It is ..= so we would have one extra tile. This way we can draw
        // partial tiles at the end & cover all of the window width
        for world_left in (world_data.bounds().left()..=world_data.bounds().right())
            .step_by(GROUND_TILE_WORLD_DIMENSION.into())
        {
            world
                .create_entity()
                .with(components::Ground)
                .with(components::Drawable {
                    tile_data: data::build_tile_data(data::Tile::Platform { tile: with_tile }),
                    world_bounds: Rect::new(
                        world_left,
                        tile_top_at_world_y,
                        GROUND_TILE_WORLD_DIMENSION.into(),
                        GROUND_TILE_WORLD_DIMENSION.into(),
                    ),
                })
                .build();
        }
    }
}
