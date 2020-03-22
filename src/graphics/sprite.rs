mod file;
pub mod version;

use super::keyframe::Modifiers;
use crate::assets::Assets;
use crate::typedefs::graphics::{Matrix4, Vec3};
use ggez::graphics;
use ggez::graphics::{Color, DrawMode, DrawParam, Image, Mesh, Rect};
use ggez::{Context, GameResult};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

pub type Sprite = SpriteV1;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpriteV1 {
    #[serde(default)]
    #[serde(skip)]
    pub image: Option<Image>,

    pub modifiers: Modifiers,
}

impl Sprite {
    fn to_modern(self) -> Self {
        self
    }

    pub fn save(
        ctx: &mut Context,
        assets: &mut Assets,
        sprite: &Self,
        path: PathBuf,
    ) -> GameResult<()> {
        file::save(ctx, assets, sprite, path)
    }

    pub fn load_new<P: AsRef<Path>>(
        ctx: &mut Context,
        assets: &mut Assets,
        path: P,
    ) -> GameResult<Self> {
        let mut sprite = Sprite::new();
        Sprite::load(ctx, assets, &mut sprite, path)?;
        Ok(sprite)
    }

    pub fn draw_ex(
        &self,
        ctx: &mut Context,
        _assets: &Assets,
        world: Matrix4,
        time: usize,
        debug: bool,
    ) -> GameResult<()> {
        let image = self.image.as_ref().unwrap();

        let image_offset = Matrix4::new_translation(&Vec3::new(
            -f32::from(image.width()) / 2.0,
            -f32::from(image.height()) / 2.0,
            0.0,
        ));

        let sprite_transform = self.modifiers.matrix_at_time(time);

        let transform = world * sprite_transform * image_offset;

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
    pub fn draw_debug(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        world: Matrix4,
        time: usize,
    ) -> GameResult<()> {
        self.draw_ex(ctx, assets, world, time, true)
    }

    pub fn draw(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        world: Matrix4,
        time: usize,
    ) -> GameResult<()> {
        self.draw_ex(ctx, assets, world, time, false)
    }
    pub fn new() -> Self {
        Self {
            image: None,
            modifiers: Modifiers::new(),
        }
    }

    pub fn load<P: AsRef<Path>>(
        ctx: &mut Context,
        assets: &mut Assets,
        sprite: &mut Sprite,
        path: P,
    ) -> GameResult<()> {
        file::load(ctx, assets, sprite, PathBuf::from(path.as_ref()))
    }
}
