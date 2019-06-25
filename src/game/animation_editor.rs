use ggez::{Context, GameResult};

use crate::animation::Animation;
use crate::timeline::AtTime;

use crate::assets::Assets;

pub struct AnimationEditor {
    pub frame: usize,
}

impl AnimationEditor {
    pub fn new() -> Self {
        Self { frame: 0 }
    }

    pub fn update(&mut self) -> GameResult<()> {
        self.frame += 1;
        Ok(())
    }

    pub fn draw(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        animation: &Animation,
    ) -> GameResult<()> {
        Animation::draw(
            ctx,
            assets,
            animation,
            self.frame % animation.frames.duration(),
            nalgebra::Translation3::new(100.0, 100.0, 0.0).to_homogeneous(),
        )?;

        Ok(())
    }
}