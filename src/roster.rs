#[macro_use]
pub mod generic_character;
mod yuyuko;

use enum_dispatch::enum_dispatch;

pub use generic_character::*;
pub use yuyuko::*;

use crate::character::components::AttackInfo;
use crate::character::state::components::GlobalParticle;
use crate::game_match::sounds::{GlobalSound, SoundList};
use crate::game_match::{FlashType, PlayArea};
use crate::graphics::particle::Particle;
use crate::hitbox::PositionedHitbox;
use crate::input::{Facing, InputState};
use crate::typedefs::{collision, graphics};
use ggez::{Context, GameResult};
use hit_info::{HitAction, HitEffect, HitResult};
use rodio::Device;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumCount, EnumIter};

#[enum_dispatch]
pub enum CharacterBehavior {
    YuyukoPlayer,
}

#[derive(Debug, Copy, Clone, PartialEq, EnumIter, Display, EnumCount, Serialize, Deserialize)]
pub enum Character {
    Yuyuko,
}

impl Default for Character {
    fn default() -> Self {
        Self::Yuyuko
    }
}

impl Character {
    pub fn sound_name_iterator(self) -> impl Iterator<Item = String> {
        match self {
            Character::Yuyuko => YuyukoSound::iter()
                //.collect::<Vec<_>>()
                //.into_iter()
                .map(|item| item.to_string()),
        }
    }
}
