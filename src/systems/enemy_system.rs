use crate::components::Enemy;
use crate::components::{Animatable, Drawable};
use crate::data::enemy_data::EnemyData;
use crate::data::{PlayerData, WorldData};
use crate::entities;
use crate::graphics::data;
use crate::graphics::data::EnemyTile;
use crate::resources::GameTick;
use rand::Rng;
use specs::join::Join;
use specs::shred::ResourceId;
use specs::World;
use specs::{Entities, Entity, SystemData};
use specs::{ReadExpect, System, WriteStorage};

pub struct EnemySystem {
    world_data: WorldData,
    last_enemy_at_tick: u64,
}

impl EnemySystem {
    pub fn new(world_data: WorldData) -> EnemySystem {
        EnemySystem {
            world_data,
            last_enemy_at_tick: 0,
        }
    }

    fn move_or_remove(
        &self,
        current_tick: u64,
        entities: &Entities,
        entity: Entity,
        enemy: &Enemy,
        animatable: &mut Animatable,
        drawable: &mut Drawable,
    ) {
        // As soon as an enemy moves out, let it go
        if drawable.world_bounds.right() <= self.world_data.bounds().left() {
            entities
                .delete(entity)
                .expect("Enemy entity couldn't be deleted");
        } else {
            // For every tick, the enemy should move at least a bit
            drawable
                .world_bounds
                .offset(-i32::from(enemy.speed_in_wc_per_tick), 0);

            // Each step should last for a few ticks
            let move_to_next_animation_frame = animatable.current_step_started_at_tick
                + u64::from(enemy.ticks_in_movement)
                <= current_tick;

            if move_to_next_animation_frame {
                animatable.current_step_started_at_tick = current_tick;
                if let data::Tile::Enemy { tile } = drawable.tile_data.tile {
                    let next_tile = match tile {
                        EnemyTile::BatFly1 => EnemyTile::BatFly2,
                        EnemyTile::BatFly2 => EnemyTile::BatFly1,
                        EnemyTile::BeeFly1 => EnemyTile::BeeFly2,
                        EnemyTile::BeeFly2 => EnemyTile::BeeFly1,
                        EnemyTile::BugRun1 => EnemyTile::BugRun2,
                        EnemyTile::BugRun2 => EnemyTile::BugRun1,
                        EnemyTile::MouseRun1 => EnemyTile::MouseRun2,
                        EnemyTile::MouseRun2 => EnemyTile::MouseRun1,
                        EnemyTile::SpiderRun1 => EnemyTile::SpiderRun2,
                        EnemyTile::SpiderRun2 => EnemyTile::SpiderRun1,
                    };

                    *drawable = entities::Enemy::build_drawable_with_right_bottom(
                        next_tile,
                        drawable.world_bounds.right(),
                        drawable.world_bounds.bottom(),
                    )
                }
            }
        }
    }

    fn can_create_new_enemy(&mut self, game_tick: &GameTick, enemy_data: &EnemyData) -> bool {
        let ticks_animated = game_tick.ticks_animated();
        let ticks_since_last_enemy = ticks_animated - self.last_enemy_at_tick;
        let create_enemy = ticks_since_last_enemy > enemy_data.min_ticks_between_enemies
            && rand::thread_rng().gen_range(0, enemy_data.randomness_factor) == 0;

        if create_enemy {
            self.last_enemy_at_tick = ticks_animated;
            true
        } else {
            false
        }
    }

    fn get_random_enemy_tile() -> data::EnemyTile {
        match rand::thread_rng().gen_range(1, 6) {
            1 => data::EnemyTile::BatFly1,
            2 => data::EnemyTile::BeeFly1,
            3 => data::EnemyTile::BugRun1,
            4 => data::EnemyTile::MouseRun1,
            _ => data::EnemyTile::SpiderRun1,
        }
    }
}

#[derive(SystemData)]
pub struct EnemySystemData<'a> {
    entities: Entities<'a>,
    enemy_data: ReadExpect<'a, EnemyData>,
    player_data: ReadExpect<'a, PlayerData>,
    animatables_storage: WriteStorage<'a, Animatable>,
    enemies_storage: WriteStorage<'a, Enemy>,
    drawables_storage: WriteStorage<'a, Drawable>,
    game_tick: ReadExpect<'a, GameTick>,
}

impl<'a> System<'a> for EnemySystem {
    type SystemData = EnemySystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        // animate/remove existing enemies
        for (enemy, entity, mut animatable, mut drawable) in (
            &data.enemies_storage,
            &data.entities,
            &mut data.animatables_storage,
            &mut data.drawables_storage,
        )
            .join()
        {
            let start_tick = data.game_tick.ticks_animated();
            let end_tick = start_tick + data.game_tick.ticks_to_animate();
            for current_tick in start_tick..end_tick {
                self.move_or_remove(
                    current_tick,
                    &data.entities,
                    entity,
                    &enemy,
                    &mut animatable,
                    &mut drawable,
                )
            }
        }

        // Create new enemies if possible & required
        if self.can_create_new_enemy(&data.game_tick, &data.enemy_data) {
            entities::Enemy::create(
                &data.enemy_data,
                &data.player_data,
                &self.world_data,
                EnemySystem::get_random_enemy_tile(),
                &data.entities,
                &mut data.animatables_storage,
                &mut data.drawables_storage,
                &mut data.enemies_storage,
            )
        }
    }
}
