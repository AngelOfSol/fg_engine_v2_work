#[macro_use]
pub mod generic_character;
mod yuyuko;

use enum_dispatch::enum_dispatch;

pub use generic_character::*;
pub use yuyuko::*;

use crate::character::components::AttackInfo;
use crate::character::state::components::Flags;
use crate::game_match::sounds::SoundList;
use crate::game_match::PlayArea;
use crate::hitbox::PositionedHitbox;
use crate::input::{Facing, InputState};
use crate::typedefs::{collision, graphics};
use ggez::{Context, GameResult};
use hit_info::{HitInfo, HitType};
use rodio::Device;

#[enum_dispatch]
pub enum CharacterBehavior {
    YuyukoPlayer,
}
