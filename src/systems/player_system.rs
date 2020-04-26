use crate::components;
use crate::components::input::InputControlled;
use crate::components::Drawable;
use crate::components::Player;
use crate::entities;

use crate::graphics::data;
use crate::resources::GameTick;

use crate::data::{AnimationData, WorldData};
use specs::join::Join;
use specs::shred::ResourceId;
use specs::SystemData;
use specs::World;
use specs::{ReadExpect, System, WriteStorage};

pub struct PlayerSystem {
    animation_data: AnimationData,
    world_data: WorldData,
    jump_gravity: f32,
    jump_velocity: f32,
}

impl PlayerSystem {
    pub fn new(animation_data: AnimationData, world_data: WorldData) -> PlayerSystem {
        // Derived using math from a GDC talk (for smooth parabolic jump):
        //      GDC link: https://www.gdcvault.com/play/1023559/Math-for-Game-Programmers-Building
        //      Video: https://www.youtube.com/watch?v=hG9SzQxaCm8
        //      Slides: http://www.mathforgameprogrammers.com/gdc2016/GDC2016_Pittman_Kyle_BuildingABetterJump.pdf

        let ticks_to_hit_apex_in_jump: f32 = animation_data.ticks_in_player_jump() as f32 / 2.0;
        let jump_gravity: f32 = (-2.0 * animation_data.player_jump_height_in_wc() as f32)
            / (ticks_to_hit_apex_in_jump.powf(2.0));

        let jump_velocity: f32 = -jump_gravity * ticks_to_hit_apex_in_jump;

        PlayerSystem {
            animation_data,
            world_data,
            jump_gravity,
            jump_velocity,
        }
    }

    fn update(
        &self,
        current_tick: u64,
        drawable: &mut Drawable,
        player: &mut Player,
        input_controlled: &mut InputControlled,
    ) {
        let tile = drawable.tile_data.tile;
        let current_step_started_at_tick = player.current_step_started_at_tick;

        if let data::Tile::Character { tile } = tile {
            match tile {
                // Already an input based animation is going on. Transfer the control
                // over to that to either carry on the animation or to finish it. When
                // an animation is going on, we don't care about more inputs from the
                // user
                data::CharacterTile::Jump => self.continue_jump_or_start_running(
                    current_tick,
                    current_step_started_at_tick,
                    drawable,
                    player,
                ),

                // Same comment as jump applies here too
                data::CharacterTile::Slide => self.continue_slide_or_start_running(
                    current_tick,
                    current_step_started_at_tick,
                    drawable,
                    player,
                ),

                // No input based animation going on. Gotta check if we should start
                // one now
                _ => match input_controlled.consume_input() {
                    // There is some input! Start a new input based animation
                    Some(input) => match input {
                        // Gotta jump!
                        components::input::data::Input::Up => {
                            self.start_jump(current_tick, drawable, player)
                        }

                        // Gotta slide!
                        components::input::data::Input::Down => {
                            self.start_slide(current_tick, drawable, player)
                        }
                    },

                    // Nothing else to do! Just continue running
                    None => self.continue_run(
                        current_tick,
                        current_step_started_at_tick,
                        drawable,
                        player,
                        tile,
                    ),
                },
            }
        }

        // If the input isn't consumed because there is another animation
        // already running, we don't want the input to be left around to be
        // used later! So, no matter what happens at the end of each tick,
        // clear the user input to allow taking in more
        input_controlled.consume_input();
    }

    fn start_slide(&self, current_tick: u64, drawable: &mut Drawable, player: &mut Player) {
        player.current_step_started_at_tick = current_tick;
        self.update_drawable_for_surface_level_tile(drawable, data::CharacterTile::Slide);
    }

    fn continue_slide_or_start_running(
        &self,
        current_tick: u64,
        slide_started_at_tick: u64,
        drawable: &mut Drawable,
        player: &mut Player,
    ) {
        let continue_slide = slide_started_at_tick
            + u64::from(self.animation_data.ticks_in_player_slide())
            >= current_tick;

        // If we are just continuing to slide, no need to update any drawable
        // data. Otherwise, will have to switch to running
        if !continue_slide {
            self.start_run(current_tick, drawable, player)
        }
    }

