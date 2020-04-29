use crate::components;
use crate::components::{Animatable, Drawable};
use crate::data::WorldData;
use crate::graphics::data;
use crate::graphics::data::EnemyTile;
use sdl2::rect::Rect;
use specs::{Entities, WriteStorage};
use std::convert::TryFrom;

pub struct Enemy;

impl Enemy {
    pub fn create(
        world_data: &WorldData,
        tile: data::EnemyTile,
        entities: &Entities,
        animatables_storage: &mut WriteStorage<Animatable>,
        drawables_storage: &mut WriteStorage<Drawable>,
        enemies_storage: &mut WriteStorage<components::Enemy>,
    ) {
        let world_bottom = match tile {
            EnemyTile::BatFly1 | EnemyTile::BatFly2 => 0,
            EnemyTile::BeeFly1 | EnemyTile::BeeFly2 => 0,
            EnemyTile::MouseRun1 | EnemyTile::MouseRun2 => world_data.world_surface_at(),
            EnemyTile::SpiderRun1 | EnemyTile::SpiderRun2 => world_data.world_surface_at(),
        };

        entities
            .build_entity()
            .with(components::Enemy, enemies_storage)
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
                    world_bottom,
                ),
                drawables_storage,
            )
            .build();
    }

    pub fn build_drawable_with_left_bottom(
        tile: data::EnemyTile,
        world_left: i32,
        world_bottom: i32,
    ) -> Drawable {
        let tile_data = data::build_tile_data(data::Tile::Enemy { tile });
        let height_in_world = tile_data.bounds_in_tile_sheet.height() / 2;
        components::Drawable {
            tile_data,
            world_bounds: Rect::new(
                world_left,
                world_bottom - i32::try_from(height_in_world).expect("u32 too big for i32"),
                tile_data.bounds_in_tile_sheet.width() / 2,
                height_in_world,
            ),
        }
    }
}
