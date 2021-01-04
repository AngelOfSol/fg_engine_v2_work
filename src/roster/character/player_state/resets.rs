use super::PlayerState;
use crate::{
    game_match::{sounds::PlayerSoundState, PlayArea},
    input::Facing,
    roster::{
        character::{
            data::Data,
            typedefs::{state::StateConsts, Character, Timed},
        },
        AllowedCancel,
    },
    typedefs::collision,
};

impl<C: Character> PlayerState<C> {
    pub fn reset_to_position_gamestart(
        &mut self,
        data: &Data<C>,
        play_area: &PlayArea,
        position: collision::Int,
        facing: Facing,
    ) {
        *self = PlayerState {
            position: collision::Vec2::new(position, 0),
            velocity: collision::Vec2::zeros(),
            current_state: Timed {
                id: C::State::GAME_START,
                time: 0,
            },
            stun: None,
            air_actions: data.properties.max_air_actions,
            spirit_gauge: 0,
            spirit_delay: 0,
            hitstop: 0,
            facing,
            health: data.properties.health,
            allowed_cancels: AllowedCancel::Always,
            rebeat_chain: Default::default(),
            should_pushback: true,
            sound_state: PlayerSoundState::new(),
            meter: 0,
            lockout: 0,
            dead: false,
            smp: Default::default(),
            most_recent_command: Timed {
                id: C::Command::default(),
                time: 0,
            },
            last_hit_using: None,
            current_combo: None,
            other: std::mem::take(&mut self.other),
        };
        self.other.round_start_reset(data);

        self.validate_position(data, play_area);
    }

    pub fn reset_to_position_roundstart(
        &mut self,
        data: &Data<C>,
        play_area: &PlayArea,
        position: collision::Int,
        facing: Facing,
    ) {
        *self = PlayerState {
            position: collision::Vec2::new(position, 0),
            velocity: collision::Vec2::zeros(),
            current_state: Timed {
                id: C::State::ROUND_START,
                time: 0,
            },
            stun: None,
            air_actions: data.properties.max_air_actions,
            spirit_gauge: 0,
            spirit_delay: 0,
            hitstop: 0,
            facing,
            health: data.properties.health,
            allowed_cancels: AllowedCancel::Always,
            rebeat_chain: Default::default(),
            should_pushback: true,
            sound_state: PlayerSoundState::new(),
            meter: 0,
            lockout: 0,
            dead: false,
            smp: Default::default(),
            most_recent_command: Timed {
                id: C::Command::default(),
                time: 0,
            },
            last_hit_using: None,
            current_combo: None,
            other: std::mem::take(&mut self.other),
        };
        self.other.round_start_reset(data);
        self.validate_position(data, play_area);
    }
}
