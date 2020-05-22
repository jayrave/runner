use crate::components;
use crate::data;
use crate::entities;
use crate::resources;
use crate::{systems, WorldData};
use specs::{Dispatcher, DispatcherBuilder, World, WorldExt};

pub struct GameWorld<'a, 'b> {
    pub world: World,
    pub dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> GameWorld<'a, 'b> {
    pub fn dispatch(&mut self) {
        self.dispatcher.dispatch(&self.world);
        self.world.maintain();
    }

    pub fn setup(world_data: WorldData) -> GameWorld<'a, 'b> {
        let mut world = World::new();

        // Insert resources
        let ground_data = data::GroundData::new(1.0);
        world.insert(data::enemy_data::EnemyData::new(ground_data));
        world.insert(data::PlayerData::new());
        world.insert(ground_data);
        world.insert(resources::EventQueue::new());
        world.insert(resources::GamePlay::new());
        world.insert(resources::GamePlayTick::new());

        // Register components
        world.register::<components::Animatable>();
        world.register::<components::Drawable>();
        world.register::<components::Enemy>();
        world.register::<components::Ground>();
        world.register::<components::player::Player>();
        world.register::<components::input::InputControlled>();

        // Create entities
        entities::Ground::create_all_tiles(&mut world, &world_data);
        entities::Player::create(&mut world, &world_data);

        // Orchestrate systems
        let game_play_tick_updater = "game_play_tick_updater";
        let dispatcher = DispatcherBuilder::new()
            .with(systems::GamePlayTickUpdater, game_play_tick_updater, &[])
            .with(
                systems::EventSystem,
                "event_system",
                &[game_play_tick_updater],
            )
            .with(systems::GameSpeedUpdater::new(), "game_speed_updater", &[])
            .with_barrier() // To let event system & game updaters to work before any other systems
            .with(systems::GroundSystem::new(world_data), "ground_system", &[])
            .with(systems::PlayerSystem::new(world_data), "player_system", &[])
            .with(systems::EnemySystem::new(world_data), "enemy_system", &[])
            .with_barrier()
            .with(systems::CollisionSystem, "collision_system", &[])
            .build();

        GameWorld { world, dispatcher }
    }
}
