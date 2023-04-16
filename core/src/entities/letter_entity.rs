use crate::components::Letter;
use crate::graphics::data::{LetterTile, Tile};
use specs::World;

const TILE_TO_WORLD_DIVIDER: u32 = 8;
const LETTER_SPACING: u32 = 2;

pub struct LetterEntity;

impl LetterEntity {
    pub fn create_game_instructions_tiles_at_world_center(world: &mut World, extra_y_offset: i32) {
        LetterEntity::create_tiles_at_world_center(
            world,
            extra_y_offset,
            &[
                Tile::Letter {
                    tile: LetterTile::P,
                },
                Tile::Letter {
                    tile: LetterTile::R,
                },
                Tile::Letter {
                    tile: LetterTile::E,
                },
                Tile::Letter {
                    tile: LetterTile::S,
                },
                Tile::Letter {
                    tile: LetterTile::S,
                },
                Tile::Letter {
                    tile: LetterTile::SPACE,
                },
                Tile::Letter {
                    tile: LetterTile::S,
                },
                Tile::Letter {
                    tile: LetterTile::P,
                },
                Tile::Letter {
                    tile: LetterTile::A,
                },
                Tile::Letter {
                    tile: LetterTile::C,
                },
                Tile::Letter {
                    tile: LetterTile::E,
                },
                Tile::Letter {
                    tile: LetterTile::SPACE,
                },
                Tile::Letter {
                    tile: LetterTile::T,
                },
                Tile::Letter {
                    tile: LetterTile::O,
                },
                Tile::Letter {
                    tile: LetterTile::SPACE,
                },
                Tile::Letter {
                    tile: LetterTile::S,
                },
                Tile::Letter {
                    tile: LetterTile::T,
                },
                Tile::Letter {
                    tile: LetterTile::A,
                },
                Tile::Letter {
                    tile: LetterTile::R,
                },
                Tile::Letter {
                    tile: LetterTile::T,
                },
            ],
        );
    }

    pub fn create_retry_tiles_at_world_center(world: &mut World, extra_y_offset: i32) {
        LetterEntity::create_tiles_at_world_center(
            world,
            extra_y_offset,
            &[
                Tile::Letter {
                    tile: LetterTile::W,
                },
                Tile::Letter {
                    tile: LetterTile::O,
                },
                Tile::Letter {
                    tile: LetterTile::U,
                },
                Tile::Letter {
                    tile: LetterTile::L,
                },
                Tile::Letter {
                    tile: LetterTile::D,
                },
                Tile::Letter {
                    tile: LetterTile::SPACE,
                },
                Tile::Letter {
                    tile: LetterTile::Y,
                },
                Tile::Letter {
                    tile: LetterTile::O,
                },
                Tile::Letter {
                    tile: LetterTile::U,
                },
                Tile::Letter {
                    tile: LetterTile::SPACE,
                },
                Tile::Letter {
                    tile: LetterTile::L,
                },
                Tile::Letter {
                    tile: LetterTile::I,
                },
                Tile::Letter {
                    tile: LetterTile::K,
                },
                Tile::Letter {
                    tile: LetterTile::E,
                },
                Tile::Letter {
                    tile: LetterTile::SPACE,
                },
                Tile::Letter {
                    tile: LetterTile::T,
                },
                Tile::Letter {
                    tile: LetterTile::O,
                },
                Tile::Letter {
                    tile: LetterTile::SPACE,
                },
                Tile::Letter {
                    tile: LetterTile::T,
                },
                Tile::Letter {
                    tile: LetterTile::R,
                },
                Tile::Letter {
                    tile: LetterTile::Y,
                },
                Tile::Letter {
                    tile: LetterTile::SPACE,
                },
                Tile::Letter {
                    tile: LetterTile::A,
                },
                Tile::Letter {
                    tile: LetterTile::G,
                },
                Tile::Letter {
                    tile: LetterTile::A,
                },
                Tile::Letter {
                    tile: LetterTile::I,
                },
                Tile::Letter {
                    tile: LetterTile::N,
                },
            ],
        );
    }

    pub fn create_celebration_tiles_at_world_center(world: &mut World, extra_y_offset: i32) -> i32 {
        LetterEntity::create_tiles_at_world_center(
            world,
            extra_y_offset,
            &[
                Tile::Letter {
                    tile: LetterTile::H,
                },
                Tile::Letter {
                    tile: LetterTile::A,
                },
                Tile::Letter {
                    tile: LetterTile::P,
                },
                Tile::Letter {
                    tile: LetterTile::P,
                },
                Tile::Letter {
                    tile: LetterTile::Y,
                },
                Tile::Letter {
                    tile: LetterTile::SPACE,
                },
                Tile::Letter {
                    tile: LetterTile::B,
                },
                Tile::Letter {
                    tile: LetterTile::I,
                },
                Tile::Letter {
                    tile: LetterTile::R,
                },
                Tile::Letter {
                    tile: LetterTile::T,
                },
                Tile::Letter {
                    tile: LetterTile::H,
                },
                Tile::Letter {
                    tile: LetterTile::D,
                },
                Tile::Letter {
                    tile: LetterTile::A,
                },
                Tile::Letter {
                    tile: LetterTile::Y,
                },
                Tile::Letter {
                    tile: LetterTile::SPACE,
                },
                Tile::Letter {
                    tile: LetterTile::J,
                },
                Tile::Letter {
                    tile: LetterTile::I,
                },
                Tile::Letter {
                    tile: LetterTile::L,
                },
                Tile::Letter {
                    tile: LetterTile::L,
                },
                Tile::Letter {
                    tile: LetterTile::U,
                },
            ],
        )
    }

    pub fn remove_all_tiles(world: &mut World) {
        super::instruction_entitities_helper::remove_all_entities_matching_type::<Letter>(world);
    }

    fn create_tiles_at_world_center(world: &mut World, extra_y_offset: i32, tiles: &[Tile]) -> i32 {
        super::instruction_entitities_helper::create_tiles_at_world_center(
            world,
            Letter,
            TILE_TO_WORLD_DIVIDER,
            LETTER_SPACING as i32,
            extra_y_offset,
            tiles,
        )
    }
}
