pub mod attack;
pub mod combo;
pub mod hitstun;
pub mod input;
pub mod meter;
pub mod object;
pub mod physics;
pub mod resets;
pub mod spirit;
pub mod update;

use super::{
    data::Data,
    smp::SmpList,
    typedefs::{state::StateConsts, Character, HitId, Timed},
};
use crate::{
    game_match::sounds::{PlayerSoundState, SoundPath},
    input::Facing,
    roster::{hit_info::ComboEffect, AllowedCancel},
    typedefs::collision,
};
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct PlayerState<C: Character> {
    pub velocity: collision::Vec2,
    pub position: collision::Vec2,
    pub current_state: Timed<C::State>,
    pub last_hit_using: Option<HitId<C::Attack>>,
    pub allowed_cancels: AllowedCancel,
    pub rebeat_chain: HashSet<C::Command>,
    pub smp: SmpList<C::Command>,
    pub most_recent_command: Timed<C::Command>,
    pub air_actions: usize,
    pub stun: Option<i32>,
    pub health: i32,
    pub spirit_gauge: i32,
    pub spirit_delay: i32,
    pub hitstop: i32,
    pub meter: i32,
    pub lockout: i32,
    pub dead: bool,
    pub should_pushback: bool,
    pub facing: Facing,
    pub current_combo: Option<ComboEffect>,
    pub other: C,
    pub sound_state: PlayerSoundState<SoundPath<C::Sound>>,
}

impl<C: Character> PlayerState<C> {
    pub fn new(data: &Data<C>) -> Self {
        Self {
            velocity: collision::Vec2::zeros(),
            position: collision::Vec2::zeros(),
            current_state: Timed {
                id: C::State::GAME_START,
                time: 0,
            },
            stun: None,
            air_actions: data.properties.max_air_actions,
            spirit_gauge: 0,
            spirit_delay: 0,
            hitstop: 0,
            facing: Facing::Right,
            health: data.properties.health,
            allowed_cancels: AllowedCancel::Always,
            rebeat_chain: HashSet::new(),
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
            other: C::default(),
        }
    }
}
