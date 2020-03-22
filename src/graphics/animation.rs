mod file;

use super::sprite::{Sprite, SpriteVersioned};
use super::BlendMode;
use crate::assets::Assets;
use crate::timeline::{AtTime, Timeline};
use crate::typedefs::graphics::Matrix4;
use ggez::graphics;
use ggez::{Context, GameResult};
use serde::{Deserialize, Deserializer, Serialize};
use std::path::PathBuf;

pub fn deserialize_versioned_frames<'de, D>(deserializer: D) -> Result<Timeline<Sprite>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Timeline::<SpriteVersioned>::deserialize(deserializer)?
        .into_iter()
        .map(|(sprite, time)| (sprite.to_modern(), time))
        .collect())
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Animation {
    pub name: String,
    #[serde(deserialize_with = "deserialize_versioned_frames")]
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
            sprite.draw(ctx, assets, world, 0)
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
            sprite.draw_debug(ctx, assets, world, 0)?
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
        let (image, remaining) = self.frames.at_time_with_remaining(time);
        image.draw(ctx, assets, world, remaining)
    }
    pub fn draw_at_time_debug(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        time: usize,
        world: Matrix4,
    ) -> GameResult<()> {
        graphics::set_blend_mode(ctx, self.blend_mode.into())?;
        let (image, remaining) = self.frames.at_time_with_remaining(time);
        image.draw_debug(ctx, assets, world, remaining)
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
