#[macro_use]
pub mod generic_character;
pub mod character;
pub mod world;
pub mod yuyuko;

use crate::game_match::sounds::{GlobalSound, SoundList};
use crate::game_match::{FlashType, PlayArea, UiElements};
use crate::graphics::animation_group::AnimationGroup;
use crate::hitbox::PositionedHitbox;
use crate::{assets::Assets, character::components::AttackInfo};
use crate::{character::state::components::GlobalGraphic, game_object::state::BulletTier};
use character::{data::Data, Player};
use enum_dispatch::enum_dispatch;
use fg_datastructures::{
    math::{collision, graphics},
    roster::RosterCharacter,
};
use fg_input::{Facing, InputState};
pub use generic_character::*;
use ggez::{Context, GameResult};
use hecs::Entity;
use hit_info::{ComboEffect, HitEffect, HitResult, HitType, Source};
use rodio::Device;
use std::borrow::Cow;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;
use yuyuko::YuyukoType;

#[enum_dispatch]
pub enum CharacterBehavior {
    YuyukoPlayer(Player<YuyukoType>),
}

#[derive(Clone)]
pub enum CharacterData {
    Yuyuko(Rc<Data<YuyukoType>>),
}

pub fn load_data(
    value: RosterCharacter,
    ctx: &mut Context,
    assets: &mut Assets,
) -> GameResult<CharacterData> {
    match value {
        RosterCharacter::Yuyuko => Ok(CharacterData::Yuyuko(Rc::new(
            Data::<YuyukoType>::new_with_path(
                ctx,
                assets,
                PathBuf::from("./resources/yuyuko.json"),
            )?,
        ))),
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
