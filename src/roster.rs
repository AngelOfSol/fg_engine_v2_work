#[macro_use]
pub mod generic_character;
mod yuyuko;

use enum_dispatch::enum_dispatch;

pub use generic_character::*;
pub use yuyuko::*;

use crate::character::state::components::GlobalGraphic;
use crate::game_match::sounds::{GlobalSound, SoundList};
use crate::game_match::{FlashType, PlayArea, UiElements};
use crate::graphics::animation_group::AnimationGroup;
use crate::hitbox::PositionedHitbox;
use crate::input::{Facing, InputState};
use crate::typedefs::{collision, graphics};
use crate::{assets::Assets, character::components::AttackInfo};
use ggez::{Context, GameResult};
use hit_info::{ComboEffect, HitEffect, HitResult, HitType, Source};
use rodio::Device;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;
use strum::IntoEnumIterator;
use strum::{Display, EnumCount, EnumIter};

#[enum_dispatch]
pub enum CharacterBehavior {
    YuyukoPlayer,
}

#[derive(Clone, Debug)]
pub enum CharacterData {
    Yuyuko(Rc<Yuyuko>),
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
    pub fn sound_name_iter(self) -> impl Iterator<Item = String> {
        match self {
            Character::Yuyuko => YuyukoSound::iter().map(|item| item.to_string()),
        }
    }
    pub fn data_id_iter(self) -> impl Iterator<Item = String> {
        match self {
            Character::Yuyuko => YuyukoDataId::iter().map(|item| item.to_string()),
        }
    }
    pub fn graphic_name_iter(self) -> impl Iterator<Item = String> {
        match self {
            Character::Yuyuko => YuyukoGraphic::iter().map(|item| item.file_name()),
        }
    }

    pub fn load_data(self, ctx: &mut Context, assets: &mut Assets) -> GameResult<CharacterData> {
        match self {
            Character::Yuyuko => Ok(CharacterData::Yuyuko(Rc::new(Yuyuko::new_with_path(
                ctx,
                assets,
                PathBuf::from("./resources/yuyuko.json"),
            )?))),
        }
    }
}

impl CharacterData {
    pub fn make_character(&self) -> CharacterBehavior {
        match self {
            CharacterData::Yuyuko(data) => YuyukoPlayer::new(data.clone()).into(),
        }
    }
    pub fn is_for(&self, character: Character) -> bool {
        matches!(
            (self, character),
            (CharacterData::Yuyuko(_), Character::Yuyuko)
        )
    }
}
