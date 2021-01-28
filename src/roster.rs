#[macro_use]
pub mod generic_character;
pub mod character;
pub mod yuyuko;

use crate::game_match::sounds::{GlobalSound, SoundList};
use crate::game_match::{FlashType, PlayArea, UiElements};
use crate::graphics::animation_group::AnimationGroup;
use crate::hitbox::PositionedHitbox;
use crate::{assets::Assets, character::components::AttackInfo};
use crate::{character::state::components::GlobalGraphic, game_object::state::BulletTier};
use character::{data::Data, Player};
use enum_dispatch::enum_dispatch;
use fg_datastructures::math::{collision, graphics};
use fg_input::{Facing, InputState};
pub use generic_character::*;
use ggez::{Context, GameResult};
use hecs::Entity;
use hit_info::{ComboEffect, HitEffect, HitResult, HitType, Source};
use rodio::Device;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;
use strum::{Display, EnumCount, EnumIter};
use yuyuko::YuyukoType;

#[enum_dispatch]
pub enum CharacterBehavior {
    YuyukoPlayer(Player<YuyukoType>),
}

#[derive(Clone)]
pub enum CharacterData {
    Yuyuko(Rc<Data<YuyukoType>>),
}

#[derive(Debug, Copy, Clone, PartialEq, EnumIter, Display, EnumCount, Serialize, Deserialize)]
pub enum RosterCharacter {
    Yuyuko,
}

impl Default for RosterCharacter {
    fn default() -> Self {
        Self::Yuyuko
    }
}

impl RosterCharacter {
    pub fn load_data(self, ctx: &mut Context, assets: &mut Assets) -> GameResult<CharacterData> {
        match self {
            RosterCharacter::Yuyuko => Ok(CharacterData::Yuyuko(Rc::new(
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
    pub fn is_for(&self, character: RosterCharacter) -> bool {
        matches!(
            (self, character),
            (CharacterData::Yuyuko(..), RosterCharacter::Yuyuko)
        )
    }
}
