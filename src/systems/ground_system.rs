use crate::components::Drawable;
use crate::components::Ground;
use crate::data::{GroundData, WorldData};
use crate::resources::GameTick;
use specs::join::Join;
use specs::shred::ResourceId;
use specs::SystemData;
use specs::World;
use specs::{ReadExpect, ReadStorage, System, WriteStorage};

pub struct GroundSystem {
    world_data: WorldData,
}

impl GroundSystem {
    pub fn new(world_data: WorldData) -> GroundSystem {
        GroundSystem { world_data }
    }

    fn update(&self, ground_data: &GroundData, drawable: &mut Drawable) {
        // World left will be in negative & so if this diff is positive, the tile
        // is completely outside the world. Wrap it around
        let diff = self.world_data.bounds().left() - drawable.world_bounds.right();
        if diff > 0 {
            drawable
                .world_bounds
                .set_x(self.world_data.bounds().right() - diff);
        }

        // Every tile needs to be moved to the left by a few world coordinates
        drawable
            .world_bounds
            .offset(-i32::from(ground_data.speed_in_wc_per_tick), 0);
    }
}

#[derive(SystemData)]
pub struct GroundSystemData<'a> {
    game_tick: ReadExpect<'a, GameTick>,
    ground_data: ReadExpect<'a, GroundData>,
    grounds_storage: ReadStorage<'a, Ground>,
    drawables_storage: WriteStorage<'a, Drawable>,
}

impl<'a> System<'a> for GroundSystem {
    type SystemData = GroundSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        for (_, mut drawable) in (&data.grounds_storage, &mut data.drawables_storage).join() {
            for _ in 0..data.game_tick.ticks_to_animate() {
                self.update(&data.ground_data, &mut drawable)
            }
        }
    }
}
