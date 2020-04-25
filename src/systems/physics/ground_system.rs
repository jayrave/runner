use crate::components::Drawable;
use crate::components::Ground;
use crate::entities;
use crate::resources::FrameStepper;
use sdl2::rect::Rect;
use specs::join::Join;
use specs::{ReadExpect, ReadStorage, System, WriteStorage};
use std::io::Write;

const FRAMES_PER_ANIMATION: u8 = 2;

pub struct GroundSystem;

impl GroundSystem {
    fn update(ground_storage: &ReadStorage<Ground>, drawable_storage: &mut WriteStorage<Drawable>) {
        let mut x_offset: Option<i32> = None;
        for (_, drawable) in (ground_storage, drawable_storage).join() {
            if x_offset.is_none() {
                x_offset = Some(match drawable.world_bounds.x() {
                    // TODO - use window bounds for computing this
                    -450 => 50,
                    _ => -2,
                });
            }

            if let Some(x_offset) = x_offset {
                drawable.world_bounds.offset(x_offset, 0);
            }
        }
    }
}

impl<'a> System<'a> for GroundSystem {
    type SystemData = (
        ReadExpect<'a, FrameStepper>,
        ReadStorage<'a, Ground>,
        WriteStorage<'a, Drawable>,
    );

    fn run(&mut self, mut data: Self::SystemData) {
        let start_frame_count = data.0.frame_count_animated();
        let end_frame_count = start_frame_count + data.0.frame_count_to_animate();
        if start_frame_count != end_frame_count {
            for frame_count in start_frame_count..end_frame_count {
                if frame_count % FRAMES_PER_ANIMATION as u64 == 0 {
                    GroundSystem::update(&data.1, &mut data.2)
                }
            }
        }
    }
}
