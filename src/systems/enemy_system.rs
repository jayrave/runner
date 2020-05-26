use crate::components::enemy::data::Position;
use crate::components::Enemy;
use crate::components::{Animatable, Drawable};
use crate::data::enemy_data::EnemyData;
use crate::data::{PlayerData, WorldData};
use crate::entities;
use crate::entities::EnemyEntity;
use crate::graphics::data;
use crate::graphics::data::EnemyTile;
use crate::resources::GamePlay;
use rand::Rng;
use specs::join::Join;
use specs::shred::ResourceId;
use specs::World;
use specs::{Entities, Entity, SystemData};
use specs::{ReadExpect, System, WriteStorage};

pub struct EnemySystem {
    world_data: WorldData,
    enemy_wave_started_at_tick: u64,
    enemies_spawned_in_current_wave: u8,
    most_recent_enemy_spawned_at_tick: u64,
}

impl EnemySystem {
    pub fn new(world_data: WorldData) -> EnemySystem {
        EnemySystem {
            world_data,
            enemy_wave_started_at_tick: 0,
            enemies_spawned_in_current_wave: 0,
            most_recent_enemy_spawned_at_tick: 0,
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

                    *drawable = EnemyEntity::build_drawable_with_right_bottom(
                        next_tile,
                        drawable.world_bounds.right(),
                        drawable.world_bounds.bottom(),
                    )
                }
            }
        }
    }

    fn should_spawn_enemy(&mut self, current_tick: u64, enemy_data: &EnemyData) -> bool {
        // Start new wave if required
        if current_tick - self.enemy_wave_started_at_tick > enemy_data.enemy_wave_ticks_count.into()
        {
            self.enemy_wave_started_at_tick = current_tick;
            self.enemies_spawned_in_current_wave = 0;
        }

        let enemies_remaining_in_wave = enemy_data.enemy_count_in_wave
            - self
                .enemies_spawned_in_current_wave
                .min(enemy_data.enemy_count_in_wave);

        let ticks_since_last_enemy = current_tick - self.most_recent_enemy_spawned_at_tick;
        if enemies_remaining_in_wave <= 0
            || ticks_since_last_enemy < u64::from(enemy_data.min_ticks_between_enemies)
        {
            false
        } else {
            let ticks_remaining_in_wave: u64 = (self.enemy_wave_started_at_tick
                + enemy_data.enemy_wave_ticks_count as u64)
                - current_tick;

            // Would panic if low == high. Hence `max(1)` for upper bound
            let spawn_enemy = rand::thread_rng().gen_range(
                0,
                (ticks_remaining_in_wave / enemies_remaining_in_wave as u64).max(1),
            ) == 0;

            if spawn_enemy {
                self.enemies_spawned_in_current_wave += 1;
                self.most_recent_enemy_spawned_at_tick = current_tick;
            }

            spawn_enemy
        }
    }

    fn get_random_enemy_tile(
        can_create_low_enemy: bool,
        can_create_mid_enemy: bool,
        can_create_high_enemy: bool,
    ) -> EnemyTile {
        loop {
            let tile = match rand::thread_rng().gen_range(1, 6) {
                1 => EnemyTile::BatFly1,
                2 => EnemyTile::BeeFly1,
                3 => EnemyTile::BugRun1,
                4 => EnemyTile::MouseRun1,
                _ => EnemyTile::SpiderRun1,
            };

            // Look at the call site for reasons behind why we have these flags
            // to control which enemy we spawn
            let can_create_such_enemy = match EnemyEntity::get_enemy_position(tile) {
                Position::Low => can_create_low_enemy,
                Position::Mid => can_create_mid_enemy,
                Position::High => can_create_high_enemy,
            };

            if can_create_such_enemy {
                return tile;
            }
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
    game_play: ReadExpect<'a, GamePlay>,
}

impl<'a> System<'a> for EnemySystem {
    type SystemData = EnemySystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        let mut has_low_enemies = false;
        let mut has_mid_enemies = false;
        let mut has_high_enemies = false;

        // animate/remove existing enemies
        for (enemy, entity, mut animatable, mut drawable) in (
            &data.enemies_storage,
            &data.entities,
            &mut data.animatables_storage,
            &mut data.drawables_storage,
        )
            .join()
        {
            match enemy.position {
                Position::Low => has_low_enemies = true,
                Position::Mid => has_mid_enemies = true,
                Position::High => has_high_enemies = true,
            };

            let start_tick = data.game_play.ticks_animated();
            let end_tick = start_tick + data.game_play.ticks_to_animate();
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
        if self.should_spawn_enemy(data.game_play.ticks_animated(), &data.enemy_data) {
            // Why do we use these flags? => To make sure that at any given
            // time, enemies are present only in 2 height positions. Otherwise,
            // there could be situation where the user can't slide or jump
            // through them. I think that even just allowing 2 enemies isn't going
            // to solve this problem but should reduce the possibility quite a bit
            let enemy_tile = EnemySystem::get_random_enemy_tile(
                !has_mid_enemies || !has_high_enemies,
                !has_low_enemies || !has_high_enemies,
                !has_low_enemies || !has_mid_enemies,
            );

            EnemyEntity::create(
                &data.enemy_data,
                &data.player_data,
                &self.world_data,
                enemy_tile,
                &data.entities,
                &mut data.animatables_storage,
                &mut data.drawables_storage,
                &mut data.enemies_storage,
            )
        }
    }
}
