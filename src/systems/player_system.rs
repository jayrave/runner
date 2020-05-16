use crate::components::input::InputControlled;
use crate::components::player::data as player_data;
use crate::components::player::Player;
use crate::components::{Animatable, Drawable};
use crate::entities;

use crate::graphics::data as graphics_data;
use crate::resources::GameTick;

use crate::data::{AnimationData, WorldData};
use specs::join::Join;
use specs::shred::ResourceId;
use specs::World;
use specs::{ReadExpect, System, WriteStorage};
use specs::{ReadStorage, SystemData};
use std::convert::TryFrom;
use crate::components::player::data::Action::Jump;

pub struct PlayerSystem {
    animation_data: AnimationData,
    world_data: WorldData,
    jump_physics: Option<JumpPhysics>,
}

struct JumpPhysics {
    start_at_tick: u64,
    initial_height: i32,
    initial_jump_velocity: f32,
    gravity: f32,
    is_lower_gravity: bool,
}

impl JumpPhysics {
    fn from_ground(current_tick: u64, animation_data: &AnimationData, world_data: &WorldData) -> JumpPhysics {
        let gravity = JumpPhysics::compute_gravity(animation_data);
        let initial_velocity = JumpPhysics::compute_initial_velocity(animation_data, gravity);
        JumpPhysics {
            start_at_tick: current_tick,
            initial_height: 0,
            initial_jump_velocity: initial_velocity,
            gravity,
            is_lower_gravity: true,
        }
    }

    fn compute_gravity(animation_data: &AnimationData) -> f32 {
        // Derived using math from a GDC talk (for smooth parabolic jump):
        //      GDC link: https://www.gdcvault.com/play/1023559/Math-for-Game-Programmers-Building
        //      Video: https://www.youtube.com/watch?v=hG9SzQxaCm8
        //      Slides: http://www.mathforgameprogrammers.com/gdc2016/GDC2016_Pittman_Kyle_BuildingABetterJump.pdf

        let ticks_to_hit_apex: f32 = animation_data.ticks_in_player_max_jump() as f32 / 2.0;
        (-2.0 * animation_data.player_jump_height_in_wc() as f32) / (ticks_to_hit_apex.powf(2.0))
    }

    fn compute_initial_velocity(animation_data: &AnimationData, gravity: f32) -> f32 {
        // Same method as described for gravity
        let ticks_to_hit_apex: f32 = animation_data.ticks_in_player_max_jump() as f32 / 2.0;
        -gravity * ticks_to_hit_apex
    }

    fn compute_height(&self, current_tick: u64) -> i32 {
        let ticks_since_jump_started = (current_tick - self.start_at_tick) as f32;
        (((self.gravity * ticks_since_jump_started.powf(2.0)) / 2.0)
            + (self.initial_jump_velocity * ticks_since_jump_started)) as i32 + self.initial_height
    }

    fn update_gravity_if_required(self, current_tick: u64, current_height: i32) -> JumpPhysics {
        if !self.is_lower_gravity {
            self
        } else {
            let ticks_since_jump_started = (current_tick - self.start_at_tick) as f32;
            let current_velocity = self.initial_jump_velocity + (self.gravity * ticks_since_jump_started);
            if current_velocity > 0.0 {
                self
            } else {
                JumpPhysics {
                    start_at_tick: current_tick,
                    initial_height: current_height,
                    initial_jump_velocity: 0.0,
                    gravity: self.gravity * 8.0,
                    is_lower_gravity: false,
                }
            }
        }
    }
}

impl PlayerSystem {
    pub fn new(animation_data: AnimationData, world_data: WorldData) -> PlayerSystem {
        PlayerSystem {
            animation_data,
            world_data,
            jump_physics: None
        }
    }

    fn input_to_action(input_ctrl: &InputControlled) -> Option<player_data::Action> {
        let mut input_action = None;
        // Order of precedence of actions:
        //  - Vertical & horizontal together
        //  - Only vertical
        //  - Only horizontal

        if input_action.is_none() && input_ctrl.up_engaged() && !input_ctrl.down_engaged() {
            input_action = Some(player_data::Action::Jump)
        }

        if input_action.is_none() && input_ctrl.down_engaged() && !input_ctrl.up_engaged() {
            input_action = Some(player_data::Action::Slide)
        }

        if input_action.is_none() && input_ctrl.right_engaged() && !input_ctrl.left_engaged() {
            input_action = Some(player_data::Action::FastRun)
        }

        if input_action.is_none() && input_ctrl.left_engaged() && !input_ctrl.right_engaged() {
            input_action = Some(player_data::Action::SlowRun)
        }

        input_action
    }

