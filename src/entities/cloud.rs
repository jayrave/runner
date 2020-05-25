use crate::components;
use crate::components::Drawable;
use crate::data::WorldData;
use crate::graphics::data;
use rand::Rng;
use sdl2::rect::Rect;
use specs::{Entities, WriteStorage};
use std::convert::TryFrom;

const TILE_TO_WORLD_DIVIDER: f32 = 2.0;
const SKY_RANGE_MULTIPLIER_FOR_CLOUD: f32 = 0.75;

pub struct Cloud;

impl Cloud {
    pub fn create(
        world_data: &WorldData,
        tile: data::CloudTile,
        entities: &Entities,
        drawables_storage: &mut WriteStorage<Drawable>,
        clouds_storage: &mut WriteStorage<components::Cloud>,
    ) {
        let world_surface = world_data.world_surface_at();
        let sky_range = (world_data.bounds().top() - world_surface).abs();
        let cloud_range = (sky_range as f32 * SKY_RANGE_MULTIPLIER_FOR_CLOUD) as u32;

        let tile_world_bottom =
            world_data.world_surface_at() - rand::thread_rng().gen_range(0, cloud_range) as i32;

        entities
            .build_entity()
            .with(components::Cloud, clouds_storage)
            .with(
                Cloud::build_drawable_with_left_bottom(
                    tile,
                    world_data.bounds().right(),
                    tile_world_bottom,
                ),
                drawables_storage,
            )
            .build();
    }

    fn build_drawable_with_left_bottom(
        tile: data::CloudTile,
        world_left: i32,
        world_bottom: i32,
    ) -> Drawable {
        let tile_data = data::build_tile_data(data::Tile::Cloud { tile });
        let width_in_world =
            (tile_data.bounds_in_tile_sheet.width() as f32 / TILE_TO_WORLD_DIVIDER) as u32;
        let height_in_world =
            (tile_data.bounds_in_tile_sheet.height() as f32 / TILE_TO_WORLD_DIVIDER) as u32;

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
}
