mod animation_data;
mod cancel_set;
mod flags;
mod hitbox_set;

use crate::animation::Animation;
use crate::assets::Assets;

use crate::imgui_extra::UiExtensions;

use imgui::*;

use ggez::{Context, GameResult};
use serde::{Deserialize, Serialize};

use std::cmp;

use nfd::Response;

use animation_data::{AnimationData, AnimationDataUi};
pub use cancel_set::{CancelSet, CancelSetUi};
pub use flags::{Flags, FlagsUi, MovementData};
pub use hitbox_set::{HitboxSet, HitboxSetUi};

use crate::timeline::{AtTime, Timeline};

use crate::typedefs::graphics::Matrix4;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct CharacterState {
    pub animations: Vec<AnimationData>,
    pub flags: Timeline<Flags>,
    pub cancels: Timeline<CancelSet>,
    pub hitboxes: Timeline<HitboxSet>,
    pub name: String,
}

impl CharacterState {
    pub fn new() -> Self {
        Self {
            animations: vec![],
            flags: vec![(Flags::new(), 1)],
            cancels: vec![(CancelSet::new(), 1)],
            hitboxes: vec![(HitboxSet::new(), 1)],
            name: "new_state".to_owned(),
        }
    }

    pub fn duration(&self) -> usize {
        self.animations
            .iter()
            .map(|item| item.delay + item.duration())
            .fold(0, cmp::max)
    }

    pub fn fix_duration(&mut self) {
        if self.duration() > 0 {
            self.flags.fix_duration(self.duration());
            self.cancels.fix_duration(self.duration());
            self.hitboxes.fix_duration(self.duration());
        }
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
    pub fn draw_at_time_debug(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        time: usize,
        world: Matrix4,
    ) -> GameResult<()> {
        if time < self.duration() {
            for animation in self.animations.iter() {
                animation.draw_at_time_debug(ctx, assets, time, world)?
            }
        }
        Ok(())
    }
}

pub struct CharacterStateUi {
    current_animation: Option<usize>,
    current_flags: Option<usize>,
    current_cancels: Option<usize>,
    current_hitboxes: Option<usize>,
}

impl CharacterStateUi {
    pub fn new() -> Self {
        Self {
            current_animation: None,
            current_flags: None,
            current_cancels: None,
            current_hitboxes: None,
        }
    }

    pub fn draw_ui(
        &mut self,
        ctx: &mut Context,
        assets: &mut Assets,
        ui: &Ui<'_>,
        data: &mut CharacterState,
    ) -> GameResult<()> {
        ui.input_string(im_str!("Name"), &mut data.name);
        ui.label_text(im_str!("Duration"), &im_str!("{}", data.duration()));

        if ui.collapsing_header(im_str!("Animations")).build() {
            ui.push_id("Animations");
            ui.rearrangable_list_box(
                im_str!("List"),
                &mut self.current_animation,
                &mut data.animations,
                |item| im_str!("{}", item.name().to_owned()),
                5,
            );
            if ui.small_button(im_str!("Add")) {
                let path_result = nfd::open_file_multiple_dialog(Some("tar"), None);
                match path_result {
                    Ok(path) => match path {
                        Response::Cancel => (),
                        Response::Okay(path) => {
                            data.animations.push(AnimationData::new(
                                Animation::load_tar(ctx, assets, &path).unwrap(),
                            ));
                        }
                        Response::OkayMultiple(paths) => {
                            for path in paths {
                                data.animations.push(AnimationData::new(
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
            if let Some(animation) = self.current_animation {
                let animation = &mut data.animations[animation];
                AnimationDataUi::new().draw_ui(ui, animation)?;
            }
            ui.separator();
            ui.pop_id();
        }
        data.fix_duration();

        if ui.collapsing_header(im_str!("Flags")).build() {
            ui.push_id("Flags");
            let mut counter = 0;
            ui.rearrangable_list_box(
                im_str!("List\n[Start, End]"),
                &mut self.current_flags,
                &mut data.flags,
                |(_, duration)| {
                    let start = counter;
                    let end = counter + duration - 1;
                    counter += duration;
                    im_str!("[{}, {}]", start, end)
                },
                5,
            );

            if let Some(ref mut idx) = self.current_flags {
                ui.timeline_modify(idx, &mut data.flags);

                let (ref mut flags, ref mut duration) = &mut data.flags[*idx];

                let _ = ui.input_whole(im_str!("Duration"), duration);
                *duration = cmp::max(*duration, 1);

                ui.separator();
                FlagsUi::new().draw_ui(ui, flags);
            }

            ui.separator();
            ui.pop_id();
        }
        if ui.collapsing_header(im_str!("Cancels")).build() {
            ui.push_id("Cancels");
            let mut counter = 0;
            ui.rearrangable_list_box(
                im_str!("List\n[Start, End]"),
                &mut self.current_cancels,
                &mut data.cancels,
                |(_, duration)| {
                    let start = counter;
                    let end = counter + duration - 1;
                    counter += duration;
                    im_str!("[{}, {}]", start, end)
                },
                5,
            );

            if let Some(ref mut idx) = self.current_cancels {
                ui.timeline_modify(idx, &mut data.cancels);

                let (ref mut cancels, ref mut duration) = &mut data.cancels[*idx];

                let _ = ui.input_whole(im_str!("Duration"), duration);
                *duration = cmp::max(*duration, 1);

                ui.separator();
                CancelSetUi::new().draw_ui(ui, cancels);
            }
            ui.separator();
            ui.pop_id();
        }
        if ui.collapsing_header(im_str!("Hitboxes")).build() {
            ui.push_id("Hitboxes");
            let mut counter = 0;
            ui.rearrangable_list_box(
                im_str!("List\n[Start, End]"),
                &mut self.current_hitboxes,
                &mut data.hitboxes,
                |(_, duration)| {
                    let start = counter;
                    let end = counter + duration - 1;
                    counter += duration;
                    im_str!("[{}, {}]", start, end)
                },
                5,
            );

            if let Some(ref mut idx) = self.current_hitboxes {
                ui.timeline_modify(idx, &mut data.hitboxes);

                let (ref mut hitboxes, ref mut duration) = &mut data.hitboxes[*idx];

                let _ = ui.input_whole(im_str!("Duration"), duration);
                *duration = cmp::max(*duration, 1);

                ui.separator();
                HitboxSetUi::new().draw_ui(ui, hitboxes);
            }

            ui.separator();
            ui.pop_id();
        }
        Ok(())
    }
}