    fn update(
        &mut self,
        current_tick: u64,
        animatable: &mut Animatable,
        drawable: &mut Drawable,
        input_controlled: &InputControlled,
        player: &mut Player,
    ) {
        let tile = drawable.tile_data.tile;
        let current_step_started_at_tick = animatable.current_step_started_at_tick;

        match player.current_action {
            // Already an uninterruptible input based animation is going on. Transfer
            // the control over to that to either carry on the animation or to finish
            // it. We don't worry about new inputs at this point
            player_data::Action::Jump => self.continue_jump_or_start_running(
                current_tick,
                animatable,
                drawable,
                input_controlled,
                player,
            ),

            // Another uninterruptible animation
            player_data::Action::Slide => self.continue_slide_or_start_running(
                current_tick,
                current_step_started_at_tick,
                animatable,
                drawable,
                player,
            ),

            // No input based animation going on. Gotta check if we should start
            // one now
            player_data::Action::FastRun
            | player_data::Action::SlowRun
            | player_data::Action::Run => match PlayerSystem::input_to_action(input_controlled) {
                // Some new input based action to start
                Some(action) => match action {
                    player_data::Action::Jump => self.start_jump(
                        current_tick,
                        animatable,
                        drawable,
                        input_controlled,
                        player,
                    ),

                    player_data::Action::Slide => {
                        self.start_slide(current_tick, animatable, drawable, player)
                    }

                    player_data::Action::FastRun
                    | player_data::Action::SlowRun
                    | player_data::Action::Run => {
                        let offset_multiplier = match action {
                            player_data::Action::SlowRun => -1,
                            player_data::Action::FastRun => 1,
                            _ => 0,
                        };

                        let x_offset: i32 = offset_multiplier
                            * i32::from(
                                self.animation_data
                                    .player_extra_input_speed_in_wc_per_tick(),
                            );

                        self.continue_run(
                            current_tick,
                            current_step_started_at_tick,
                            x_offset,
                            animatable,
                            drawable,
                            player,
                            tile,
                        )
                    }
                },

                // Nothing else to do! Just continue running
                None => self.continue_run(
                    current_tick,
                    current_step_started_at_tick,
                    0,
                    animatable,
                    drawable,
                    player,
                    tile,
                ),
            },
        }
    }

    fn start_slide(
        &self,
        current_tick: u64,
        animatable: &mut Animatable,
        drawable: &mut Drawable,
        player: &mut Player,
    ) {
        animatable.current_step_started_at_tick = current_tick;
        player.current_action = player_data::Action::Slide;
        self.update_drawable_for_surface_level_tile(drawable, graphics_data::CharacterTile::Slide);
    }

    fn continue_slide_or_start_running(
        &self,
        current_tick: u64,
        slide_started_at_tick: u64,
        animatable: &mut Animatable,
        drawable: &mut Drawable,
        player: &mut Player,
    ) {
        let continue_slide = slide_started_at_tick
            + u64::from(self.animation_data.ticks_in_player_slide())
            >= current_tick;

        // If we are just continuing to slide, no need to update any drawable
        // data. Otherwise, will have to switch to running
        if !continue_slide {
            self.start_run(current_tick, animatable, drawable, player)
        }
    }

    fn start_jump(
        &mut self,
        current_tick: u64,
        animatable: &mut Animatable,
        drawable: &mut Drawable,
        input_ctrl: &InputControlled,
        player: &mut Player,
    ) {
        animatable.current_step_started_at_tick = current_tick;
        player.current_action = player_data::Action::Jump;
        self.jump_physics = Some(JumpPhysics::from_ground(current_tick, &self.animation_data, &self.world_data));

        self.update_drawable_for_jump_tile(current_tick, drawable, input_ctrl);
    }

    fn continue_jump_or_start_running(
        &mut self,
        current_tick: u64,
        animatable: &mut Animatable,
        drawable: &mut Drawable,
        input_ctrl: &InputControlled,
        player: &mut Player,
    ) {
        let still_jumping = self.update_drawable_for_jump_tile(
            current_tick,
            drawable,
            input_ctrl,
        );

        if !still_jumping {
            self.start_run(current_tick, animatable, drawable, player)
        }
    }

    fn start_run(
        &self,
        current_tick: u64,
        animatable: &mut Animatable,
        drawable: &mut Drawable,
        player: &mut Player,
    ) {
        animatable.current_step_started_at_tick = current_tick;
        player.current_action = player_data::Action::Run;
        self.update_drawable_for_surface_level_tile(drawable, graphics_data::CharacterTile::Run1);
    }

