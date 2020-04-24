use crate::components::Drawable;
use crate::components::Player;
use crate::constants;
use crate::entities;
use crate::graphics::data;
use crate::resources::FrameStepper;
use sdl2::rect::Rect;
use specs::join::Join;
use specs::{ReadExpect, ReadStorage, System, WriteStorage};
use std::io::Write;
use std::time::SystemTime;

const FRAMES_PER_ANIMATION: u8 = 20;

pub struct PlayerSystem;

impl PlayerSystem {
    fn update(player_storage: &ReadStorage<Player>, drawable_storage: &mut WriteStorage<Drawable>) {
        for (_, drawable) in (player_storage, drawable_storage).join() {
            let tile = drawable.tile_data.tile;
            if let data::Tile::Character { tile } = tile {
                let tile = match tile {
                    data::CharacterTile::Walk1 => data::CharacterTile::Walk2,
                    data::CharacterTile::Walk2 => data::CharacterTile::Walk1,
                    _ => tile,
                };

                drawable.tile_data = data::build_tile_data(data::Tile::Character { tile });
            }
        }
    }
}

impl<'a> System<'a> for PlayerSystem {
    type SystemData = (
        ReadExpect<'a, FrameStepper>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Drawable>,
    );

    fn run(&mut self, mut data: Self::SystemData) {
        let start_frame_count = data.0.frame_count_animated();
        let end_frame_count = start_frame_count + data.0.frame_count_to_animate();
        if start_frame_count != end_frame_count {
            for frame_count in start_frame_count..end_frame_count {
                if frame_count % FRAMES_PER_ANIMATION as u64 == 0 {
                    PlayerSystem::update(&data.1, &mut data.2)
                }
            }
        }
    }
}
