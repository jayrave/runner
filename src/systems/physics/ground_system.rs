use crate::components::Drawable;
use crate::components::Ground;
use crate::constants;
use crate::entities;
use sdl2::rect::Rect;
use specs::join::Join;
use specs::{ReadStorage, System, WriteStorage};
use std::io::Write;

pub struct GroundSystem;
impl<'a> System<'a> for GroundSystem {
    type SystemData = (
        ReadStorage<'a, Ground>,
        WriteStorage<'a, Drawable>,
    );

    fn run(&mut self, mut data: Self::SystemData) {
        let mut x_offset: Option<i32> = None;
        for (_, drawable) in (&data.0, &mut data.1).join() {
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