    fn continue_run(
        &self,
        current_tick: u64,
        run_started_at_tick: u64,
        x_offset: i32,
        animatable: &mut Animatable,
        drawable: &mut Drawable,
        player: &mut Player,
        current_tile: graphics_data::Tile,
    ) {
        let ticks_in_run_step: u64 = if x_offset == 0 {
            u64::from(self.animation_data.ticks_in_player_run_step())
        } else {
            (f32::from(self.animation_data.ticks_in_player_run_step())
                * if x_offset < 0 {
                    self.animation_data.ticks_multiplier_for_slower_running()
                } else {
                    self.animation_data.ticks_multiplier_for_faster_running()
                }) as u64
        };

        if run_started_at_tick + ticks_in_run_step <= current_tick {
            animatable.current_step_started_at_tick = current_tick;
            self.update_drawable_for_surface_level_tile(
                drawable,
                match current_tile {
                    graphics_data::Tile::Character { tile } => match tile {
                        graphics_data::CharacterTile::Run1 => graphics_data::CharacterTile::Run2,
                        graphics_data::CharacterTile::Run2 => graphics_data::CharacterTile::Run3,
                        graphics_data::CharacterTile::Run3 => graphics_data::CharacterTile::Run1,
                        _ => graphics_data::CharacterTile::Run1, //  Fallback
                    },
                    _ => graphics_data::CharacterTile::Run1, //  Fallback
                },
            )
        }

        // Move the player horizontally in the screen if wished for
        if x_offset != 0 {
            let world_x_with_offset = drawable.world_bounds.x() + x_offset;
            if world_x_with_offset >= self.world_data.bounds().left()
                && world_x_with_offset
                    <= (self.world_data.bounds().right()
                        - i32::try_from(drawable.world_bounds.width()).unwrap())
            {
                drawable.world_bounds.set_x(world_x_with_offset);
            }
        }

        player.current_action = player_data::Action::Run;
    }

    fn update_drawable_for_surface_level_tile(
        &self,
        drawable: &mut Drawable,
        tile: graphics_data::CharacterTile,
    ) {
        drawable.tile_data =
            graphics_data::build_tile_data(graphics_data::Tile::Character { tile });
        drawable
            .world_bounds
            .set_y(entities::Player::running_y(&self.world_data));
    }

    /// returns [true] if player is still going through the jump
    fn update_drawable_for_jump_tile(
        &mut self,
        current_tick: u64,
        drawable: &mut Drawable,
        input_ctrl: &InputControlled,
    ) -> bool {
        drawable.tile_data = graphics_data::build_tile_data(graphics_data::Tile::Character {
            tile: graphics_data::CharacterTile::Jump,
        });

        let jump_physics = match self.jump_physics.take() {
            Some(physics) => physics,
            None => JumpPhysics::from_ground(current_tick, &self.animation_data, &self.world_data),
        };

        let height = jump_physics.compute_height(current_tick);
        let running_y = entities::Player::running_y(&self.world_data);
        let new_y = (running_y - height).min(running_y);
        drawable.world_bounds.set_y(new_y);

        let jump_physics = jump_physics.update_gravity_if_required(current_tick, height);
        self.jump_physics = Some(jump_physics);

        // If the player is at the same height as the ground, jump animation
        // has come to an end
        (new_y != running_y) || height >= 0
    }
}

#[derive(SystemData)]
pub struct PlayerSystemData<'a> {
    game_tick: ReadExpect<'a, GameTick>,
    animatable_storage: WriteStorage<'a, Animatable>,
    drawables_storage: WriteStorage<'a, Drawable>,
    players_storage: WriteStorage<'a, Player>,
    input_controlled_storage: ReadStorage<'a, InputControlled>,
}

impl<'a> System<'a> for PlayerSystem {
    type SystemData = PlayerSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        for (mut animatable, mut drawable, input_controlled, mut player) in (
            &mut data.animatable_storage,
            &mut data.drawables_storage,
            &data.input_controlled_storage,
            &mut data.players_storage,
        )
            .join()
        {
            let start_tick = data.game_tick.ticks_animated();
            let end_tick = start_tick + data.game_tick.ticks_to_animate();
            for current_tick in start_tick..end_tick {
                self.update(
                    current_tick,
                    &mut animatable,
                    &mut drawable,
                    &input_controlled,
                    &mut player,
                )
            }
        }
    }
}
