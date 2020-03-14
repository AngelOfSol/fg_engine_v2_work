use crate::typedefs::graphics::{Matrix4, Vec3};
use ggez::graphics;
use ggez::graphics::{DrawParam, Image};
use ggez::{Context, GameResult};
use std::path::Path;

#[derive(Clone)]
pub struct Stage {
    image: Image,
}

impl Stage {
    pub fn new<P: AsRef<Path>>(ctx: &mut Context, path: P) -> GameResult<Self> {
        Ok(Self {
            image: Image::new(ctx, path)?,
        })
    }
    pub fn width(&self) -> f32 {
        f32::from(self.image.width())
    }

    pub fn draw(&self, ctx: &mut Context, world: Matrix4) -> GameResult<()> {
        graphics::set_transform(
            ctx,
            world * Matrix4::new_translation(&Vec3::new(-self.width() / 2.0, -860.0, 0.0)),
        );
        graphics::apply_transformations(ctx)?;
        graphics::draw(ctx, &self.image, DrawParam::default())?;

        Ok(())
    }
}
