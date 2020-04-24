use crate::components::Drawable;
use crate::components::Player;
use crate::resources::FrameStepper;
use crate::constants;
use crate::entities;
use crate::graphics::data;
use sdl2::rect::Rect;
use specs::join::Join;
use specs::{ReadStorage, System, WriteStorage, ReadExpect};
use std::io::Write;

pub struct PlayerSystem;
impl<'a> System<'a> for PlayerSystem {
    type SystemData = (
        ReadExpect<'a, FrameStepper>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Drawable>,
    );

    fn run(&mut self, mut data: Self::SystemData) {
        for _ in 1..=data.0.frame_count_to_animate() {
            for (_, drawable) in (&data.1, &mut data.2).join() {
                let tile = drawable.tile_data.tile;
                if let data::Tile::Character { tile } = tile {
                    let tile = match tile {
                        data::CharacterTile::Walk1 => data::CharacterTile::Walk2,
                        data::CharacterTile::Walk2 => data::CharacterTile::Walk1,
                        _ => tile
                    };

                    drawable.tile_data = data::build_tile_data(data::Tile::Character { tile });
                }
            }
        }
    }
}
