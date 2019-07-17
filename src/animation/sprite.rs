mod file;
mod ui;

use ggez::error::GameError;
use ggez::graphics;
use ggez::graphics::{Color, DrawMode, DrawParam, Mesh, Rect};
use ggez::{Context, GameResult};

use serde::{Deserialize, Serialize};

use crate::assets::Assets;

use std::path::PathBuf;

use crate::typedefs::graphics::{up_dimension, Matrix4, Vec2, Vec3};

pub use file::load_image;
pub use ui::SpriteUi;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sprite {
    pub offset: Vec2,
    pub image: String,
    pub rotation: f32,
    #[serde(default = "default_scale")]
    pub scale: Vec2,
}

fn default_scale() -> Vec2 {
    Vec2::new(1.0, 1.0)
}

impl Sprite {
    pub fn load(
        ctx: &mut Context,
        assets: &mut Assets,
        sprite: &Sprite,
        path: PathBuf,
    ) -> GameResult<()> {
        file::load(ctx, assets, sprite, path)
    }
    pub fn save(
        ctx: &mut Context,
        assets: &mut Assets,
        sprite: &Sprite,
        path: PathBuf,
    ) -> GameResult<()> {
        file::save(ctx, assets, sprite, path)
    }

    pub fn load_image(&self, ctx: &mut Context, assets: &mut Assets) -> GameResult<()> {
        if assets.images.contains_key(&self.image) {
            return Ok(());
        }
        file::load_image(self.image.clone(), &self.image, ctx, assets)
    }
    pub fn new<S: Into<String>>(path: S) -> Self {
        Self {
            offset: nalgebra::zero(),
            image: path.into(),
            rotation: 0.0,
            scale: default_scale(),
        }
    }

    pub fn draw_ex(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        world: Matrix4,
        debug: bool,
    ) -> GameResult<()> {
        let image = assets
            .images
            .get(&self.image)
            .ok_or_else(|| GameError::ResourceNotFound(self.image.clone(), Vec::new()))?;

        let image_offset = Matrix4::new_translation(&Vec3::new(
            -f32::from(image.width()) / 2.0,
            -f32::from(image.height()) / 2.0,
            0.0,
        ));

        let sprite_scale = Matrix4::new_nonuniform_scaling(&up_dimension(self.scale));

        let sprite_offset = Matrix4::new_translation(&Vec3::new(self.offset.x, self.offset.y, 0.0));

        let transform = world * sprite_scale * image_offset * sprite_offset;

        graphics::set_transform(ctx, transform);
        graphics::apply_transformations(ctx)?;

        graphics::draw(ctx, image, DrawParam::default())?;

        if debug {
            let rectangle = Mesh::new_rectangle(
                ctx,
                DrawMode::stroke(1.0),
                Rect::new(
                    0.0,
                    0.0,
                    f32::from(image.width()),
                    f32::from(image.height()),
                ),
                Color::new(1.0, 0.0, 0.0, 1.0),
            )?;
            graphics::draw(ctx, &rectangle, DrawParam::default())?;
        }
        graphics::set_transform(ctx, Matrix4::identity());
        graphics::apply_transformations(ctx)?;

        Ok(())
    }
    pub fn draw_debug(&self, ctx: &mut Context, assets: &Assets, world: Matrix4) -> GameResult<()> {
        self.draw_ex(ctx, assets, world, true)
    }

    pub fn draw(&self, ctx: &mut Context, assets: &Assets, world: Matrix4) -> GameResult<()> {
        self.draw_ex(ctx, assets, world, false)
    }

    pub fn rename<S: Into<String>>(&mut self, new_name: S, assets: &mut Assets) {
        ui::rename(self, new_name, assets)
    }
}
