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
use specs::{ReadExpect, ReadStorage, System, WriteStorage};

const FRAMES_PER_TILE: u8 = 20;
const FRAMES_IN_INPUT_BASED_ANIMATION: u8 = FRAMES_PER_TILE * 5;

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
        drawables_storage: &mut WriteStorage<Drawable>,
        players_storage: &mut WriteStorage<Player>,
        user_controlled_storage: &mut WriteStorage<UserControlled>,
    ) {
        for (drawable, player, user_controlled) in
            (drawables_storage, players_storage, user_controlled_storage).join()
        {
            let tile = drawable.tile_data.tile;
            if let data::Tile::Character { tile } = tile {
                match player.input_based_animation_started_at_frame {
                    // Already an input based animation is going on. Transfer the control
                    // over to that to either carry on the animation or to finish it. When
                    // an animation is going on, we don't care about more inputs from the
                    // user
                    Some(animation_start_frame) => match tile {
                        data::CharacterTile::Jump => self.continue_jump_or_start_walking(
                            current_frame_count,
                            animation_start_frame,
                            drawable,
                            player,
                        ),

                        data::CharacterTile::Slide => self.continue_slide_or_start_walking(
                            current_frame_count,
                            animation_start_frame,
                            drawable,
                            player,
                        ),

                        // This shouldn't happen but we have this here to satisfy
                        // exhaustive checks
                        _ => self.animate_walk(drawable, player, tile),
                    },

                    // No input based animation going on. Gotta check if we should start
                    // one now
                    None => match user_controlled.consume_input() {
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
                        None => self.animate_walk(drawable, player, tile),
                    },
                }
            }

            // If the input isn't consumed because there is another animation
            // already running, we don't want the input to be left around to be
            // used later! So, no matter what happens at the end of each consumed
            // frame, clear the user input to allow taking in more
            user_controlled.consume_input();
        }
    }

    fn start_slide(&self, current_frame_count: u64, drawable: &mut Drawable, player: &mut Player) {
        // Update data about user controlled animation start
        player.input_based_animation_started_at_frame = Some(current_frame_count);

        // Update drawable
        self.update_drawable_for_surface_level_tile(drawable, data::CharacterTile::Slide);
    }

    fn continue_slide_or_start_walking(
        &self,
        current_frame_count: u64,
        animation_start_frame: u64,
        drawable: &mut Drawable,
        player: &mut Player,
    ) {
        let continue_slide = animation_start_frame + u64::from(FRAMES_IN_INPUT_BASED_ANIMATION)
            >= current_frame_count;

        // If we are just continuing to slide, no need to update any drawable
        // data. Otherwise, will have to switch to walking
        if !continue_slide {
            self.animate_walk(drawable, player, data::CharacterTile::Slide)
        }
    }

    fn start_jump(&self, current_frame_count: u64, drawable: &mut Drawable, player: &mut Player) {
        // Update data about user controlled animation start
        player.input_based_animation_started_at_frame = Some(current_frame_count);

        // Update drawable
        self.update_drawable_for_jump_tile(drawable);
    }

    fn continue_jump_or_start_walking(
        &self,
        current_frame_count: u64,
        animation_start_frame: u64,
        drawable: &mut Drawable,
        player: &mut Player,
    ) {
        let continue_jump = animation_start_frame + u64::from(FRAMES_IN_INPUT_BASED_ANIMATION)
            >= current_frame_count;

        // If we are just continuing to jump, no need to update any drawable
        // data. Otherwise, will have to switch to walking
        if !continue_jump {
            self.animate_walk(drawable, player, data::CharacterTile::Slide)
        }
    }

    fn animate_walk(
        &self,
        drawable: &mut Drawable,
        player: &mut Player,
        current_tile: data::CharacterTile,
    ) {
        // Reset data about user controlled animation start
        player.input_based_animation_started_at_frame = None;

        // Update drawable
        self.update_drawable_for_surface_level_tile(
            drawable,
            match current_tile {
                data::CharacterTile::Walk1 => data::CharacterTile::Walk2,
                data::CharacterTile::Walk2 => data::CharacterTile::Walk1,
                _ => data::CharacterTile::Walk1, //  If any other animation is going on, start walking
            },
        )
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

    fn update_drawable_for_jump_tile(&self, drawable: &mut Drawable) {
        drawable.tile_data = data::build_tile_data(data::Tile::Character {
            tile: data::CharacterTile::Jump,
        });

        drawable
            .world_bounds
            .set_y(entities::Player::walking_y(&self.world_data) - 75);
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
        let start_frame_count = data.frame_stepper.frame_count_animated();
        let end_frame_count = start_frame_count + data.frame_stepper.frame_count_to_animate();
        if start_frame_count != end_frame_count {
            for frame_count in start_frame_count..end_frame_count {
                if frame_count % u64::from(FRAMES_PER_TILE) == 0 {
                    self.update(
                        frame_count,
                        &mut data.drawables_storage,
                        &mut data.players_storage,
                        &mut data.user_controlled_storage,
                    )
                }
            }
        }
    }
}
