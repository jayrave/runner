use crate::components::player::Player;
use crate::components::{Drawable, Enemy};

use crate::resources::{GamePlay, GameTick};
use specs::join::Join;
use specs::shred::ResourceId;
use specs::World;
use specs::WriteExpect;
use specs::{ReadExpect, System};
use specs::{ReadStorage, SystemData};

pub struct CollisionSystem;

#[derive(SystemData)]
pub struct CollisionSystemData<'a> {
    game_play: WriteExpect<'a, GamePlay>,
    game_tick: ReadExpect<'a, GameTick>,
    enemies_storage: ReadStorage<'a, Enemy>,
    players_storage: ReadStorage<'a, Player>,
    drawables_storage: ReadStorage<'a, Drawable>,
}

impl<'a> System<'a> for CollisionSystem {
    type SystemData = CollisionSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        if data.game_tick.ticked() {
            for (player_drawable, _) in (&data.drawables_storage, &data.players_storage).join() {
                for (enemy_drawable, _) in (&data.drawables_storage, &data.enemies_storage).join() {
                    if player_drawable
                        .world_bounds
                        .has_intersection(enemy_drawable.world_bounds)
                    {
                        data.game_play.mark_over();
                        return;
                    }
                }
            }
        }
    }
}
