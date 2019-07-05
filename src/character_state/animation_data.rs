use crate::animation::Animation;
use serde::{Deserialize, Serialize};

use crate::assets::Assets;
use crate::timeline::AtTime;

use ggez::{Context, GameResult};

use crate::typedefs::graphics::{Matrix4, Vec2, Vec3};

use crate::imgui_extra::UiExtensions;
use imgui::*;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
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
}
pub struct AnimationDataUi;

impl AnimationDataUi {
    pub fn new() -> Self {
        Self
    }

    pub fn draw_ui(&mut self, ui: &Ui<'_>, data: &mut AnimationData) -> GameResult<()> {
        let _ = ui.input_whole(im_str!("Delay"), &mut data.delay);

        if ui
            .collapsing_header(im_str!("Offset"))
            .default_open(true)
            .build()
        {
            ui.input_float(im_str!("X##Offset"), &mut data.offset.x)
                .build();
            ui.input_float(im_str!("Y##Offset"), &mut data.offset.y)
                .build();
        }

        if ui
            .collapsing_header(im_str!("Scale"))
            .default_open(true)
            .build()
        {
            ui.input_float(im_str!("X##Scale"), &mut data.scale.x)
                .build();
            ui.input_float(im_str!("Y##Scale"), &mut data.scale.y)
                .build();
        }
        Ok(())
    }
}
