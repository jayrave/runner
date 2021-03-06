use crate::components::{Drawable, Ground};
use crate::data::WorldData;
use crate::graphics::data;
use crate::graphics::data::PlatformTile;
use crate::rect::Rect;
use specs::{Entities, World, WorldExt, WriteStorage};
use std::iter::Iterator;

const GROUND_TILE_WORLD_DIMENSION: u8 = 50;

pub struct GroundEntity;

impl GroundEntity {
    pub fn create_all_tiles(world: &mut World, world_data: &WorldData) {
        GroundEntity::create_all_world_tiles_start_at_world_x(
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
        grounds_storage: &mut WriteStorage<Ground>,
    ) {
        // `ceil` is to make sure that even if ground tiles don't exactly add up to
        // the surface, we can still enough tiles to cover the required area
        let total_row_count = ((world_data.bounds().bottom() - world_data.world_surface_at())
            as f32
            / f32::from(GROUND_TILE_WORLD_DIMENSION))
        .ceil() as u8;

        for row_number in 0..total_row_count {
            let tile = if row_number == 0 {
                PlatformTile::GrassyGround
            } else {
                PlatformTile::Ground
            };

            GroundEntity::create_ground_row_starting_at_world_x(
                starting_at_world_x,
                world_data.world_surface_at() + i32::from(GROUND_TILE_WORLD_DIMENSION * row_number),
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
        tile: PlatformTile,
        world_data: &WorldData,
        entities: &Entities,
        drawables_storage: &mut WriteStorage<Drawable>,
        grounds_storage: &mut WriteStorage<Ground>,
    ) {
        // It is ..= is so that we don't miss covering the last pixel
        for world_left in (starting_at_world_x..=world_data.bounds().right())
            .step_by(GROUND_TILE_WORLD_DIMENSION.into())
        {
            entities
                .build_entity()
                .with(Ground, grounds_storage)
                .with(
                    Drawable {
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
