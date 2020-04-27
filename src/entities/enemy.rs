use crate::components;
use crate::components::{Animatable, Drawable};
use crate::data::WorldData;
use crate::graphics::data;
use crate::graphics::data::EnemyTile;
use sdl2::rect::Rect;
use specs::{Entities, WriteStorage};

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
        let world_y = match tile {
            EnemyTile::BatFly1 | EnemyTile::BatFly2 => 0,
            EnemyTile::BeeFly1 | EnemyTile::BeeFly2 => 0,
            EnemyTile::MouseRun1 | EnemyTile::MouseRun2 => 0,
            EnemyTile::SpiderRun1 | EnemyTile::SpiderRun2 => 0,
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
                Enemy::build_drawable(tile, world_data.world_right(), world_y),
                drawables_storage,
            )
            .build();
    }

    pub fn build_drawable(tile: data::EnemyTile, world_x: i32, world_y: i32) -> Drawable {
        let tile_data = data::build_tile_data(data::Tile::Enemy { tile });
        components::Drawable {
            tile_data,
            world_bounds: Rect::new(
                world_x,
                world_y,
                tile_data.bounds_in_tile_sheet.width() / 2,
                tile_data.bounds_in_tile_sheet.height() / 2,
            ),
        }
    }
}
