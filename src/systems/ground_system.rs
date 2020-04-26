use crate::components::Drawable;
use crate::components::Ground;
use crate::resources::FrameStepper;
use crate::WorldData;
use specs::join::Join;
use specs::shred::ResourceId;
use specs::SystemData;
use specs::World;
use specs::{ReadExpect, ReadStorage, System, WriteStorage};

const X_OFFSET_PER_FRAME: i32 = -2;

pub struct GroundSystem {
    world_data: WorldData,
}

impl GroundSystem {
    pub fn new(world_data: WorldData) -> GroundSystem {
        GroundSystem { world_data }
    }

    fn update(&self, drawable: &mut Drawable) {
        // Instead of `<=`, I am going to stick with `==` so I can be easily
        // figure out when the precondition (movement being a factor of world
        // width) fails. Wrap around tile if it is off screen
        if drawable.world_bounds.right() == self.world_data.world_left() {
            drawable.world_bounds.set_x(self.world_data.world_right());
        }

        // Every tile needs to be moved to the left by a few world coordinates
        drawable.world_bounds.offset(X_OFFSET_PER_FRAME, 0);
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
        for (_, mut drawable) in (&data.grounds_storage, &mut data.drawables_storage).join() {
            for _ in 0..data.frame_stepper.frame_count_to_animate() {
                self.update(&mut drawable)
            }
        }
    }
}
