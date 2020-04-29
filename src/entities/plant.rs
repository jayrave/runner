use crate::components;
use crate::components::Drawable;
use crate::data::WorldData;
use crate::graphics::data;
use sdl2::rect::Rect;
use specs::{Entities, WriteStorage};

const PLANT_TILE_WORLD_DIMENSION: u8 = 24;

pub struct Plant;

impl Plant {
    pub fn create(
        world_data: &WorldData,
        entities: &Entities,
        drawables_storage: &mut WriteStorage<Drawable>,
        plants_storage: &mut WriteStorage<components::Plant>,
    ) {
        entities
            .build_entity()
            .with(components::Plant, plants_storage)
            .with(
                components::Drawable {
                    tile_data: data::build_tile_data(data::Tile::Environment {
                        tile: data::EnvironmentTile::Plant,
                    }),
                    world_bounds: Rect::new(
                        world_data.bounds().right(),
                        world_data.world_surface_at() - i32::from(PLANT_TILE_WORLD_DIMENSION),
                        PLANT_TILE_WORLD_DIMENSION.into(),
                        PLANT_TILE_WORLD_DIMENSION.into(),
                    ),
                },
                drawables_storage,
            )
            .build();
    }
}
