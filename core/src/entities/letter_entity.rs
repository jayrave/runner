use crate::components::{Drawable, Letter};
use crate::graphics::data;
use crate::graphics::data::{LetterTile, Tile};
use crate::rect::Rect;
use specs::join::Join;
use specs::{Builder, World, WorldExt};

const TILE_TO_WORLD_DIVIDER: u32 = 8;
const LETTER_SPACING: u32 = 2;

pub struct LetterEntity;

impl LetterEntity {
    pub fn create_game_instructions_tiles_at_world_center(world: &mut World) -> u32 {
        LetterEntity::create_tiles_at_world_center(
            world,
            &[
                LetterTile::P,
                LetterTile::R,
                LetterTile::E,
                LetterTile::S,
                LetterTile::S,
                LetterTile::SPACE,
                LetterTile::S,
                LetterTile::P,
                LetterTile::A,
                LetterTile::C,
                LetterTile::E,
                LetterTile::SPACE,
                LetterTile::T,
                LetterTile::O,
                LetterTile::SPACE,
                LetterTile::S,
                LetterTile::T,
                LetterTile::A,
                LetterTile::R,
                LetterTile::T,
            ],
        )
    }

    pub fn create_retry_tiles_at_world_center(world: &mut World) -> u32 {
        LetterEntity::create_tiles_at_world_center(
            world,
            &[
                LetterTile::P,
                LetterTile::R,
                LetterTile::E,
                LetterTile::S,
                LetterTile::S,
                LetterTile::SPACE,
                LetterTile::S,
                LetterTile::P,
                LetterTile::A,
                LetterTile::C,
                LetterTile::E,
                LetterTile::SPACE,
                LetterTile::T,
                LetterTile::O,
                LetterTile::SPACE,
                LetterTile::P,
                LetterTile::L,
                LetterTile::A,
                LetterTile::Y,
                LetterTile::SPACE,
                LetterTile::A,
                LetterTile::G,
                LetterTile::A,
                LetterTile::I,
                LetterTile::N,
            ]
        )
    }

    pub fn remove_all_tiles(world: &mut World) {
        super::entity_remover::remove_all_entities_matching_type::<Letter>(world);
    }

    fn create_tiles_at_world_center(world: &mut World, tiles: &[LetterTile]) -> u32 {
        // Initially create drawables at 0, 0
        let mut last_icon_x_end_at = 0;
        let mut max_drawable_height = 0;
        let drawables: Vec<Drawable> = tiles
            .iter()
            .map(|tile| {
                let tile_data = data::build_tile_data(Tile::Letter { tile: *tile });
                let width_in_world = tile_data.bounds_in_tile_sheet.width() / TILE_TO_WORLD_DIVIDER;
                let height_in_world =
                    tile_data.bounds_in_tile_sheet.height() / TILE_TO_WORLD_DIVIDER;

                let bounds_in_world = Rect::new(
                    last_icon_x_end_at + LETTER_SPACING as i32,
                    0,
                    width_in_world,
                    height_in_world,
                );

                // Update tracked measures
                last_icon_x_end_at = bounds_in_world.right();
                if max_drawable_height < height_in_world {
                    max_drawable_height = height_in_world;
                }

                Drawable {
                    tile_data,
                    world_bounds: bounds_in_world,
                }
            })
            .collect();

        // Compute offsets to move drawables to world center
        let height_offset: i32 = -((max_drawable_height / 2) as i32);
        let width_offset = -last_icon_x_end_at / 2;

        // Offset drawables & create entities
        for mut drawable in drawables.into_iter() {
            drawable.world_bounds.offset(width_offset, height_offset);
            world.create_entity().with(Letter).with(drawable).build();
        }

        height_offset.abs() as u32
    }
}
