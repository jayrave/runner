use crate::resources::GamePlay;
use specs::shred::ResourceId;
use specs::{SystemData, WriteStorage, ReadStorage};
use specs::World;
use specs::WriteExpect;
use crate::graphics::data;
use specs::{ReadExpect, System};
use specs::join::Join;
use crate::components::Drawable;
use crate::components::score::{Score, Position};
use crate::graphics::data::NumberTile;

pub struct ScoreSystem;

#[derive(SystemData)]
pub struct ScoreUpdaterSystemData<'a> {
    game_play: ReadExpect<'a, GamePlay>,
    scores_storage: ReadStorage<'a, Score>,
    drawables_storage: WriteStorage<'a, Drawable>,
}

impl<'a> System<'a> for ScoreSystem {
    type SystemData = ScoreUpdaterSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        let current_score = data.game_play.ticks_animated();
        for (score, mut drawable) in (&data.scores_storage, &mut data.drawables_storage).join() {
            let number_to_find_remainder = match score.position {
                Position::One => 10,
                Position::Ten => 100,
                Position::Hundred => 1000,
                Position::Thousand => 10000,
                Position::TenThousand => 100000,
                Position::HundredThousand => 1000000,
            };

            let remainder = current_score % number_to_find_remainder;
            let single_digit = remainder / (number_to_find_remainder / 10);

            drawable.tile_data = data::build_tile_data(data::Tile::Number {
                tile: match single_digit {
                    0 => NumberTile::ZERO,
                    1 => NumberTile::ONE,
                    2 => NumberTile::TWO,
                    3 => NumberTile::THREE,
                    4 => NumberTile::FOUR,
                    5 => NumberTile::FIVE,
                    6 => NumberTile::SIX,
                    7 => NumberTile::SEVEN,
                    8 => NumberTile::EIGHT,
                    9 => NumberTile::NINE,
                    _ => panic!("Score math is off!")
                },
            });
        }
    }
}
