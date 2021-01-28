mod file;
use crate::assets::{Assets, ValueAlpha};
use crate::graphics::animation::Animation;
use crate::graphics::keyframe::Modifiers;
use fg_datastructures::math::graphics::Matrix4;
use ggez::{Context, GameResult};
use inspect_design::Inspect;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::keyframe::KeyframeExt;

pub type AnimationGroup = AnimationGroupV1;

#[derive(Debug, Deserialize, Serialize, Clone, Default, Inspect)]
pub struct AnimationGroupV1 {
    pub animations: Vec<Animation>,
    pub modifiers: Modifiers,
}

impl AnimationGroup {
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
            .fold(1, std::cmp::max)
    }

    pub fn fix_durations(&mut self) {
        let duration = self.duration();
        self.modifiers.set_duration(duration);
    }

    #[allow(dead_code)]
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
        let transform = self.modifiers.get_matrix(time);
        let world = world * transform;
        for animation in self.animations.iter() {
            animation.draw_at_time(
                ctx,
                assets,
                time,
                world,
                ValueAlpha {
                    value: self.modifiers.value.get_eased(time).unwrap_or(1.0),
                    alpha: self.modifiers.alpha.get_eased(time).unwrap_or(1.0),
                },
            )?
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
        let transform = self.modifiers.get_matrix(time);
        let world = world * transform;
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
                    value: self.modifiers.value.get_eased(time).unwrap_or(1.0),
                    alpha: self.modifiers.alpha.get_eased(time).unwrap_or(1.0),
                },
            )?
        }

        Ok(())
    }

    pub fn draw_at_time_debug(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        time: usize,
        world: Matrix4,
        constants: ValueAlpha,
    ) -> GameResult<()> {
        let transform = self.modifiers.get_matrix(time);
        let world = world * transform;
        for animation in self.animations.iter() {
            animation.draw_at_time_debug(
                ctx,
                assets,
                time,
                world,
                ValueAlpha {
                    value: self.modifiers.value.get_eased(time).unwrap_or(1.0) * constants.value,
                    alpha: self.modifiers.alpha.get_eased(time).unwrap_or(1.0) * constants.value,
                },
            )?
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
        animation_group: &Self,
        path: PathBuf,
    ) -> GameResult<()> {
        file::save(ctx, assets, animation_group, path)
    }
    pub fn load(
        ctx: &mut Context,
        assets: &mut Assets,
        animation_group: &mut Self,
        path: PathBuf,
    ) -> GameResult<()> {
        file::load(ctx, assets, animation_group, path)
    }
}
