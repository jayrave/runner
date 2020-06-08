use crate::components::score::{Position, Score};
use crate::components::Drawable;
use crate::data::WorldData;
use crate::graphics::data;
use crate::graphics::data::NumberTile;
use crate::rect::Rect;
use specs::{Builder, World, WorldExt};

const TILE_TO_WORLD_DIVIDER: u32 = 8;
const DISTANCE_FROM_WORLD_EDGE: i32 = 16;
const NUMBER_SPACING: u32 = 2;

pub struct ScoreEntity;

impl ScoreEntity {
    pub fn create_all_tiles(world: &mut World, world_data: &WorldData) {
        ScoreEntity::create_tile(world, world_data, Position::One);
        ScoreEntity::create_tile(world, world_data, Position::Ten);
        ScoreEntity::create_tile(world, world_data, Position::Hundred);
        ScoreEntity::create_tile(world, world_data, Position::Thousand);
        ScoreEntity::create_tile(world, world_data, Position::TenThousand);
        ScoreEntity::create_tile(world, world_data, Position::HundredThousand);
    }

    fn create_tile(world: &mut World, world_data: &WorldData, position: Position) {
        let tile_data = data::build_tile_data(data::Tile::Number {
            tile: NumberTile::Zero,
        });
        let width_in_world = tile_data.bounds_in_tile_sheet.width() / TILE_TO_WORLD_DIVIDER;
        let height_in_world = tile_data.bounds_in_tile_sheet.height() / TILE_TO_WORLD_DIVIDER;

        let distance_from_world_right = DISTANCE_FROM_WORLD_EDGE
            + match position {
                Position::One => 1,
                Position::Ten => 2,
                Position::Hundred => 3,
                Position::Thousand => 4,
                Position::TenThousand => 5,
                Position::HundredThousand => 6,
            } * (width_in_world + NUMBER_SPACING) as i32;

        let drawable = Drawable {
            tile_data,
            world_bounds: Rect::new(
                world_data.bounds().right() - distance_from_world_right,
                world_data.bounds().top() + DISTANCE_FROM_WORLD_EDGE,
                width_in_world,
                height_in_world,
            ),
        };

        world
            .create_entity()
            .with(Score { position })
            .with(drawable)
            .build();
    }
}
