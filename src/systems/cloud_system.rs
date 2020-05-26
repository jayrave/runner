use crate::components::Cloud;
use crate::components::Drawable;
use crate::data::{CloudData, GroundData, WorldData};
use crate::entities;
use crate::graphics::data;
use crate::resources::GamePlay;
use rand::Rng;
use specs::join::Join;
use specs::shred::ResourceId;
use specs::World;
use specs::{Entities, Entity, SystemData};
use specs::{ReadExpect, System, WriteStorage};

pub struct CloudSystem {
    ground_data: GroundData,
    world_data: WorldData,
    cloud_wave_started_at_tick: u64,
    clouds_spawned_in_current_wave: u8,
}

impl CloudSystem {
    pub fn new(ground_data: GroundData, world_data: WorldData) -> CloudSystem {
        CloudSystem {
            ground_data,
            world_data,
            cloud_wave_started_at_tick: 0,
            clouds_spawned_in_current_wave: 0,
        }
    }

    fn move_or_remove(
        &self,
        entities: &Entities,
        entity: Entity,
        cloud: &Cloud,
        drawable: &mut Drawable,
    ) {
        // As soon as an enemy moves out, let it go
        if drawable.world_bounds.right() <= self.world_data.bounds().left() {
            entities
                .delete(entity)
                .expect("Cloud entity couldn't be deleted");
        } else {
            // For every tick, the cloud should move at least a bit
            drawable
                .world_bounds
                .offset(-i32::from(cloud.speed_in_wc_per_tick), 0);
        }
    }

    fn should_spawn_cloud(&mut self, current_tick: u64, cloud_data: &CloudData) -> bool {
        // Start new wave if required
        if current_tick - self.cloud_wave_started_at_tick > cloud_data.cloud_wave_ticks_count.into()
        {
            self.cloud_wave_started_at_tick = current_tick;
            self.clouds_spawned_in_current_wave = 0;
        }

        let clouds_remaining_in_wave = cloud_data.cloud_count_in_wave
            - self
                .clouds_spawned_in_current_wave
                .min(cloud_data.cloud_count_in_wave);

        if clouds_remaining_in_wave <= 0 {
            false
        } else {
            let ticks_remaining_in_wave: u64 = (self.cloud_wave_started_at_tick
                + cloud_data.cloud_wave_ticks_count as u64)
                - current_tick;

            // Would panic if low == high. Hence `max(1)` for upper bound
            let spawn_cloud = rand::thread_rng().gen_range(
                0,
                (ticks_remaining_in_wave / clouds_remaining_in_wave as u64).max(1),
            ) == 0;

            if spawn_cloud {
                self.clouds_spawned_in_current_wave += 1
            }

            spawn_cloud
        }
    }

    fn get_random_cloud_tile() -> data::CloudTile {
        match rand::thread_rng().gen_range(1, 5) {
            1 => data::CloudTile::Cloud1,
            2 => data::CloudTile::Cloud2,
            3 => data::CloudTile::Cloud3,
            _ => data::CloudTile::Cloud4,
        }
    }
}

#[derive(SystemData)]
pub struct CloudSystemData<'a> {
    entities: Entities<'a>,
    cloud_data: ReadExpect<'a, CloudData>,
    clouds_storage: WriteStorage<'a, Cloud>,
    drawables_storage: WriteStorage<'a, Drawable>,
    game_play: ReadExpect<'a, GamePlay>,
}

impl<'a> System<'a> for CloudSystem {
    type SystemData = CloudSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        // animate/remove existing cloud
        for (cloud, entity, mut drawable) in (
            &data.clouds_storage,
            &data.entities,
            &mut data.drawables_storage,
        )
            .join()
        {
            let start_tick = data.game_play.ticks_animated();
            let end_tick = start_tick + data.game_play.ticks_to_animate();
            for _ in start_tick..end_tick {
                self.move_or_remove(&data.entities, entity, cloud, &mut drawable)
            }
        }

        // Create new clouds if possible & required
        if self.should_spawn_cloud(data.game_play.ticks_animated(), &data.cloud_data) {
            let cloud_tile = CloudSystem::get_random_cloud_tile();
            entities::CloudEntity::create(
                &self.ground_data,
                &self.world_data,
                cloud_tile,
                &data.entities,
                &mut data.drawables_storage,
                &mut data.clouds_storage,
            )
        }
    }
}
