use crate::components;
use crate::components::Drawable;
use crate::data::{GroundData, WorldData};
use crate::graphics::data;
use rand::Rng;
use sdl2::rect::Rect;
use specs::{Entities, WriteStorage};
use std::convert::TryFrom;

const MIN_SPEED_NEGATOR: i32 = 2;
const MAX_SPEED_NEGATOR: i32 = 12;
const MIN_TILE_TO_WORLD_DIVIDER: f32 = 3.0;
const MAX_TILE_TO_WORLD_DIVIDER: f32 = 8.0;
const SKY_RANGE_MULTIPLIER_FOR_CLOUD: f32 = 0.75;

pub struct CloudEntity;

impl CloudEntity {
    pub fn create(
        ground_data: &GroundData,
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
            .with(
                components::Cloud::new(CloudEntity::random_cloud_speed_in_wc_per_tick(ground_data)),
                clouds_storage,
            )
            .with(
                CloudEntity::build_drawable_with_left_bottom(
                    tile,
                    world_data.bounds().right(),
                    tile_world_bottom,
                ),
                drawables_storage,
            )
            .build();
    }

    fn random_cloud_speed_in_wc_per_tick(ground_data: &GroundData) -> u16 {
        let random_cloud_speed_negator =
            rand::thread_rng().gen_range(MIN_SPEED_NEGATOR, MAX_SPEED_NEGATOR);

        (i32::from(ground_data.speed_in_wc_per_tick) - random_cloud_speed_negator).max(1) as u16
    }

    fn build_drawable_with_left_bottom(
        tile: data::CloudTile,
        world_left: i32,
        world_bottom: i32,
    ) -> Drawable {
        let tile_data = data::build_tile_data(data::Tile::Cloud { tile });
        let divider =
            rand::thread_rng().gen_range(MIN_TILE_TO_WORLD_DIVIDER, MAX_TILE_TO_WORLD_DIVIDER);

        let width_in_world = (tile_data.bounds_in_tile_sheet.width() as f32 / divider) as u32;
        let height_in_world = (tile_data.bounds_in_tile_sheet.height() as f32 / divider) as u32;

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