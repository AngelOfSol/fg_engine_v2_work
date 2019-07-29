use ggez::graphics;
use ggez::graphics::{DrawParam, Image};
use ggez::{Context, GameResult};

use std::path::Path;

use crate::typedefs::graphics::{Matrix4, Vec3};

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
    pub fn give(&self, ctx: &Context) -> f32 {
        let draw_size = graphics::drawable_size(ctx);
        (self.width() - draw_size.0) / 2.0
    }

    pub fn draw(&self, ctx: &mut Context, world: Matrix4) -> GameResult<()> {
        graphics::set_transform(
            ctx,
            world * Matrix4::new_translation(&Vec3::new(-self.give(ctx) * 0.0, -200.0, 0.0)),
        );
        graphics::apply_transformations(ctx)?;
        graphics::draw(ctx, &self.image, DrawParam::default())?;

        Ok(())
    }
}
