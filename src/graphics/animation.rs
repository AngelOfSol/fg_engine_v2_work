mod file;
mod ui;

use super::sprite::Sprite;
use super::BlendMode;
use crate::assets::Assets;
use crate::timeline::{AtTime, Timeline};
use crate::typedefs::graphics::Matrix4;
use ggez::graphics;
use ggez::{Context, GameResult};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
pub use ui::AnimationUi;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Animation {
    pub name: String,
    pub frames: Timeline<Sprite>,
    #[serde(default = "default_blend_mode")]
    pub blend_mode: BlendMode,
}

fn default_blend_mode() -> BlendMode {
    BlendMode::Alpha
}

impl Animation {
    pub fn get_path_to_image(&self, idx: usize) -> String {
        format!("{}-{:03}.png", &self.name, idx)
    }
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            frames: Timeline::new(),
            blend_mode: BlendMode::Alpha,
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
        if let Some((ref sprite, _)) = data {
            graphics::set_blend_mode(ctx, self.blend_mode.into())?;
            sprite.draw(ctx, assets, world)
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
        for sprite in self.frames.iter().map(|(ref sprite, _)| sprite) {
            sprite.draw_debug(ctx, assets, world)?
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
        graphics::set_blend_mode(ctx, self.blend_mode.into())?;
        let image = self.frames.at_time(time);
        image.draw(ctx, assets, world)
    }
    pub fn draw_at_time_debug(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        time: usize,
        world: Matrix4,
    ) -> GameResult<()> {
        graphics::set_blend_mode(ctx, self.blend_mode.into())?;
        let image = self.frames.at_time(time);
        image.draw_debug(ctx, assets, world)
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
