use crate::components::input::InputControlled;
use crate::components::player::data as player_data;
use crate::components::player::Player;
use crate::components::{Animatable, Drawable};
use crate::entities;

use crate::graphics::data as graphics_data;
use crate::resources::GamePlay;

use crate::data::{PlayerData, WorldData};
use crate::graphics::data::CharacterTile;
use crate::jump_physics::JumpPhysics;
use specs::join::Join;
use specs::shred::ResourceId;
use specs::{ReadExpect, System, WriteStorage};
use specs::{ReadStorage, SystemData};
use specs::{World, WriteExpect};
use std::convert::TryFrom;

pub struct PlayerSystem {
    world_data: WorldData,
}

impl PlayerSystem {
    pub fn new(world_data: WorldData) -> PlayerSystem {
        PlayerSystem { world_data }
    }

    fn input_to_vertical_action(
        current_tick: u64,
        player_data: &PlayerData,
        player: &Player,
        input_ctrl: &InputControlled,
    ) -> Option<player_data::Action> {
        // Order of precedence of actions: jump over slide
        let mut input_action = None;
        if input_action.is_none() && input_ctrl.up_engaged() && !input_ctrl.down_engaged() {
            let ticks_since_last_jump = current_tick - player.most_recent_max_jump_end_at;
            if ticks_since_last_jump >= u64::from(player_data.ticks_between_consecutive_jumps) {
                input_action = Some(player_data::Action::Jump)
            }
        }

        if input_action.is_none() && input_ctrl.down_engaged() && !input_ctrl.up_engaged() {
            let ticks_since_last_slide = current_tick - player.most_recent_max_slide_end_at;
            if ticks_since_last_slide >= u64::from(player_data.ticks_between_consecutive_slides) {
                input_action = Some(player_data::Action::Slide)
            }
        }

        input_action
    }

    fn input_to_bounded_x_offset(
        &self,
        player_data: &PlayerData,
        drawable: &mut Drawable,
        input_ctrl: &InputControlled,
    ) -> i32 {
        let x_offset = if input_ctrl.right_engaged() && !input_ctrl.left_engaged() {
            player_data.speed_in_wc_per_tick_fast_run.into()
        } else if input_ctrl.left_engaged() && !input_ctrl.right_engaged() {
            -(i32::from(player_data.speed_in_wc_per_tick_slow_run))
        } else {
            0 // Player will run at place which will keep the player keep up with ground
        };

        let new_x = drawable.world_bounds.x() + x_offset;
        let new_x = new_x.max(self.world_data.bounds().left()).min(
            self.world_data.bounds().right()
                - i32::try_from(drawable.world_bounds.width()).unwrap(),
        );

        new_x - drawable.world_bounds.x()
    }

    fn update(
        &mut self,
        current_tick: u64,
        player_data: &PlayerData,
        animatable: &mut Animatable,
        drawable: &mut Drawable,
        input_ctrl: &InputControlled,
        player: &mut Player,
    ) {
        let tile = drawable.tile_data.tile;
        let current_step_started_at_tick = animatable.current_step_started_at_tick;

        // Move the player horizontally in the screen if wished for
        let bounded_x_offset = self.input_to_bounded_x_offset(player_data, drawable, input_ctrl);
        drawable.world_bounds.offset(bounded_x_offset, 0);

        // Move (if required) & animate with appropriate action
        match player.current_action {
            // Already an uninterruptible input based animation is going on. Transfer
            // the control over to that to either carry on the animation or to finish
            // it. We don't worry about new inputs at this point
            player_data::Action::Jump => self.continue_jump_or_start_running(
                current_tick,
                player_data,
                animatable,
                drawable,
                input_ctrl,
                player,
            ),

            // Another uninterruptible animation
            player_data::Action::Slide => self.continue_slide_or_start_running(
                current_tick,
                current_step_started_at_tick,
                player_data,
                animatable,
                drawable,
                player,
                input_ctrl,
            ),

            // No input based animation going on. Gotta check if we should start one now
            player_data::Action::Run => {
                let new_action = PlayerSystem::input_to_vertical_action(
                    current_tick,
                    player_data,
                    player,
                    input_ctrl,
                );

                match new_action {
                    // Some new input based action to start
                    Some(action) => match action {
                        player_data::Action::Jump => self.start_jump(
                            current_tick,
                            player_data,
                            animatable,
                            drawable,
                            input_ctrl,
                            player,
                        ),

                        player_data::Action::Slide => self.start_slide(
                            current_tick,
                            current_tick,
                            player_data,
                            animatable,
                            drawable,
                            player,
                            input_ctrl,
                        ),

                        player_data::Action::Run => self.continue_run(
                            current_tick,
                            current_step_started_at_tick,
                            bounded_x_offset,
                            player_data,
                            animatable,
                            drawable,
                            player,
                            tile,
                        ),
                    },

                    // Nothing else to do! Just continue running
                    None => self.continue_run(
                        current_tick,
                        current_step_started_at_tick,
                        bounded_x_offset,
                        player_data,
                        animatable,
                        drawable,
                        player,
                        tile,
                    ),
                }
            }
        }
    }

    fn start_slide(
        &self,
        current_tick: u64,
        slide_started_at_tick: u64,
        player_data: &PlayerData,
        animatable: &mut Animatable,
        drawable: &mut Drawable,
        player: &mut Player,
        input_ctrl: &InputControlled,
    ) {
        animatable.current_step_started_at_tick = current_tick;
        player.current_action = player_data::Action::Slide;
        self.update_drawable_for_slide_tile(
            current_tick,
            slide_started_at_tick,
            player_data,
            drawable,
            input_ctrl,
        );
    }

