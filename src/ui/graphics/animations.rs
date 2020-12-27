use crate::assets::Assets;
use crate::graphics::Animation;
use crate::imgui_extra::UiExtensions;
use crate::ui::character::state::AnimationUi;
use ggez::Context;
use imgui::{im_str, Ui};
use nfd::Response;
use std::path::PathBuf;

pub struct AnimationsUi {
    current_animation: Option<usize>,
    animation_state: AnimationUi,
}

impl AnimationsUi {
    pub fn draw_ui(
        &mut self,
        ctx: &mut Context,
        assets: &mut Assets,
        ui: &Ui<'_>,
        data: &mut Vec<Animation>,
    ) -> Option<String> {
        let mut ret = None;

        let id = ui.push_id("Animations");
        ui.rearrangable_list_box(
            im_str!("List"),
            &mut self.current_animation,
            data,
            |item| im_str!("{}", item.name.to_owned()),
            5,
        );
        if ui.small_button(im_str!("Load")) {
            let paths = match nfd::open_file_multiple_dialog(Some("json"), None) {
                Ok(Response::Okay(path)) => vec![path],
                Ok(Response::OkayMultiple(paths)) => paths,
                _ => vec![],
            };
            for path in paths {
                data.push(Animation::load_from_json(ctx, assets, PathBuf::from(path)).unwrap());
            }
        }

        ui.same_line(0.0);
        if ui.small_button(im_str!("New")) {
            // TODO add popup?
            data.push(Animation::new("new"));
        }

        if let Some(animation) = self.current_animation {
            if let Some(animation) = data.get_mut(animation) {
                ui.same_line(0.0);
                if ui.small_button(im_str!("Edit")) {
                    ret = Some(animation.name.clone());
                }
            }
            ui.same_line(0.0);
            if ui.small_button(im_str!("Delete")) {
                data.remove(animation);
            }
            if let Some(animation) = data.get_mut(animation) {
                ui.separator();
                self.animation_state.draw_ui(ui, animation);
            }
        }
        id.pop(ui);

        ret
    }

    pub fn new() -> Self {
        Self {
            current_animation: None,
            animation_state: Default::default(),
        }
    }
}
