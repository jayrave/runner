use crate::components;
use crate::components::{Animatable, Drawable};
use crate::data::enemy_data::{Animation, EnemyData};
use crate::data::{PlayerData, WorldData};
use crate::graphics::data;
use crate::graphics::data::EnemyTile;
use sdl2::rect::Rect;
use specs::{Entities, WriteStorage};
use std::convert::TryFrom;

const HIGH_ENEMY_MULTIPLIER: f32 = 0.9;
const MID_ENEMY_MULTIPLIER: f32 = 0.45;

const TILE_TO_WORLD_DIVIDER_BAT: f32 = 2.0;
const TILE_TO_WORLD_DIVIDER_BEE: f32 = 2.5;
const TILE_TO_WORLD_DIVIDER_BUG: f32 = 3.5;
const TILE_TO_WORLD_DIVIDER_MOUSE: f32 = 2.5;
const TILE_TO_WORLD_DIVIDER_SPIDER: f32 = 3.0;

pub struct Enemy;

impl Enemy {
    pub fn create(
        enemy_data: &EnemyData,
        player_data: &PlayerData,
        world_data: &WorldData,
        tile: data::EnemyTile,
        entities: &Entities,
        animatables_storage: &mut WriteStorage<Animatable>,
        drawables_storage: &mut WriteStorage<Drawable>,
        enemies_storage: &mut WriteStorage<components::Enemy>,
    ) {
        let animation: Animation;
        let tile_world_bottom: i32;
        match tile {
            EnemyTile::BatFly1 | EnemyTile::BatFly2 => {
                animation = enemy_data.bat_animation;
                tile_world_bottom = world_data.world_surface_at()
                    - (player_data.max_jump_height_in_wc as f32 * HIGH_ENEMY_MULTIPLIER) as i32;
            }
            EnemyTile::BeeFly1 | EnemyTile::BeeFly2 => {
                animation = enemy_data.bee_animation;
                tile_world_bottom = world_data.world_surface_at()
                    - (player_data.max_jump_height_in_wc as f32 * MID_ENEMY_MULTIPLIER) as i32;
            }
            EnemyTile::BugRun1 | EnemyTile::BugRun2 => {
                animation = enemy_data.bug_animation;
                tile_world_bottom = world_data.world_surface_at();
            }
            EnemyTile::MouseRun1 | EnemyTile::MouseRun2 => {
                animation = enemy_data.mouse_animation;
                tile_world_bottom = world_data.world_surface_at();
            }
            EnemyTile::SpiderRun1 | EnemyTile::SpiderRun2 => {
                animation = enemy_data.spider_animation;
                tile_world_bottom = world_data.world_surface_at();
            }
        };

        entities
            .build_entity()
            .with(
                components::Enemy::new(animation.speed_in_wc_per_tick, animation.ticks_in_movement),
                enemies_storage,
            )
            .with(
                components::Animatable {
                    current_step_started_at_tick: 0,
                },
                animatables_storage,
            )
            .with(
                Enemy::build_drawable_with_left_bottom(
                    tile,
                    world_data.bounds().right(),
                    tile_world_bottom,
                ),
                drawables_storage,
            )
            .build();
    }

    /// Instead of (left, bottom) like we do for player, we are taking
    /// in (right, bottom) because enemies travel right to left
    pub fn build_drawable_with_right_bottom(
        tile: data::EnemyTile,
        world_right: i32,
        world_bottom: i32,
    ) -> Drawable {
        let tile_data = data::build_tile_data(data::Tile::Enemy { tile });
        let (width_in_world, height_in_world) =
            Enemy::build_world_bounds(tile, &tile_data.bounds_in_tile_sheet);

        components::Drawable {
            tile_data,
            world_bounds: Rect::new(
                world_right - i32::try_from(width_in_world).expect("u32 too big for i32"),
                world_bottom - i32::try_from(height_in_world).expect("u32 too big for i32"),
                width_in_world,
                height_in_world,
            ),
        }
    }

    fn build_drawable_with_left_bottom(
        tile: data::EnemyTile,
        world_left: i32,
        world_bottom: i32,
    ) -> Drawable {
        // To prevent logic repetition, we will create a drawable with right & offset as required
        let mut drawable = Enemy::build_drawable_with_right_bottom(tile, world_left, world_bottom);
        drawable
            .world_bounds
            .offset(drawable.world_bounds.width() as i32, 0);
        drawable
    }

    fn build_world_bounds(tile: data::EnemyTile, tile_bounds: &Rect) -> (u32, u32) {
        let divider = match tile {
            EnemyTile::BatFly1 | EnemyTile::BatFly2 => TILE_TO_WORLD_DIVIDER_BAT,
            EnemyTile::BeeFly1 | EnemyTile::BeeFly2 => TILE_TO_WORLD_DIVIDER_BEE,
            EnemyTile::BugRun1 | EnemyTile::BugRun2 => TILE_TO_WORLD_DIVIDER_BUG,
            EnemyTile::MouseRun1 | EnemyTile::MouseRun2 => TILE_TO_WORLD_DIVIDER_MOUSE,
            EnemyTile::SpiderRun1 | EnemyTile::SpiderRun2 => TILE_TO_WORLD_DIVIDER_SPIDER,
        };

        (
            (tile_bounds.width() as f32 / divider) as u32,
            (tile_bounds.height() as f32 / divider) as u32,
        )
    }
}
