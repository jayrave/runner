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

pub struct PlantSystem {
    animation_data: AnimationData,
    world_data: WorldData,
    last_plant_at_tick: u64,
}

impl PlantSystem {
    pub fn new(animation_data: AnimationData, world_data: WorldData) -> PlantSystem {
        PlantSystem {
            animation_data,
            world_data,
            last_plant_at_tick: 0,
        }
    }

    fn move_or_remove(&self, drawable: &mut Drawable, entity: Entity, entities: &Entities) {
        // As soon as a plant moves out, let it go
        if drawable.world_bounds.right() <= self.world_data.bounds().left() {
            entities
                .delete(entity)
                .expect("Plant entity couldn't be deleted");
        } else {
            drawable.world_bounds.offset(
                -i32::from(self.animation_data.ground_speed_in_wc_per_tick()),
                0,
            );
        }
    }

    fn can_create_new_plant(&mut self, game_tick: &GameTick) -> bool {
        let ticks_animated = game_tick.ticks_animated();
        let ticks_since_last_plant = ticks_animated - self.last_plant_at_tick;
        if ticks_since_last_plant > self.animation_data.min_ticks_between_plants()
            && ticks_animated % 60 == 0
            && rand::thread_rng().gen_range(1, 11) == 5
        {
            self.last_plant_at_tick = ticks_animated;
            true
        } else {
            false
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

        // Create new plants if possible & required
        if self.can_create_new_plant(&data.game_tick) {
            entities::Plant::create(
                &self.world_data,
                &data.entities,
                &mut data.drawables_storage,
                &mut data.plants_storage,
            )
        }
    }
}
