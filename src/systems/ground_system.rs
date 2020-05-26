use crate::components::Drawable;
use crate::components::Ground;
use crate::data::{GroundData, WorldData};
use crate::resources::GamePlay;
use specs::join::Join;
use specs::shred::ResourceId;
use specs::World;
use specs::{Entities, Entity, SystemData};
use specs::{ReadExpect, System, WriteStorage};

pub struct GroundSystem {
    world_data: WorldData,
}

impl GroundSystem {
    pub fn new(world_data: WorldData) -> GroundSystem {
        GroundSystem { world_data }
    }

    fn move_or_remove(
        &self,
        entities: &Entities,
        entity: Entity,
        ground_data: GroundData,
        drawable: &mut Drawable,
    ) {
        // Every tile needs to be moved to the left by a few world coordinates. Even
        // if a tile is outside the world bounds, keep moving it. This way everyone
        // will stick to the same grid even when creating a new tile based on the
        // current tiles. This will make sure there aren't any surprises in case any
        // computation is based on the min number
        drawable
            .world_bounds
            .offset(-i32::from(ground_data.speed_in_wc_per_tick), 0);

        // If ground tile is outside the world bounds! Remove. The delete won't get
        // into effect until a call to `World#maintain`
        if drawable.world_bounds.right() < self.world_data.bounds().left() {
            entities
                .delete(entity)
                .expect("Ground entity couldn't be deleted");
        }
    }
}

#[derive(SystemData)]
pub struct GroundSystemData<'a> {
    entities: Entities<'a>,
    game_play: ReadExpect<'a, GamePlay>,
    ground_data: ReadExpect<'a, GroundData>,
    grounds_storage: WriteStorage<'a, Ground>,
    drawables_storage: WriteStorage<'a, Drawable>,
}

impl<'a> System<'a> for GroundSystem {
    type SystemData = GroundSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        let mut max_right: Option<i32> = None;

        for (entity, _, mut drawable) in (
            &data.entities,
            &data.grounds_storage,
            &mut data.drawables_storage,
        )
            .join()
        {
            for _ in 0..data.game_play.ticks_to_animate() {
                self.move_or_remove(&data.entities, entity, *data.ground_data, &mut drawable);
            }

            // Track data to figure out ground tiles to create. Make sure to do this
            // after all moves for a tile is done to prevent gaps between tiles. Otherwise
            // max_right could have an old, wrong value (in case of animating 2 ticks at
            // a time)
            let max_right_candidate = drawable.world_bounds.right();
            max_right = match max_right {
                None => Some(max_right_candidate),
                Some(x) => Some(x.max(max_right_candidate)),
            };
        }

        // May be there were no ticks to animate! Safely get max_right & then add
        // more ground tiles if required
        if let Some(max_right) = max_right {
            if max_right < self.world_data.bounds().right() {
                crate::entities::GroundEntity::create_all_world_tiles_start_at_world_x(
                    max_right,
                    &self.world_data,
                    &data.entities,
                    &mut data.drawables_storage,
                    &mut data.grounds_storage,
                );
            }
        }
    }
}
