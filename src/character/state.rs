mod cancel_set;
mod flags;
mod hitbox_set;
mod particle_spawn_data;
mod sound_play_info;

mod file;

pub mod components {
    pub use super::cancel_set::*;
    pub use super::flags::*;
    pub use super::hitbox_set::*;
    pub use super::particle_spawn_data::*;
    pub use super::sound_play_info::*;
}
use crate::graphics::{self, Animation};
use crate::timeline::{AtTime, Timeline};
use crate::typedefs::graphics::Matrix4;
use crate::{
    assets::{Assets, ValueAlpha},
    game_object::constructors::Constructor,
};
use cancel_set::{CancelSet, MoveType};
use flags::Flags;
use ggez::{Context, GameResult};
use hitbox_set::HitboxSet;
use particle_spawn_data::ParticleSpawn;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use sound_play_info::SoundPlayInfo;
use std::cmp;
use std::hash::Hash;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct SpawnerInfo {
    pub frame: usize,
    pub data: Vec<Constructor>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct State<Id, ParticleId, AttackId, SoundType> {
    #[serde(deserialize_with = "graphics::animation::version::vec::deserialize")]
    pub animations: Vec<Animation>,
    pub flags: Timeline<Flags>,
    #[serde(bound(
        serialize = "CancelSet<Id>: Serialize",
        deserialize = "CancelSet<Id>: Deserialize<'de>"
    ))]
    pub cancels: Timeline<CancelSet<Id>>,
    pub hitboxes: Timeline<HitboxSet<AttackId>>,
    #[serde(default)]
    pub particles: Vec<ParticleSpawn<ParticleId>>,
    #[serde(default)]
    pub spawns: Vec<SpawnerInfo>,

    #[serde(default)]
    pub sounds: Vec<SoundPlayInfo<SoundType>>,
    #[serde(default = "default_move_type")]
    pub state_type: MoveType,
    #[serde(default)]
    pub on_expire_state: Id,
    #[serde(default)]
    pub minimum_spirit_required: i32,
    #[serde(default)]
    pub minimum_meter_required: i32,
}

impl<Id, ParticleId, AttackId, SoundType> PartialEq for State<Id, ParticleId, AttackId, SoundType>
where
    Id: PartialEq,
    ParticleId: PartialEq,
    AttackId: PartialEq,
    CancelSet<Id>: PartialEq,
{
    fn eq(&self, rhs: &Self) -> bool {
        self.animations.eq(&rhs.animations)
            && self.flags.eq(&rhs.flags)
            && self.cancels.eq(&rhs.cancels)
            && self.hitboxes.eq(&rhs.hitboxes)
            && self.particles.eq(&rhs.particles)
            && self.state_type.eq(&rhs.state_type)
            && self.on_expire_state.eq(&rhs.on_expire_state)
            && self
                .minimum_spirit_required
                .eq(&rhs.minimum_spirit_required)
    }
}
impl<Id, ParticleId, AttackId, SoundType> Eq for State<Id, ParticleId, AttackId, SoundType>
where
    Id: PartialEq,
    ParticleId: PartialEq,
    AttackId: PartialEq,
    CancelSet<Id>: PartialEq,
{
}

impl<Id, ParticleId, AttackId, SoundType> std::fmt::Debug
    for State<Id, ParticleId, AttackId, SoundType>
where
    Id: std::fmt::Debug,
    ParticleId: std::fmt::Debug,
    AttackId: std::fmt::Debug,
    CancelSet<Id>: std::fmt::Debug,
{
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let mut builder = fmt.debug_struct("State");
        let _ = builder.field("flags", &self.flags);
        let _ = builder.field("cancels", &self.cancels);
        let _ = builder.field("hitboxes", &self.hitboxes);
        let _ = builder.field("particles", &self.particles);
        let _ = builder.field("state_type", &self.state_type);
        let _ = builder.field("on_expire_state", &self.on_expire_state);
        let _ = builder.field("minimum_spirit_required", &self.minimum_spirit_required);
        builder.finish()
    }
}

pub type EditorCharacterState = State<String, String, String, String>;
fn default_move_type() -> MoveType {
    MoveType::Idle
}
impl<
        Id: Serialize + DeserializeOwned + Eq + Hash + Default,
        ParticleId: Serialize + DeserializeOwned + Default,
        AttackId: Serialize + DeserializeOwned + Default,
        SoundType: Serialize + DeserializeOwned + Default,
    > State<Id, ParticleId, AttackId, SoundType>
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

    pub fn fix_duration(&mut self) {
        if self.duration() > 0 {
            self.flags.fix_duration(self.duration());
            self.cancels.fix_duration(self.duration());
            self.hitboxes.fix_duration(self.duration());
        }
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
            flags: vec![(Flags::new(), 1)],
            cancels: vec![(CancelSet::new(), 1)],
            hitboxes: vec![(HitboxSet::new(), 1)],
            state_type: default_move_type(),
            on_expire_state: "".to_owned(),
            minimum_spirit_required: 0,
            minimum_meter_required: 0,
            particles: Vec::new(),
            spawns: Vec::new(),
            sounds: Vec::new(),
        }
    }
}
