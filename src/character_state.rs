mod animation_data;

use crate::animation::Animation;
use crate::assets::Assets;
use crate::timeline::{AtTime, Timeline};

use crate::imgui_extra::UiExtensions;

use imgui::*;

use ggez::graphics;
use ggez::{Context, GameResult};
use serde::{Deserialize, Serialize};

use std::convert::TryFrom;


use std::fs::File;
use std::path::Path;

use animation_data::AnimationData;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct CharacterState {
    pub animations: Vec<AnimationData>,
}

pub struct CharacterStateUi {
    current_animation: Option<usize>,
}

impl CharacterStateUi {
    pub fn new() -> Self {
        Self {
            current_animation: None,
        }
    }
}


impl CharacterState {
    pub fn new() -> Self {
        Self { animations: vec![] }
    }

    pub fn duration(&self) -> usize {
        self.animations
            .iter()
            .map(|item| item.delay + item.duration())
            .fold(0, std::cmp::max)
    }

    pub fn draw_at_time(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        time: usize,
        world: nalgebra::Matrix4<f32>,
    ) -> GameResult<()> {
        if time < self.duration() {
            for animation in self.animations.iter() {
                animation.draw_at_time(ctx, assets, time, world)?
            }
        }
        Ok(())
    }

    pub fn draw_ui(
        &mut self,
        ctx: &mut Context,
        assets: &mut Assets,
        ui: &Ui<'_>,
        ui_data: &mut CharacterStateUi,
    ) -> GameResult<()> {
        ui.label_text(im_str!("Duration"), &im_str!("{}", self.duration()));

        let mut buffer = ui_data
            .current_animation
            .and_then(|item| i32::try_from(item).ok())
            .unwrap_or(-1);
        ui.list_box_owned(
            im_str!("Frame List"),
            &mut buffer,
            &self
                .animations
                .iter()
                .map(|item| im_str!("{}", item.name().to_owned()))
                .collect::<Vec<_>>(),
            5,
        );
        ui_data.current_animation = usize::try_from(buffer).ok();

        if let (Some(current_animation), true) =
            (ui_data.current_animation, !self.animations.is_empty())
        {
            let (up, down) = if current_animation == 0 {
                let temp = ui.arrow_button(im_str!("Swap Down"), imgui::Direction::Down);
                (false, temp)
            } else if current_animation == self.animations.len() - 1 {
                let temp = ui.arrow_button(im_str!("Swap Up"), imgui::Direction::Up);
                (temp, false)
            } else {
                let up = ui.arrow_button(im_str!("Swap Up"), imgui::Direction::Up);
                ui.same_line(0.0);
                let down = ui.arrow_button(im_str!("Swap Down"), imgui::Direction::Down);
                (up, down)
            };
            if up && current_animation != 0 {
                self.animations
                    .swap(current_animation, current_animation - 1);
                ui_data.current_animation = Some(current_animation - 1);
            } else if down && current_animation != self.animations.len() - 1 {
                self.animations
                    .swap(current_animation, current_animation + 1);
                ui_data.current_animation = Some(current_animation + 1);
            }
        }

        if ui.small_button(im_str!("Add Animation(s)")) {
            let path_result = nfd::open_file_multiple_dialog(Some("tar"), None);
            match path_result {
                Ok(path) => match path {
                    nfd::Response::Cancel => (),
                    nfd::Response::Okay(path) => {
                        self.animations.push(AnimationData::new(
                            Animation::load_tar(ctx, assets, &path).unwrap(),
                        ));
                    }
                    nfd::Response::OkayMultiple(paths) => {
                        for path in paths {
                            self.animations.push(AnimationData::new(
                                Animation::load_tar(ctx, assets, &path).unwrap(),
                            ));
                        }
                    }
                },
                Err(err) => {
                    dbg!(err);
                }

            }
        }
        Ok(())
    }
}