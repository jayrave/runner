use crate::components;
use crate::data::enemy_data::EnemyData;
use crate::data::{CloudData, GroundData, PlayerData, WorldData};
use crate::entities::{GroundEntity, PlayerEntity};
use crate::resources::{EventQueue, GamePlay, Score};
use crate::systems::{
    CloudSystem, CollisionSystem, EnemySystem, EventSystem, GamePlayTickUpdater, GameSpeedUpdater,
    GroundSystem, PlayerSystem, ScoreUpdater,
};
use specs::{Dispatcher, DispatcherBuilder, World, WorldExt};

pub struct Ecs<'a, 'b> {
    pub world: World,
    pub dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> Ecs<'a, 'b> {
    pub fn dispatch(&mut self) {
        self.dispatcher.dispatch(&self.world);
        self.world.maintain();
    }

    pub fn setup(world_data: WorldData) -> Ecs<'a, 'b> {
        let mut world = World::new();

        // Insert resources
        let ground_data = GroundData::new(1.0);
        world.insert(CloudData::new(world_data, ground_data));
        world.insert(EnemyData::new(world_data, ground_data));
        world.insert(PlayerData::new());
        world.insert(ground_data);
        world.insert(EventQueue::new());
        world.insert(GamePlay::new());
        world.insert(Score::new());

        // Register components
        world.register::<components::Animatable>();
        world.register::<components::Cloud>();
        world.register::<components::Drawable>();
        world.register::<components::Enemy>();
        world.register::<components::Ground>();
        world.register::<components::player::Player>();
        world.register::<components::input::InputControlled>();

        // Create entities
        GroundEntity::create_all_tiles(&mut world, &world_data);
        PlayerEntity::create(&mut world, &world_data);

        // Orchestrate systems
        let game_play_tick_updater = "game_play_tick_updater";
        let dispatcher = DispatcherBuilder::new()
            .with(GamePlayTickUpdater, game_play_tick_updater, &[])
            .with(EventSystem, "event_system", &[game_play_tick_updater])
            .with(GameSpeedUpdater::new(world_data), "game_speed_updater", &[])
            .with_barrier() // To let event system & game updaters to work before any other systems
            .with(
                CloudSystem::new(ground_data, world_data),
                "cloud_system",
                &[],
            )
            .with(GroundSystem::new(world_data), "ground_system", &[])
            .with(PlayerSystem::new(world_data), "player_system", &[])
            .with(EnemySystem::new(world_data), "enemy_system", &[])
            .with(ScoreUpdater, "score_updater", &[])
            .with_barrier()
            .with(CollisionSystem, "collision_system", &[])
            .build();

        Ecs { world, dispatcher }
    }
}
