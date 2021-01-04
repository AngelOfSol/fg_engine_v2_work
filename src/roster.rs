#[macro_use]
pub mod generic_character;
pub mod character;
pub mod yuyuko;

use crate::character::state::components::GlobalGraphic;
use crate::game_match::sounds::{GlobalSound, SoundList};
use crate::game_match::{FlashType, PlayArea, UiElements};
use crate::graphics::animation_group::AnimationGroup;
use crate::hitbox::PositionedHitbox;
use crate::input::{Facing, InputState};
use crate::typedefs::{collision, graphics};
use crate::{assets::Assets, character::components::AttackInfo};
use character::{data::Data, Player};
use enum_dispatch::enum_dispatch;
pub use generic_character::*;
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
use yuyuko::YuyukoType;

#[allow(clippy::large_enum_variant)]
#[enum_dispatch]
pub enum CharacterBehavior {
    YuyukoPlayer(Player<YuyukoType>),
}

#[derive(Clone)]
pub enum CharacterData {
    Yuyuko(Rc<Data<YuyukoType>>),
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
            Character::Yuyuko => yuyuko::Sound::iter().map(|item| item.to_string()),
        }
    }
    pub fn data_id_iter(self) -> impl Iterator<Item = String> {
        match self {
            Character::Yuyuko => yuyuko::ObjectData::iter().map(|item| item.to_string()),
        }
    }
    pub fn graphic_name_iter(self) -> impl Iterator<Item = String> {
        match self {
            Character::Yuyuko => yuyuko::Graphic::iter().map(|item| item.file_name()),
        }
    }

    pub fn load_data(self, ctx: &mut Context, assets: &mut Assets) -> GameResult<CharacterData> {
        match self {
            Character::Yuyuko => Ok(CharacterData::Yuyuko(Rc::new(
                Data::<YuyukoType>::new_with_path(
                    ctx,
                    assets,
                    PathBuf::from("./resources/yuyuko.json"),
                )?,
            ))),
        }
    }
}
// TODO TEST
impl CharacterData {
    pub fn make_character(&self) -> CharacterBehavior {
        match self {
            CharacterData::Yuyuko(data) => Player::new((**data).clone()).into(),
        }
    }
    pub fn is_for(&self, character: Character) -> bool {
        matches!(
            (self, character),
            (CharacterData::Yuyuko(..), Character::Yuyuko)
        )
    }
}
