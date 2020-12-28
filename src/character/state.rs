mod cancel_set;
mod flags;
mod global_graphics;
mod hitbox_set;
mod sound_play_info;

mod file;

pub mod components {
    pub use super::cancel_set::*;
    pub use super::flags::*;
    pub use super::global_graphics::*;
    pub use super::hitbox_set::*;
    pub use super::sound_play_info::*;
}
use crate::graphics::Animation;
use crate::timeline::Timeline;
use crate::typedefs::graphics::Matrix4;
use crate::{
    assets::{Assets, ValueAlpha},
    game_object::constructors::Constructor,
};
use cancel_set::{CancelSet, CommandType};
use flags::Flags;
use ggez::{Context, GameResult};
use hitbox_set::HitboxSet;
use inspect_design::Inspect;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use sound_play_info::SoundPlayInfo;
use std::hash::Hash;
use std::path::PathBuf;
use std::{
    cmp,
    fmt::{Display, Formatter},
};

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Eq, Inspect)]
pub struct SpawnerInfo {
    pub frame: usize,
    pub data: Vec<Constructor>,
}

#[derive(Clone, Deserialize, Serialize, Inspect, Default)]
pub struct State<Id, AttackId, SoundType> {
    #[tab = "Animation"]
    pub animations: Vec<Animation>,
    #[tab = "Flags"]
    pub flags: Timeline<Flags>,
    #[serde(bound(
        serialize = "CancelSet<Id>: Serialize",
        deserialize = "CancelSet<Id>: Deserialize<'de>"
    ))]
    #[inspect_mut_bounds = "Id: Clone"]
    #[tab = "Cancels"]
    pub cancels: Timeline<CancelSet<Id>>,
    #[inspect_mut_bounds = "AttackId: Clone"]
    #[tab = "Hitboxes"]
    pub hitboxes: Timeline<HitboxSet<AttackId>>,
    #[tab = "Spawns"]
    pub spawns: Vec<SpawnerInfo>,
    #[tab = "Spawns"]
    pub sounds: Vec<SoundPlayInfo<SoundType>>,
    #[serde(default = "default_move_type")]
    pub state_type: CommandType,
    #[serde(alias = "on_expire_state")]
    pub on_expire: OnExpire<Id>,
}

#[derive(Clone, Deserialize, Serialize, Inspect, Default, PartialEq, Eq, Debug)]
pub struct OnExpire<Id> {
    pub state_id: Id,
    pub frame: usize,
}

impl<Id: Display> Display for OnExpire<Id> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.frame, self.state_id)
    }
}

impl<Id> From<Id> for OnExpire<Id> {
    fn from(value: Id) -> Self {
        Self {
            state_id: value,
            frame: 0,
        }
    }
}

impl<Id, AttackId, SoundType> PartialEq for State<Id, AttackId, SoundType>
where
    Id: PartialEq,
    AttackId: PartialEq,
    CancelSet<Id>: PartialEq,
{
    fn eq(&self, rhs: &Self) -> bool {
        self.animations.eq(&rhs.animations)
            && self.flags.eq(&rhs.flags)
            && self.cancels.eq(&rhs.cancels)
            && self.hitboxes.eq(&rhs.hitboxes)
            && self.state_type.eq(&rhs.state_type)
            && self.on_expire.eq(&rhs.on_expire)
    }
}
impl<Id, AttackId, SoundType> Eq for State<Id, AttackId, SoundType>
where
    Id: PartialEq,
    AttackId: PartialEq,
    CancelSet<Id>: PartialEq,
{
}

impl<Id, AttackId, SoundType> std::fmt::Debug for State<Id, AttackId, SoundType>
where
    Id: std::fmt::Debug,
    AttackId: std::fmt::Debug,
    CancelSet<Id>: std::fmt::Debug,
{
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let mut builder = fmt.debug_struct("State");
        let _ = builder.field("flags", &self.flags);
        let _ = builder.field("cancels", &self.cancels);
        let _ = builder.field("hitboxes", &self.hitboxes);
        let _ = builder.field("state_type", &self.state_type);
        let _ = builder.field("on_expire_state", &self.on_expire);
        builder.finish()
    }
}

pub type EditorCharacterState = State<String, String, String>;
fn default_move_type() -> CommandType {
    CommandType::Idle
}
impl<
        Id: Serialize + DeserializeOwned + Eq + Hash + Default,
        AttackId: Serialize + DeserializeOwned + Default,
        SoundType: Serialize + DeserializeOwned + Default,
    > State<Id, AttackId, SoundType>
{
    pub fn load_from_json(
        ctx: &mut Context,
        assets: &mut Assets,
        path: PathBuf,
    ) -> GameResult<Self> {
        file::load_from_json(ctx, assets, path)
    }
    pub fn load(
        ctx: &mut Context,
        assets: &mut Assets,
        state: &mut Self,
        name: &str,
        path: PathBuf,
    ) -> GameResult<()> {
        file::load(ctx, assets, state, name, path)
    }
    pub fn save(
        ctx: &mut Context,
        assets: &mut Assets,
        state: &Self,
        path: PathBuf,
    ) -> GameResult<()> {
        file::save(ctx, assets, state, path)
    }

    pub fn duration(&self) -> usize {
        self.animations
            .iter()
            .map(|item| item.delay + item.duration())
            .fold(0, cmp::max)
    }

    pub fn draw_at_time(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        time: usize,
        world: Matrix4,
    ) -> GameResult<()> {
        if time < self.duration() {
            for animation in self.animations.iter() {
                animation.draw_at_time(
                    ctx,
                    assets,
                    time,
                    world,
                    ValueAlpha {
                        value: 1.0,
                        alpha: 1.0,
                    },
                )?
            }
        }
        Ok(())
    }
    pub fn draw_shadow_at_time(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        time: usize,
        world: Matrix4,
    ) -> GameResult<()> {
        if time < self.duration() {
            for animation in self
                .animations
                .iter()
                .filter(|item| item.blend_mode == crate::graphics::BlendMode::Alpha)
            {
                animation.draw_at_time(
                    ctx,
                    assets,
                    time,
                    world,
                    ValueAlpha {
                        value: 1.0,
                        alpha: 1.0,
                    },
                )?
            }
        }
        Ok(())
    }
    pub fn draw_at_time_debug(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        time: usize,
        world: Matrix4,
    ) -> GameResult<()> {
        if time < self.duration() {
            for animation in self.animations.iter() {
                animation.draw_at_time_debug(
                    ctx,
                    assets,
                    time,
                    world,
                    ValueAlpha {
                        value: 1.0,
                        alpha: 1.0,
                    },
                )?
            }
        }
        Ok(())
    }
}

impl EditorCharacterState {
    pub fn new() -> Self {
        Self {
            animations: vec![],
            flags: Timeline::with_data(vec![(0, Flags::new())], 1).unwrap(),
            cancels: Timeline::with_data(vec![(0, CancelSet::new())], 1).unwrap(),
            hitboxes: Timeline::with_data(vec![(0, HitboxSet::new())], 1).unwrap(),
            state_type: default_move_type(),
            on_expire: OnExpire::default(),
            spawns: Vec::new(),
            sounds: Vec::new(),
        }
    }
}
