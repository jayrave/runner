use crate::components::Drawable;
use crate::components::Plant;
use crate::data::{AnimationData, WorldData};
use crate::entities;
use crate::resources::GameTick;
use rand::Rng;
use specs::join::Join;
use specs::shred::ResourceId;
use specs::World;
use specs::{Entities, Entity, SystemData};
use specs::{ReadExpect, System, WriteStorage};

const MIN_TICKS_BETWEEN_PLANTS: u64 = 250;
const RANDOM_MIN: u64 = 1;
const RANDOM_MAX: u64 = 225;

pub struct PlantSystem {
    animation_data: AnimationData,
    world_data: WorldData,
    last_plant_drawn_at: u64,
}

impl PlantSystem {
    pub fn new(animation_data: AnimationData, world_data: WorldData) -> PlantSystem {
        PlantSystem {
            animation_data,
            world_data,
            last_plant_drawn_at: 0,
        }
    }

    fn move_or_remove(&self, drawable: &mut Drawable, entity: Entity, entities: &Entities) {
        // As soon as a plant moves out, let it go
        if drawable.world_bounds.right() <= self.world_data.world_left() {
            entities.delete(entity).expect("Plant entity to be deleted");
        } else {
            drawable.world_bounds.offset(
                -i32::from(self.animation_data.ground_speed_in_wc_per_tick()),
                0,
            );
        }
    }
}

#[derive(SystemData)]
pub struct PlantSystemData<'a> {
    entities: Entities<'a>,
    drawables_storage: WriteStorage<'a, Drawable>,
    game_tick: ReadExpect<'a, GameTick>,
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
            for _ in 0..data.game_tick.ticks_to_animate() {
                self.move_or_remove(&mut drawable, entity, &data.entities)
            }
        }

        // Create new plants if willed
        let ticks_animated = data.game_tick.ticks_animated();
        if ticks_animated - self.last_plant_drawn_at > MIN_TICKS_BETWEEN_PLANTS {
            let random_number = rand::thread_rng().gen_range(RANDOM_MIN, RANDOM_MAX);
            if ticks_animated % random_number == 0 {
                self.last_plant_drawn_at = ticks_animated;
                entities::Plant::create(
                    &self.world_data,
                    &data.entities,
                    &mut data.drawables_storage,
                    &mut data.plants_storage,
                )
            }
        }
    }
}
