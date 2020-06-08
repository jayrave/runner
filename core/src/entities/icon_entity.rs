use crate::components::Icon;
use crate::graphics::data::{IconTile, Tile};
use specs::World;

const TILE_TO_WORLD_DIVIDER: u32 = 1;
const ICON_SPACING: u32 = 8;

pub struct IconEntity;

impl IconEntity {
    /// Returns the max bottom of all the tiles created
    pub fn create_direction_tiles_at_world_center(world: &mut World) -> i32 {
        IconEntity::create_tiles_at_world_center(
            world,
            &[
                Tile::Icon {
                    tile: IconTile::Left,
                },
                Tile::Icon {
                    tile: IconTile::Down,
                },
                Tile::Icon { tile: IconTile::Up },
                Tile::Icon {
                    tile: IconTile::Right,
                },
            ],
        )
    }

    /// Returns the max bottom of all the tiles created
    pub fn create_retry_tile_at_world_center(world: &mut World) -> i32 {
        IconEntity::create_tiles_at_world_center(
            world,
            &[Tile::Icon {
                tile: IconTile::Retry,
            }],
        )
    }

    pub fn remove_all_tiles(world: &mut World) {
        super::instruction_entitities_helper::remove_all_entities_matching_type::<Icon>(world);
    }

    fn create_tiles_at_world_center(world: &mut World, tiles: &[Tile]) -> i32 {
        super::instruction_entitities_helper::create_tiles_at_world_center(
            world,
            Icon,
            TILE_TO_WORLD_DIVIDER,
            ICON_SPACING as i32,
            0,
            tiles,
        )
    }
}