    fn continue_slide_or_start_running(
        &self,
        current_tick: u64,
        slide_started_at_tick: u64,
        player_data: &PlayerData,
        animatable: &mut Animatable,
        drawable: &mut Drawable,
        player: &mut Player,
        input_ctrl: &InputControlled,
    ) {
        let continue_slide = self.update_drawable_for_slide_tile(
            current_tick,
            slide_started_at_tick,
            player_data,
            drawable,
            input_ctrl,
        );

        if !continue_slide {
            player.most_recent_max_slide_end_at = current_tick;
            self.start_run(current_tick, animatable, drawable, player)
        }
    }

    fn start_jump(
        &mut self,
        current_tick: u64,
        player_data: &PlayerData,
        animatable: &mut Animatable,
        drawable: &mut Drawable,
        input_ctrl: &InputControlled,
        player: &mut Player,
    ) {
        animatable.current_step_started_at_tick = current_tick;
        player.current_action = player_data::Action::Jump;
        self.jump_physics = Some(JumpPhysics::from_ground(
            current_tick,
            player_data.ticks_in_max_jump,
            player_data.max_jump_height_in_wc,
        ));

        self.update_drawable_for_jump_tile(current_tick, player_data, drawable, input_ctrl);
    }

    fn continue_jump_or_start_running(
        &mut self,
        current_tick: u64,
        player_data: &PlayerData,
        animatable: &mut Animatable,
        drawable: &mut Drawable,
        input_ctrl: &InputControlled,
        player: &mut Player,
    ) {
        let still_jumping =
            self.update_drawable_for_jump_tile(current_tick, player_data, drawable, input_ctrl);
        if !still_jumping {
            player.most_recent_max_jump_end_at = current_tick;
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
        self.update_drawable_for_run_tile(drawable, graphics_data::CharacterTile::Run1);
    }

    fn continue_run(
        &self,
        current_tick: u64,
        run_started_at_tick: u64,
        x_offset: i32,
        player_data: &PlayerData,
        animatable: &mut Animatable,
        drawable: &mut Drawable,
        player: &mut Player,
        current_tile: graphics_data::Tile,
    ) {
        let ticks_in_run_step: u64 = if x_offset == 0 {
            u64::from(player_data.ticks_in_run_step)
        } else {
            (f32::from(player_data.ticks_in_run_step)
                * if x_offset < 0 {
                    player_data.ticks_multiplier_for_slower_running
                } else {
                    player_data.ticks_multiplier_for_faster_running
                }) as u64
        };

        if run_started_at_tick + ticks_in_run_step <= current_tick {
            animatable.current_step_started_at_tick = current_tick;
            self.update_drawable_for_run_tile(
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

        player.current_action = player_data::Action::Run;
    }

    fn update_drawable_for_run_tile(
        &self,
        drawable: &mut Drawable,
        tile: graphics_data::CharacterTile,
    ) {
        *drawable = entities::PlayerEntity::build_drawable_with_left_bottom(
            tile,
            drawable.world_bounds.left(),
            self.world_data.world_surface_at(),
        );
    }

    /// returns [true] if player is still going through the slide
    fn update_drawable_for_slide_tile(
        &self,
        current_tick: u64,
        slide_started_at_tick: u64,
        player_data: &PlayerData,
        drawable: &mut Drawable,
        input_ctrl: &InputControlled,
    ) -> bool {
        *drawable = entities::PlayerEntity::build_drawable_with_left_bottom(
            graphics_data::CharacterTile::Slide,
            drawable.world_bounds.left(),
            self.world_data.world_surface_at(),
        );

        let enough_ticks_passed_in_slide =
            current_tick >= slide_started_at_tick + u64::from(player_data.ticks_in_slide);

        // Slide can go on only for so long & also `down` should be engaged
        input_ctrl.down_engaged() && !enough_ticks_passed_in_slide
    }

    /// returns [true] if player is still going through the jump
    fn update_drawable_for_jump_tile(
        &mut self,
        current_tick: u64,
        player_data: &PlayerData,
        drawable: &mut Drawable,
        input_ctrl: &InputControlled,
    ) -> bool {
        let jump_physics = match self.jump_physics.take() {
            Some(physics) => physics,
            None => JumpPhysics::from_ground(
                current_tick,
                player_data.ticks_in_max_jump,
                player_data.max_jump_height_in_wc,
            ),
        };

        let height = jump_physics.compute_height(current_tick);
        let world_surface = self.world_data.world_surface_at();
        let new_y = (world_surface - height).min(world_surface);

        *drawable = entities::PlayerEntity::build_drawable_with_left_bottom(
            graphics_data::CharacterTile::Jump,
            drawable.world_bounds.left(),
            new_y,
        );

        let jump_physics =
            jump_physics.update_gravity_if_required(current_tick, height, input_ctrl);
        self.jump_physics = Some(jump_physics);

        // If the player is at the same height as the ground, jump animation
        // has come to an end
        (new_y != world_surface) || height >= 0
    }
}

#[derive(SystemData)]
pub struct PlayerSystemData<'a> {
    game_play: WriteExpect<'a, GamePlay>,
    player_data: ReadExpect<'a, PlayerData>,
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
            let start_tick = data.game_play.ticks_animated();
            let end_tick = start_tick + data.game_play.ticks_to_animate();
            for current_tick in start_tick..end_tick {
                if player.is_hit {
                    *drawable = entities::PlayerEntity::build_drawable_with_left_bottom(
                        CharacterTile::Hit,
                        drawable.world_bounds.left(),
                        drawable.world_bounds.bottom(),
                    );

                    data.game_play.mark_over()
                } else {
                    self.update(
                        current_tick,
                        &data.player_data,
                        &mut animatable,
                        &mut drawable,
                        &input_controlled,
                        &mut player,
                    )
                }
            }
        }
    }
}
