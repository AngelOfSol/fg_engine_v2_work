use crate::animation::Animation;
use serde::{Deserialize, Serialize};

use crate::assets::Assets;
use crate::timeline::{AtTime};


use imgui::im_str;

use ggez::graphics;
use ggez::{Context, GameResult};

use std::convert::TryFrom;


use std::fs::File;
use std::path::Path;


#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct AnimationData {
    pub animation: Animation,
    pub delay: usize,
    pub offset: nalgebra::Vector2<f32>,
    pub scale: nalgebra::Vector2<f32>,
}

impl AnimationData {
    pub fn new(animation: Animation) -> Self {
        Self {
            animation,
            delay: 0,
            offset: nalgebra::zero(),
            scale: nalgebra::Vector2::new(1.0, 1.0),
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
        world: nalgebra::Matrix4<f32>,
    ) -> GameResult<()> {
        if time >= self.delay && time < self.duration() {
            let transform = 
            nalgebra::Matrix4::new_translation(&nalgebra::Vector3::new(self.offset.x, self.offset.y, 0.0)) *
            nalgebra::Matrix4::new_nonuniform_scaling(&nalgebra::Vector3::new(self.scale.x, self.scale.y, 1.0));
            self.animation.draw_at_time(ctx, assets, time - self.delay, world * transform)
        } else { 
            Ok(())
        }
    }
}