use crate::components::Drawable;
use crate::components::Ground;
use crate::resources::FrameStepper;
use crate::{WorldData};

use specs::join::Join;
use specs::shred::ResourceId;
use specs::SystemData;
use specs::World;
use specs::{ReadExpect, ReadStorage, System, WriteStorage};
use std::convert::TryFrom;


const FRAMES_PER_TILE: u8 = 2;
const X_OFFSET_PER_FRAME: i32 = -2;

pub struct GroundSystem {
    world_data: WorldData,
}

impl GroundSystem {
    pub fn new(world_data: WorldData) -> GroundSystem {
        GroundSystem { world_data }
    }

    fn update(
        &self,
        grounds_storage: &ReadStorage<Ground>,
        drawables_storage: &mut WriteStorage<Drawable>,
    ) {
        for (_, drawable) in (grounds_storage, drawables_storage).join() {
            // Instead of `<=`, I am going to stick with `==` so I can be easily
            // figure out when the precondition (movement being a factor of world
            // width) fails. Wrap around tile if it is off screen
            if drawable.world_bounds.right() == self.world_data.world_left() {
                let x_offset =
                    i32::try_from(self.world_data.world_width() + drawable.world_bounds.width())
                        .expect("World or tile is too wide");

                drawable.world_bounds.offset(x_offset, 0);
            }

            // Every tile needs to be moved to the left by a few world coordinates
            drawable.world_bounds.offset(X_OFFSET_PER_FRAME, 0);
        }
    }
}

#[derive(SystemData)]
pub struct GroundSystemData<'a> {
    frame_stepper: ReadExpect<'a, FrameStepper>,
    grounds_storage: ReadStorage<'a, Ground>,
    drawables_storage: WriteStorage<'a, Drawable>,
}

impl<'a> System<'a> for GroundSystem {
    type SystemData = GroundSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        let start_frame_count = data.frame_stepper.frame_count_animated();
        let end_frame_count = start_frame_count + data.frame_stepper.frame_count_to_animate();
        if start_frame_count != end_frame_count {
            for frame_count in start_frame_count..end_frame_count {
                if frame_count % u64::from(FRAMES_PER_TILE) == 0 {
                    self.update(&data.grounds_storage, &mut data.drawables_storage)
                }
            }
        }
    }
}
