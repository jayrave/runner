use crate::components;
use crate::constants;
use crate::graphics::data;
use sdl2::rect::Rect;
use specs::{Builder, World, WorldExt};
use std::iter::Iterator;

const GROUND_TILE_WORLD_DIMENSION: u8 = 50;
const GROUND_TILE_ROW_COUNT: u8 = 3;
pub struct Ground;
impl Ground {
    pub fn create_all_tiles(world: &mut World) {
        for row_count in 1..=3 {
            Ground::create_ground_row(
                world,
                constants::WORLD_BOTTOM - (GROUND_TILE_WORLD_DIMENSION * row_count) as i32,
                match row_count {
                    GROUND_TILE_ROW_COUNT => data::EnvironmentTile::GrassyGround,
                    _ => data::EnvironmentTile::Ground,
                },
            )
        }
    }

    fn create_ground_row(
        world: &mut World,
        tile_top_at_world_y: i32,
        with_tile: data::EnvironmentTile,
    ) {
        for world_left in (constants::WORLD_LEFT..constants::WORLD_RIGHT)
            .step_by(GROUND_TILE_WORLD_DIMENSION.into())
        {
            world.create_entity().with(components::Drawable {
                tile_data: data::build_tile_data(data::Tile::Environment { tile: with_tile }),
                world_bounds: Rect::new(
                    world_left,
                    tile_top_at_world_y.into(),
                    GROUND_TILE_WORLD_DIMENSION.into(),
                    GROUND_TILE_WORLD_DIMENSION.into(),
                ),
            }).build();
        }
    }
}
