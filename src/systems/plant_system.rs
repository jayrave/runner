use crate::components::Drawable;
use crate::components::Plant;
use crate::entities;
use crate::resources::FrameStepper;
use crate::WorldData;
use rand::Rng;
use specs::join::Join;
use specs::shred::ResourceId;
use specs::World;
use specs::{Entities, Entity, SystemData};
use specs::{ReadExpect, ReadStorage, System, WriteStorage};

pub struct PlantSystem {
    world_data: WorldData,
}

impl PlantSystem {
    pub fn new(world_data: WorldData) -> PlantSystem {
        PlantSystem { world_data }
    }

    fn move_or_remove(&self, drawable: &mut Drawable, entity: Entity, entities: &Entities) {
        // As soon as a plant moves out, let it go
        if drawable.world_bounds.right() <= self.world_data.world_left() {
            entities.delete(entity).expect("Plant entity to be deleted");
        } else {
            drawable
                .world_bounds
                .offset(-i32::from(self.world_data.ground_speed_in_wc()), 0);
        }
    }
}

#[derive(SystemData)]
pub struct PlantSystemData<'a> {
    entities: Entities<'a>,
    drawables_storage: WriteStorage<'a, Drawable>,
    frame_stepper: ReadExpect<'a, FrameStepper>,
    plants_storage: WriteStorage<'a, Plant>,
}

impl<'a> System<'a> for PlantSystem {
    type SystemData = PlantSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        // animate/remove existing plants
        for (entity, _, mut drawable) in (
            &data.entities,
            &data.plants_storage,
            &mut data.drawables_storage,
        )
            .join()
        {
            for _ in 0..data.frame_stepper.frame_count_to_animate() {
                self.move_or_remove(&mut drawable, entity, &data.entities)
            }
        }

        // Create new plants if willed
        let random_number = rand::thread_rng().gen_range(1u64, 20u64);
        if data.frame_stepper.frame_count_animated() % 20 == 0 {
            entities::Plant::create(
                &self.world_data,
                &data.entities,
                &mut data.drawables_storage,
                &mut data.plants_storage,
            )
        }
    }
}
