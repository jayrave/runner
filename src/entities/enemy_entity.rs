use crate::components;
use crate::components::enemy::data::Position;
use crate::components::{Animatable, Drawable, Enemy};
use crate::data::enemy_data::EnemyData;
use crate::data::{PlayerData, WorldData};
use crate::entities::PlayerEntity;
use crate::graphics::data;
use crate::graphics::data::EnemyTile;
use sdl2::rect::Rect;
use specs::{Entities, WriteStorage};
use std::convert::TryFrom;

const TILE_TO_WORLD_DIVIDER_BAT: f32 = 2.0;
const TILE_TO_WORLD_DIVIDER_BEE: f32 = 2.5;
const TILE_TO_WORLD_DIVIDER_BUG: f32 = 3.5;
const TILE_TO_WORLD_DIVIDER_MOUSE: f32 = 2.5;
const TILE_TO_WORLD_DIVIDER_SPIDER: f32 = 3.0;

pub struct EnemyEntity;

impl EnemyEntity {
    pub fn create(
        enemy_data: &EnemyData,
        player_data: &PlayerData,
        world_data: &WorldData,
        tile: EnemyTile,
        entities: &Entities,
        animatables_storage: &mut WriteStorage<Animatable>,
        drawables_storage: &mut WriteStorage<Drawable>,
        enemies_storage: &mut WriteStorage<components::Enemy>,
    ) {
        let animation = match tile {
            EnemyTile::BatFly1 | EnemyTile::BatFly2 => enemy_data.bat_animation,
            EnemyTile::BeeFly1 | EnemyTile::BeeFly2 => enemy_data.bee_animation,
            EnemyTile::BugRun1 | EnemyTile::BugRun2 => enemy_data.bug_animation,
            EnemyTile::MouseRun1 | EnemyTile::MouseRun2 => enemy_data.mouse_animation,
            EnemyTile::SpiderRun1 | EnemyTile::SpiderRun2 => enemy_data.spider_animation,
        };

        let position = EnemyEntity::get_enemy_position(tile);
        let tile_world_bottom = match position {
            Position::Low => world_data.world_surface_at(),
            // To force player to slide
            Position::Mid => PlayerEntity::top_when_sliding(world_data),
            // Offset later to make computations uniform
            Position::High => PlayerEntity::bottom_when_max_jumping(world_data, player_data),
        };

        let mut drawable = EnemyEntity::build_drawable_with_left_bottom(
            tile,
            world_data.bounds().right(),
            tile_world_bottom,
        );

        // For high enemy, we want to force enemy to not jump over it
        if position == Position::High {
            drawable
                .world_bounds
                .offset(0, (drawable.world_bounds.height() / 2) as i32)
        }

        entities
            .build_entity()
            .with(
                Enemy::new(
                    animation.speed_in_wc_per_tick,
                    animation.ticks_in_movement,
                    position,
                ),
                enemies_storage,
            )
            .with(
                Animatable {
                    current_step_started_at_tick: 0,
                },
                animatables_storage,
            )
            .with(drawable, drawables_storage)
            .build();
    }

    pub fn get_enemy_position(tile: EnemyTile) -> Position {
        match tile {
            EnemyTile::BatFly1 | EnemyTile::BatFly2 => Position::High,
            EnemyTile::BeeFly1 | EnemyTile::BeeFly2 => Position::Mid,
            EnemyTile::BugRun1 | EnemyTile::BugRun2 => Position::Low,
            EnemyTile::MouseRun1 | EnemyTile::MouseRun2 => Position::Low,
            EnemyTile::SpiderRun1 | EnemyTile::SpiderRun2 => Position::Low,
        }
    }

    /// Instead of (left, bottom) like we do for player, we are taking
    /// in (right, bottom) because enemies travel right to left
    pub fn build_drawable_with_right_bottom(
        tile: EnemyTile,
        world_right: i32,
        world_bottom: i32,
    ) -> Drawable {
        let tile_data = data::build_tile_data(data::Tile::Enemy { tile });
        let (width_in_world, height_in_world) =
            EnemyEntity::build_world_bounds(tile, &tile_data.bounds_in_tile_sheet);

        Drawable {
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
        tile: EnemyTile,
        world_left: i32,
        world_bottom: i32,
    ) -> Drawable {
        // To prevent logic repetition, we will create a drawable with right & offset as required
        let mut drawable =
            EnemyEntity::build_drawable_with_right_bottom(tile, world_left, world_bottom);
        drawable
            .world_bounds
            .offset(drawable.world_bounds.width() as i32, 0);
        drawable
    }

    fn build_world_bounds(tile: EnemyTile, tile_bounds: &Rect) -> (u32, u32) {
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
