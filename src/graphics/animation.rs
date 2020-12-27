mod file;

use super::keyframe::{KeyframeExt, Modifiers};
use super::sprite::Sprite;
use super::BlendMode;
use crate::assets::{Assets, ValueAlpha};
use crate::timeline::Timeline;
use crate::typedefs::graphics::Matrix4;
use ggez::graphics;
use ggez::{Context, GameResult};
use inspect_design::Inspect;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub type Animation = AnimationV1;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default, Inspect)]
pub struct AnimationV1 {
    pub name: String,
    pub frames: Timeline<Sprite>,
    pub blend_mode: BlendMode,
    #[serde(default)]
    pub modifiers: Modifiers,
    #[serde(default)]
    pub delay: usize,
}

impl Animation {
    pub fn duration(&self) -> usize {
        self.frames.duration()
    }

    pub fn get_path_to_image(&self, idx: usize) -> String {
        format!("{}-{:03}.png", &self.name, idx)
    }
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            frames: Timeline::new(1),
            blend_mode: BlendMode::Alpha,
            modifiers: Modifiers::new(),
            delay: 0,
        }
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
        animation: &Animation,
        path: PathBuf,
    ) -> GameResult<()> {
        file::save(ctx, assets, animation, path)
    }

    pub fn draw_frame(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        index: usize,
        world: Matrix4,
    ) -> GameResult<()> {
        let data = self.frames.get(index);
        if let Some((_, sprite)) = data {
            graphics::set_blend_mode(ctx, self.blend_mode.into())?;

            sprite.draw(
                ctx,
                assets,
                world,
                0,
                ValueAlpha {
                    value: 1.0,
                    alpha: 1.0,
                },
            )
        } else {
            Ok(())
        }
    }

    pub fn draw_every_frame(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        world: Matrix4,
    ) -> GameResult<()> {
        graphics::set_blend_mode(ctx, self.blend_mode.into())?;
        for sprite in self.frames.iter() {
            sprite.draw_debug(
                ctx,
                assets,
                world,
                0,
                ValueAlpha {
                    value: 1.0,
                    alpha: 1.0,
                },
            )?
        }

        Ok(())
    }

    pub fn draw_at_time(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        time: usize,
        world: Matrix4,
        constants: ValueAlpha,
    ) -> GameResult<()> {
        graphics::set_blend_mode(ctx, self.blend_mode.into())?;

        let time = if let Some(time) = time.checked_sub(self.delay) {
            time
        } else {
            return Ok(());
        };

        if let Some((_, image)) = self.frames.get(time) {
            let transform = self.modifiers.get_matrix(time);
            image.draw(
                ctx,
                assets,
                world * transform,
                time,
                ValueAlpha {
                    value: self.modifiers.value.get_eased(time).unwrap_or(1.0) * constants.value,
                    alpha: self.modifiers.alpha.get_eased(time).unwrap_or(1.0) * constants.value,
                },
            )
        } else {
            Ok(())
        }
    }
    pub fn draw_at_time_debug(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        time: usize,
        world: Matrix4,
        constants: ValueAlpha,
    ) -> GameResult<()> {
        graphics::set_blend_mode(ctx, self.blend_mode.into())?;

        let time = if let Some(time) = time.checked_sub(self.delay) {
            time
        } else {
            return Ok(());
        };

        if let Some((_, image)) = self.frames.get(time) {
            let transform = self.modifiers.get_matrix(time);
            image.draw_debug(
                ctx,
                assets,
                world * transform,
                time,
                ValueAlpha {
                    value: self.modifiers.value.get_eased(time).unwrap_or(1.0) * constants.value,
                    alpha: self.modifiers.alpha.get_eased(time).unwrap_or(1.0) * constants.value,
                },
            )
        } else {
            Ok(())
        }
    }

    pub fn load(
        ctx: &mut Context,
        assets: &mut Assets,
        animation: &mut Animation,
        path: PathBuf,
    ) -> GameResult<()> {
        file::load(ctx, assets, animation, path)
    }
}
