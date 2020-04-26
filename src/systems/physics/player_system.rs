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

const FRAMES_IN_JUMP_ANIMATION: u8 = 60;
const FRAMES_IN_SLIDE_ANIMATION: u8 = 60;
const FRAMES_IN_WALK_ANIMATION: u8 = 20;
const JUMP_HEIGHT_IN_WORLD_COORDINATES: u8 = 100;

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
                data::CharacterTile::Jump => self.continue_jump_or_start_walking(
                    current_frame_count,
                    current_step_started_at_frame,
                    drawable,
                    player,
                ),

                // Same comment as jump applies here too
                data::CharacterTile::Slide => self.continue_slide_or_start_walking(
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

                    // Nothing else to do! Just continue walking
                    None => self.continue_walk(
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

    fn continue_slide_or_start_walking(
        &self,
        current_frame_count: u64,
        slide_started_at_frame: u64,
        drawable: &mut Drawable,
        player: &mut Player,
    ) {
        let continue_slide =
            slide_started_at_frame + u64::from(FRAMES_IN_JUMP_ANIMATION) >= current_frame_count;

        // If we are just continuing to slide, no need to update any drawable
        // data. Otherwise, will have to switch to walking
        if !continue_slide {
            self.start_walk(current_frame_count, drawable, player)
        }
    }

    fn start_jump(&self, current_frame_count: u64, drawable: &mut Drawable, player: &mut Player) {
        player.current_step_started_at_frame = current_frame_count;
        self.update_drawable_for_jump_tile(current_frame_count, current_frame_count, drawable);
    }

    fn continue_jump_or_start_walking(
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
            self.start_walk(current_frame_count, drawable, player)
        }
    }

    fn start_walk(&self, current_frame_count: u64, drawable: &mut Drawable, player: &mut Player) {
        player.current_step_started_at_frame = current_frame_count;
        self.update_drawable_for_surface_level_tile(drawable, data::CharacterTile::Walk1);
    }

    fn continue_walk(
        &self,
        current_frame_count: u64,
        walk_started_at_frame: u64,
        drawable: &mut Drawable,
        player: &mut Player,
        current_tile: data::CharacterTile,
    ) {
        let change_tile =
            walk_started_at_frame + u64::from(FRAMES_IN_WALK_ANIMATION) <= current_frame_count;

        if change_tile {
            player.current_step_started_at_frame = current_frame_count;
            self.update_drawable_for_surface_level_tile(
                drawable,
                match current_tile {
                    data::CharacterTile::Walk1 => data::CharacterTile::Walk2,
                    data::CharacterTile::Walk2 => data::CharacterTile::Walk3,
                    data::CharacterTile::Walk3 => data::CharacterTile::Walk1,
                    _ => data::CharacterTile::Walk1, //  Fallback
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
            .set_y(entities::Player::walking_y(&self.world_data));
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

        let frames_since_jump_started = current_frame_count - jump_started_at_frame;
        let frames_to_hit_apex = u64::from(FRAMES_IN_JUMP_ANIMATION) / 2;

        let travel_per_frame: u64 = (f32::from(JUMP_HEIGHT_IN_WORLD_COORDINATES)
            / frames_to_hit_apex as f32)
            .round() as u64;

        let jump_height = if frames_since_jump_started <= frames_to_hit_apex {
            travel_per_frame * frames_since_jump_started
        } else {
            u64::from(JUMP_HEIGHT_IN_WORLD_COORDINATES)
                - (travel_per_frame * (frames_since_jump_started - frames_to_hit_apex))
        };

        drawable.world_bounds.set_y(
            entities::Player::walking_y(&self.world_data)
                - i32::try_from(jump_height).expect("Jumped too high!"),
        );
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
