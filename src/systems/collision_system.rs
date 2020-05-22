use crate::components::player::Player;
use crate::components::{Drawable, Enemy};

use crate::resources::GamePlayTick;
use specs::join::Join;
use specs::shred::ResourceId;
use specs::{ReadExpect, System};
use specs::{ReadStorage, SystemData};
use specs::{World, WriteStorage};

pub struct CollisionSystem;

#[derive(SystemData)]
pub struct CollisionSystemData<'a> {
    game_play_tick: ReadExpect<'a, GamePlayTick>,
    enemies_storage: ReadStorage<'a, Enemy>,
    players_storage: WriteStorage<'a, Player>,
    drawables_storage: ReadStorage<'a, Drawable>,
}

impl<'a> System<'a> for CollisionSystem {
    type SystemData = CollisionSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        if data.game_play_tick.ticked() {
            for (player_drawable, mut player) in
                (&data.drawables_storage, &mut data.players_storage).join()
            {
                for (enemy_drawable, _) in (&data.drawables_storage, &data.enemies_storage).join() {
                    if player_drawable
                        .world_bounds
                        .has_intersection(enemy_drawable.world_bounds)
                    {
                        player.is_hit = true;
                        return;
                    }
                }
            }
        }
    }
}
