mod animation_data;

use crate::animation::Animation;
use crate::assets::Assets;

use crate::imgui_extra::UiExtensions;

use imgui::*;

use ggez::{Context, GameResult};
use serde::{Deserialize, Serialize};

use std::cmp;

use nfd::Response;

use animation_data::AnimationData;

use crate::typedefs::graphics::Matrix4;

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
            .fold(0, cmp::max)
    }

    pub fn draw_at_time(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        time: usize,
        world: Matrix4,
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

        ui.rearrangable_list_box(
            im_str!("Frame List"),
            &mut ui_data.current_animation,
            &mut self.animations,
            |item| im_str!("{}", item.name().to_owned()),
            5,
        );

        if ui.small_button(im_str!("Add Animation(s)")) {
            let path_result = nfd::open_file_multiple_dialog(Some("tar"), None);
            match path_result {
                Ok(path) => match path {
                    Response::Cancel => (),
                    Response::Okay(path) => {
                        self.animations.push(AnimationData::new(
                            Animation::load_tar(ctx, assets, &path).unwrap(),
                        ));
                    }
                    Response::OkayMultiple(paths) => {
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