pub mod animation_data;
pub mod bullet_spawn_data;
pub mod cancel_set;
pub mod flags;
pub mod hitbox_set;
pub mod particle_spawn_data;

mod file;

use crate::assets::Assets;
use crate::timeline::{AtTime, Timeline};
use crate::typedefs::graphics::Matrix4;
use crate::typedefs::FgSerializable;
use crate::typedefs::{HashId, StateId};
pub use animation_data::AnimationData;
pub use bullet_spawn_data::BulletSpawn;
pub use cancel_set::{CancelSet, MoveType};
pub use flags::{Flags, MagicHittable, MeleeHittable, MovementData};
use ggez::{Context, GameResult};
pub use hitbox_set::{AttackData, HitboxSet};
pub use particle_spawn_data::ParticleSpawn;
use serde::{Deserialize, Serialize};
use std::cmp;
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CharacterState<Id, ParticleId, BulletSpawnInfo>
where
    Id: HashId,
{
    pub animations: Vec<AnimationData>,
    pub flags: Timeline<Flags>,
    pub cancels: Timeline<CancelSet<Id>>,
    pub hitboxes: Timeline<HitboxSet>,
    #[serde(default)]
    pub particles: Vec<ParticleSpawn<ParticleId>>,
    #[serde(default)]
    pub bullets: Vec<BulletSpawnInfo>,
    #[serde(default = "default_move_type")]
    pub state_type: MoveType,
    #[serde(default)]
    pub on_expire_state: Id,
    #[serde(default)]
    pub minimum_spirit_required: i32,
}
pub type EditorCharacterState = CharacterState<String, String, BulletSpawn>;
fn default_move_type() -> MoveType {
    MoveType::Idle
}
impl<Id: StateId, ParticleId: StateId, BulletSpawnInfo: FgSerializable>
    CharacterState<Id, ParticleId, BulletSpawnInfo>
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
                animation.draw_at_time(ctx, assets, time, world)?
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
                .filter(|item| item.animation.blend_mode == crate::graphics::BlendMode::Alpha)
            {
                animation.draw_at_time(ctx, assets, time, world)?
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
                animation.draw_at_time_debug(ctx, assets, time, world)?
            }
        }
        Ok(())
    }
}

impl CharacterState<String, String, BulletSpawn> {
    pub fn new() -> Self {
        Self {
            animations: vec![],
            flags: vec![(Flags::new(), 1)],
            cancels: vec![(CancelSet::new(), 1)],
            hitboxes: vec![(HitboxSet::new(), 1)],
            state_type: default_move_type(),
            on_expire_state: "stand".to_owned(),
            minimum_spirit_required: 0,
            particles: Vec::new(),
            bullets: Vec::new(),
        }
    }
}