    fn start_jump(&self, current_tick: u64, drawable: &mut Drawable, player: &mut Player) {
        player.current_step_started_at_tick = current_tick;
        self.update_drawable_for_jump_tile(current_tick, current_tick, drawable);
    }

    fn continue_jump_or_start_running(
        &self,
        current_tick: u64,
        jump_started_at_tick: u64,
        drawable: &mut Drawable,
        player: &mut Player,
    ) {
        let continue_jump = jump_started_at_tick
            + u64::from(self.animation_data.ticks_in_player_jump())
            >= current_tick;

        if continue_jump {
            self.update_drawable_for_jump_tile(current_tick, jump_started_at_tick, drawable)
        } else {
            self.start_run(current_tick, drawable, player)
        }
    }

    fn start_run(&self, current_tick: u64, drawable: &mut Drawable, player: &mut Player) {
        player.current_step_started_at_tick = current_tick;
        self.update_drawable_for_surface_level_tile(drawable, data::CharacterTile::Run1);
    }

    fn continue_run(
        &self,
        current_tick: u64,
        run_started_at_tick: u64,
        drawable: &mut Drawable,
        player: &mut Player,
        current_tile: data::CharacterTile,
    ) {
        let change_tile = run_started_at_tick
            + u64::from(self.animation_data.ticks_in_player_run_step())
            <= current_tick;

        if change_tile {
            player.current_step_started_at_tick = current_tick;
            self.update_drawable_for_surface_level_tile(
                drawable,
                match current_tile {
                    data::CharacterTile::Run1 => data::CharacterTile::Run2,
                    data::CharacterTile::Run2 => data::CharacterTile::Run3,
                    data::CharacterTile::Run3 => data::CharacterTile::Run1,
                    _ => data::CharacterTile::Run1, //  Fallback
                },
            )
        }
    }

    fn update_drawable_for_surface_level_tile(
        &self,
        drawable: &mut Drawable,
        tile: data::CharacterTile,
    ) {
        drawable.tile_data = data::build_tile_data(data::Tile::Character { tile });
        drawable
            .world_bounds
            .set_y(entities::Player::running_y(&self.world_data));
    }

    fn update_drawable_for_jump_tile(
        &self,
        current_tick: u64,
        jump_started_at_tick: u64,
        drawable: &mut Drawable,
    ) {
        drawable.tile_data = data::build_tile_data(data::Tile::Character {
            tile: data::CharacterTile::Jump,
        });

        let ticks_since_jump_started = (current_tick - jump_started_at_tick) as f32;
        let height = ((self.jump_gravity * ticks_since_jump_started.powf(2.0)) / 2.0)
            + (self.jump_velocity * ticks_since_jump_started);

        drawable
            .world_bounds
            .set_y(entities::Player::running_y(&self.world_data) - height.round() as i32);
    }
}

#[derive(SystemData)]
pub struct PlayerSystemData<'a> {
    game_tick: ReadExpect<'a, GameTick>,
    drawables_storage: WriteStorage<'a, Drawable>,
    players_storage: WriteStorage<'a, Player>,
    input_controlled_storage: WriteStorage<'a, InputControlled>,
}

impl<'a> System<'a> for PlayerSystem {
    type SystemData = PlayerSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        for (mut drawable, mut player, mut input_controlled) in (
            &mut data.drawables_storage,
            &mut data.players_storage,
            &mut data.input_controlled_storage,
        )
            .join()
        {
            let start_tick = data.game_tick.ticks_animated();
            let end_tick = start_tick + data.game_tick.ticks_to_animate();
            for current_tick in start_tick..end_tick {
                self.update(
                    current_tick,
                    &mut drawable,
                    &mut player,
                    &mut input_controlled,
                )
            }
        }
    }
}
