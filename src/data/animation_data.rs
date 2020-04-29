#[derive(Copy, Clone, Debug)]
pub struct AnimationData {
    ground_speed_in_wc_per_tick: u8,
    min_ticks_between_plants: u64,

    // Player related
    ticks_in_player_jump: u8,
    ticks_in_player_slide: u8,
    ticks_in_player_run_step: u8,
    player_jump_height_in_wc: u8,
    player_extra_input_speed_in_wc_per_tick: u8,

    // Enemy related
    enemy_speed_in_wc_per_tick: u8,
    ticks_in_enemy_movement: u8,
    min_ticks_between_enemies: u64,
}

impl Default for AnimationData {
    fn default() -> Self {
        AnimationData {
            ground_speed_in_wc_per_tick: 4,
            min_ticks_between_plants: 250,

            ticks_in_player_jump: 60,
            ticks_in_player_slide: 40,
            ticks_in_player_run_step: 12,
            player_jump_height_in_wc: 120,
            player_extra_input_speed_in_wc_per_tick: 4,

            enemy_speed_in_wc_per_tick: 6,
            ticks_in_enemy_movement: 12,
            min_ticks_between_enemies: 180,
        }
    }
}

impl AnimationData {
    pub fn new() -> AnimationData {
        AnimationData::default()
    }

    pub fn ground_speed_in_wc_per_tick(&self) -> u8 {
        self.ground_speed_in_wc_per_tick
    }

    pub fn min_ticks_between_plants(&self) -> u64 {
        self.min_ticks_between_plants
    }

    pub fn ticks_in_player_jump(&self) -> u8 {
        self.ticks_in_player_jump
    }

    pub fn ticks_in_player_slide(&self) -> u8 {
        self.ticks_in_player_slide
    }

    pub fn ticks_in_player_run_step(&self) -> u8 {
        self.ticks_in_player_run_step
    }

    pub fn player_jump_height_in_wc(&self) -> u8 {
        self.player_jump_height_in_wc
    }

    pub fn player_extra_input_speed_in_wc_per_tick(&self) -> u8 {
        self.player_extra_input_speed_in_wc_per_tick
    }

    pub fn enemy_speed_in_wc_per_tick(&self) -> u8 {
        self.enemy_speed_in_wc_per_tick
    }

    pub fn ticks_in_enemy_movement(&self) -> u8 {
        self.ticks_in_enemy_movement
    }

    pub fn min_ticks_between_enemies(&self) -> u64 {
        self.min_ticks_between_enemies
    }
}
