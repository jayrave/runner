use crate::components;
use crate::components::Drawable;
use crate::data::WorldData;
use crate::graphics::data;
use sdl2::rect::Rect;
use specs::{Entities, World, WorldExt, WriteStorage};
use std::convert::TryFrom;
use std::iter::Iterator;

const GROUND_TILE_WORLD_DIMENSION: u8 = 50;

pub struct Ground;

impl Ground {
    pub fn create_all_tiles(world: &mut World, world_data: &WorldData) {
        Ground::create_all_world_tiles_start_at_world_x(
            world_data.bounds().left(),
            world_data,
            &world.entities(),
            &mut world.write_storage(),
            &mut world.write_storage(),
        );
    }

    pub fn create_all_world_tiles_start_at_world_x(
        starting_at_world_x: i32,
        world_data: &WorldData,
        entities: &Entities,
        drawables_storage: &mut WriteStorage<Drawable>,
        grounds_storage: &mut WriteStorage<components::Ground>,
    ) {
        let total_row_count = u8::try_from(
            (world_data.bounds().bottom() - world_data.world_surface_at())
                / i32::from(GROUND_TILE_WORLD_DIMENSION),
        )
        .expect("Too many ground tiles to draw!");

        for row_number in 1..=total_row_count {
            let tile = if row_number == total_row_count {
                data::PlatformTile::GrassyGround
            } else {
                data::PlatformTile::Ground
            };

            Ground::create_ground_row_starting_at_world_x(
                starting_at_world_x,
                world_data.bounds().bottom() - i32::from(GROUND_TILE_WORLD_DIMENSION * row_number),
                tile,
                world_data,
                entities,
                drawables_storage,
                grounds_storage,
            )
        }
    }

    fn create_ground_row_starting_at_world_x(
        starting_at_world_x: i32,
        tile_top_at_world_y: i32,
        tile: data::PlatformTile,
        world_data: &WorldData,
        entities: &Entities,
        drawables_storage: &mut WriteStorage<Drawable>,
        grounds_storage: &mut WriteStorage<components::Ground>,
    ) {
        // It is ..= is so that we don't miss covering the last pixel
        for world_left in (starting_at_world_x..=world_data.bounds().right())
            .step_by(GROUND_TILE_WORLD_DIMENSION.into())
        {
            entities
                .build_entity()
                .with(components::Ground, grounds_storage)
                .with(
                    components::Drawable {
                        tile_data: data::build_tile_data(data::Tile::Platform { tile }),
                        world_bounds: Rect::new(
                            world_left,
                            tile_top_at_world_y,
                            GROUND_TILE_WORLD_DIMENSION.into(),
                            GROUND_TILE_WORLD_DIMENSION.into(),
                        ),
                    },
                    drawables_storage,
                )
                .build();
        }
    }
}
