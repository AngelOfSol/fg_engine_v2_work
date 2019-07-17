use crate::graphics::Animation;
use serde::{Deserialize, Serialize};

use crate::assets::Assets;
use crate::timeline::AtTime;

use ggez::{Context, GameResult};

use crate::typedefs::graphics::{Matrix4, Vec2, Vec3};

use crate::imgui_extra::UiExtensions;
use imgui::*;

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
pub struct AnimationDataUi;

impl AnimationDataUi {
    pub fn draw_ui(ui: &Ui<'_>, data: &mut AnimationData) {
        ui.label_text(im_str!("Name"), &im_str!("{}", data.animation.name.clone()));
        let _ = ui.input_whole(im_str!("Delay"), &mut data.delay);

        ui.input_vec2_float(im_str!("Offset"), &mut data.offset);
        ui.input_vec2_float(im_str!("Scale"), &mut data.scale);
    }
}
