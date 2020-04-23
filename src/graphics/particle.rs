mod file;
pub mod version;

use crate::assets::Assets;
use crate::graphics::animation::{self, Animation};
use crate::graphics::keyframe::Modifiers;
use crate::typedefs::graphics::Matrix4;
use ggez::{Context, GameResult};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub type Particle = ParticleV1;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ParticleV1 {
    #[serde(deserialize_with = "animation::version::vec::deserialize")]
    pub animations: Vec<Animation>,
    pub modifiers: Modifiers,
}

impl Particle {
    pub fn to_modern(self) -> Self {
        self
    }

    pub fn new() -> Self {
        Self {
            animations: vec![],
            modifiers: Modifiers::default(),
        }
    }

    pub fn duration(&self) -> usize {
        self.animations
            .iter()
            .map(|item| item.delay + item.duration())
            .fold(0, std::cmp::max)
    }
    pub fn draw_frame(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        frame: usize,
        world: Matrix4,
    ) -> GameResult<()> {
        for animation in self.animations.iter() {
            animation.draw_frame(ctx, assets, frame, world)?
        }

        Ok(())
    }

    pub fn draw_at_time(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        time: usize,
        world: Matrix4,
    ) -> GameResult<()> {
        let transform = self.modifiers.matrix_at_time(time);
        let world = world * transform;
        for animation in self.animations.iter() {
            animation.draw_at_time(ctx, assets, time, world)?
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
        let transform = self.modifiers.matrix_at_time(time);
        let world = world * transform;
        for animation in self.animations.iter() {
            animation.draw_at_time_debug(ctx, assets, time, world)?
        }

        Ok(())
    }

    pub fn load_from_json(
        ctx: &mut Context,
        assets: &mut Assets,
        path: PathBuf,
    ) -> GameResult<Self> {
        file::load_from_json(ctx, assets, path)
    }
    pub fn save(
        ctx: &mut Context,
        assets: &mut Assets,
        particle: &Self,
        path: PathBuf,
    ) -> GameResult<()> {
        file::save(ctx, assets, particle, path)
    }
    pub fn load(
        ctx: &mut Context,
        assets: &mut Assets,
        particle: &mut Self,
        path: PathBuf,
    ) -> GameResult<()> {
        file::load(ctx, assets, particle, path)
    }
}
