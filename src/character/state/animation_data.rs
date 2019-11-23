use crate::assets::Assets;
use crate::graphics::Animation;
use crate::timeline::AtTime;
use crate::typedefs::graphics::{Matrix4, Vec2, Vec3};
use ggez::{Context, GameResult};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AnimationData {
    pub animation: Animation,
    pub delay: usize,
    pub offset: Vec2,
    pub scale: Vec2,
}

impl AnimationData {
    pub fn new(animation: Animation) -> Self {
        Self {
            animation,
            delay: 0,
            offset: nalgebra::zero(),
            scale: Vec2::new(1.0, 1.0),
        }
    }

    pub fn name(&self) -> &str {
        &self.animation.name
    }

    pub fn duration(&self) -> usize {
        self.animation.frames.duration()
    }

    pub fn draw_at_time(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        time: usize,
        world: Matrix4,
    ) -> GameResult<()> {
        if time >= self.delay && time - self.delay < self.duration() {
            let transform = Matrix4::new_translation(&Vec3::new(self.offset.x, self.offset.y, 0.0))
                * Matrix4::new_nonuniform_scaling(&Vec3::new(self.scale.x, self.scale.y, 1.0));
            self.animation
                .draw_at_time(ctx, assets, time - self.delay, world * transform)
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
    ) -> GameResult<()> {
        if time >= self.delay && time - self.delay < self.duration() {
            let transform = Matrix4::new_translation(&Vec3::new(self.offset.x, self.offset.y, 0.0))
                * Matrix4::new_nonuniform_scaling(&Vec3::new(self.scale.x, self.scale.y, 1.0));
            self.animation
                .draw_at_time_debug(ctx, assets, time - self.delay, world * transform)
        } else {
            Ok(())
        }
    }
}
