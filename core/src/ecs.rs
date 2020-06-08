use crate::components;
use crate::data::enemy_data::EnemyData;
use crate::data::{CloudData, GroundData, PlayerData, WorldData};
use crate::entities::{GroundEntity, IconEntity, PlayerEntity, ScoreEntity};
use crate::resources::{EventQueue, GamePlay};
use crate::systems::{
    CloudSystem, CollisionSystem, EnemySystem, EventSystem, GamePlayTickUpdater, GameSpeedUpdater,
    GroundSystem, PlayerSystem, ScoreSystem,
};
use specs::{Dispatcher, DispatcherBuilder, World, WorldExt};

pub struct Ecs<'a, 'b> {
    pub world: World,
    world_data: WorldData,
    dispatcher: Option<Dispatcher<'a, 'b>>,
}

impl<'a, 'b> Ecs<'a, 'b> {
    pub fn dispatch(&mut self) {
        if let Some(ref mut dispatcher) = &mut self.dispatcher {
            dispatcher.dispatch(&self.world);
            self.world.maintain();
        }
    }

    pub fn setup(world_data: WorldData) -> Ecs<'a, 'b> {
        let mut world = World::new();

        // Insert resources
        let ground_data = Ecs::build_default_ground_data();
        world.insert(CloudData::new(world_data, ground_data));
        world.insert(EnemyData::new(world_data, ground_data));
        world.insert(PlayerData::new());
        world.insert(ground_data);
        world.insert(EventQueue::new());
        world.insert(GamePlay::new());

        // Register components
        world.register::<components::Animatable>();
        world.register::<components::Cloud>();
        world.register::<components::Drawable>();
        world.register::<components::Enemy>();
        world.register::<components::Ground>();
        world.register::<components::Icon>();
        world.register::<components::input::InputControlled>();
        world.register::<components::player::Player>();
        world.register::<components::score::Score>();

        // Create entities
        GroundEntity::create_all_tiles(&mut world, &world_data);
        PlayerEntity::create(&mut world, &world_data);
        ScoreEntity::create_all_tiles(&mut world, &world_data);

        Ecs {
            world,
            world_data,
            dispatcher: None,
        }
    }

    pub fn show_instructions(&mut self) {
        IconEntity::create_direction_tiles_at_world_center(&mut self.world);
    }

    pub fn show_game_end(&mut self) {
        IconEntity::create_retry_tile_at_world_center(&mut self.world);
    }

    pub fn start_game_play(&mut self) {
        // Remove everything that was added for instructional purposes
        IconEntity::remove_all_tiles(&mut self.world);

        // Orchestrate systems for game play
        let game_play_tick_updater = "game_play_tick_updater";
        let dispatcher = DispatcherBuilder::new()
            .with(GamePlayTickUpdater, game_play_tick_updater, &[])
            .with(EventSystem, "event_system", &[game_play_tick_updater])
            .with(
                GameSpeedUpdater::new(self.world_data),
                "game_speed_updater",
                &[],
            )
            .with_barrier() // To let event system & game updaters to work before any other systems
            .with(
                CloudSystem::new(Ecs::build_default_ground_data(), self.world_data),
                "cloud_system",
                &[],
            )
            .with(GroundSystem::new(self.world_data), "ground_system", &[])
            .with(PlayerSystem::new(self.world_data), "player_system", &[])
            .with(EnemySystem::new(self.world_data), "enemy_system", &[])
            .with(ScoreSystem, "score_system", &[])
            .with_barrier()
            .with(CollisionSystem, "collision_system", &[])
            .build();

        self.dispatcher = Some(dispatcher)
    }

    fn build_default_ground_data() -> GroundData {
        GroundData::new(1.0)
    }
}
