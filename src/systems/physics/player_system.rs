use crate::components;
use crate::components::Player;
use crate::components::{Drawable, UserControlled};
use crate::entities;

use crate::graphics::data;
use crate::resources::FrameStepper;

use crate::WorldData;
use specs::join::Join;
use specs::shred::ResourceId;
use specs::SystemData;
use specs::World;
use specs::{ReadExpect, System, WriteStorage};
use std::convert::TryFrom;

const FRAMES_IN_JUMP_ANIMATION: u8 = 40;
const FRAMES_IN_SLIDE_ANIMATION: u8 = 40;
const FRAMES_IN_RUN_ANIMATION: u8 = 12;
const JUMP_HEIGHT_IN_WORLD_COORDINATES: u8 = 100;

// Derived using math from a GDC talk (for smooth parabolic jump):
//      GDC link: https://www.gdcvault.com/play/1023559/Math-for-Game-Programmers-Building
//      Video: https://www.youtube.com/watch?v=hG9SzQxaCm8
//      Slides: http://www.mathforgameprogrammers.com/gdc2016/GDC2016_Pittman_Kyle_BuildingABetterJump.pdf
const FRAMES_TO_HIT_APEX_IN_JUMP: f32 = FRAMES_IN_JUMP_ANIMATION as f32 / 2.0;
const JUMP_VELOCITY: f32 = -JUMP_GRAVITY * FRAMES_TO_HIT_APEX_IN_JUMP;
const JUMP_GRAVITY: f32 = (-2.0 * JUMP_HEIGHT_IN_WORLD_COORDINATES as f32)
    / (FRAMES_TO_HIT_APEX_IN_JUMP * FRAMES_TO_HIT_APEX_IN_JUMP);

pub struct PlayerSystem {
    world_data: WorldData,
}

impl PlayerSystem {
    pub fn new(world_data: WorldData) -> PlayerSystem {
        PlayerSystem { world_data }
    }

    fn update(
        &self,
        current_frame_count: u64,
        drawable: &mut Drawable,
        player: &mut Player,
        user_controlled: &mut UserControlled,
    ) {
        let tile = drawable.tile_data.tile;
        let current_step_started_at_frame = player.current_step_started_at_frame;

        if let data::Tile::Character { tile } = tile {
            match tile {
                // Already an input based animation is going on. Transfer the control
                // over to that to either carry on the animation or to finish it. When
                // an animation is going on, we don't care about more inputs from the
                // user
                data::CharacterTile::Jump => self.continue_jump_or_start_running(
                    current_frame_count,
                    current_step_started_at_frame,
                    drawable,
                    player,
                ),

                // Same comment as jump applies here too
                data::CharacterTile::Slide => self.continue_slide_or_start_running(
                    current_frame_count,
                    current_step_started_at_frame,
                    drawable,
                    player,
                ),

                // No input based animation going on. Gotta check if we should start
                // one now
                _ => match user_controlled.consume_input() {
                    // There is some input! Start a new input based animation
                    Some(input) => match input {
                        // Gotta jump!
                        components::data::Input::Up => {
                            self.start_jump(current_frame_count, drawable, player)
                        }

                        // Gotta slide!
                        components::data::Input::Down => {
                            self.start_slide(current_frame_count, drawable, player)
                        }
                    },

                    // Nothing else to do! Just continue running
                    None => self.continue_run(
                        current_frame_count,
                        current_step_started_at_frame,
                        drawable,
                        player,
                        tile,
                    ),
                },
            }
        }

        // If the input isn't consumed because there is another animation
        // already running, we don't want the input to be left around to be
        // used later! So, no matter what happens at the end of each consumed
        // frame, clear the user input to allow taking in more
        user_controlled.consume_input();
    }

    fn start_slide(&self, current_frame_count: u64, drawable: &mut Drawable, player: &mut Player) {
        player.current_step_started_at_frame = current_frame_count;
        self.update_drawable_for_surface_level_tile(drawable, data::CharacterTile::Slide);
    }

    fn continue_slide_or_start_running(
        &self,
        current_frame_count: u64,
        slide_started_at_frame: u64,
        drawable: &mut Drawable,
        player: &mut Player,
    ) {
        let continue_slide =
            slide_started_at_frame + u64::from(FRAMES_IN_JUMP_ANIMATION) >= current_frame_count;

        // If we are just continuing to slide, no need to update any drawable
        // data. Otherwise, will have to switch to running
        if !continue_slide {
            self.start_run(current_frame_count, drawable, player)
        }
    }

    fn start_jump(&self, current_frame_count: u64, drawable: &mut Drawable, player: &mut Player) {
        player.current_step_started_at_frame = current_frame_count;
        self.update_drawable_for_jump_tile(current_frame_count, current_frame_count, drawable);
    }

    fn continue_jump_or_start_running(
        &self,
        current_frame_count: u64,
        jump_started_at_frame: u64,
        drawable: &mut Drawable,
        player: &mut Player,
    ) {
        let continue_jump =
            jump_started_at_frame + u64::from(FRAMES_IN_SLIDE_ANIMATION) >= current_frame_count;

        if continue_jump {
            self.update_drawable_for_jump_tile(current_frame_count, jump_started_at_frame, drawable)
        } else {
            self.start_run(current_frame_count, drawable, player)
        }
    }

    fn start_run(&self, current_frame_count: u64, drawable: &mut Drawable, player: &mut Player) {
        player.current_step_started_at_frame = current_frame_count;
        self.update_drawable_for_surface_level_tile(drawable, data::CharacterTile::Run1);
    }

    fn continue_run(
        &self,
        current_frame_count: u64,
        run_started_at_frame: u64,
        drawable: &mut Drawable,
        player: &mut Player,
        current_tile: data::CharacterTile,
    ) {
        let change_tile =
            run_started_at_frame + u64::from(FRAMES_IN_RUN_ANIMATION) <= current_frame_count;

        if change_tile {
            player.current_step_started_at_frame = current_frame_count;
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
        current_frame_count: u64,
        jump_started_at_frame: u64,
        drawable: &mut Drawable,
    ) {
        drawable.tile_data = data::build_tile_data(data::Tile::Character {
            tile: data::CharacterTile::Jump,
        });

        let frames_since_jump_started = (current_frame_count - jump_started_at_frame) as f32;
        let height = ((JUMP_GRAVITY * frames_since_jump_started.powf(2.0)) / 2.0)
            + (JUMP_VELOCITY * frames_since_jump_started);

        drawable
            .world_bounds
            .set_y(entities::Player::running_y(&self.world_data) - height.round() as i32);
    }
}

#[derive(SystemData)]
pub struct PlayerSystemData<'a> {
    frame_stepper: ReadExpect<'a, FrameStepper>,
    drawables_storage: WriteStorage<'a, Drawable>,
    players_storage: WriteStorage<'a, Player>,
    user_controlled_storage: WriteStorage<'a, UserControlled>,
}

impl<'a> System<'a> for PlayerSystem {
    type SystemData = PlayerSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        for (mut drawable, mut player, mut user_controlled) in (
            &mut data.drawables_storage,
            &mut data.players_storage,
            &mut data.user_controlled_storage,
        )
            .join()
        {
            let start_frame_count = data.frame_stepper.frame_count_animated();
            let end_frame_count = start_frame_count + data.frame_stepper.frame_count_to_animate();
            for frame_count in start_frame_count..end_frame_count {
                self.update(
                    frame_count,
                    &mut drawable,
                    &mut player,
                    &mut user_controlled,
                )
            }
        }
    }
}
