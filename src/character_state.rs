mod animation_data;

use crate::animation::Animation;
use crate::assets::Assets;
use crate::timeline::{AtTime, Timeline};

use crate::imgui_extra::UiExtensions;

use imgui::im_str;

use ggez::graphics;
use ggez::{Context, GameResult};
use serde::{Deserialize, Serialize};

use std::convert::TryFrom;


use std::fs::File;
use std::path::Path;

use animation_data::AnimationData;

#[derive(Debug, Deserialize,PartialEq, Serialize)]
pub struct CharacterState {
    animations: Vec<AnimationData>,
    pub duration: usize,
}



impl CharacterState {
    pub fn new() -> Self {
        Self {
            animations: vec![],
            duration: 1,
        }
    }

    pub fn draw_at_time(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        time: usize,
        world: nalgebra::Matrix4<f32>,
    ) -> GameResult<()> {
        for animation in self.animations.iter() {
           animation.draw_at_time(ctx, assets, time, world)?
        }
        Ok(())
    }
}