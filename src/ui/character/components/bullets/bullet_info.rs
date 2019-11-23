use crate::assets::Assets;
use crate::graphics::Animation;
use crate::hitbox::Hitbox;
use crate::imgui_extra::UiExtensions;
use crate::ui::editor::Mode;
use ggez::Context;
use imgui::{im_str, Ui};
use nfd::Response;
use std::path::PathBuf;

use crate::character::components::BulletInfo;

pub struct BulletInfoUi {
    new_property: String,
}

impl BulletInfoUi {
    pub fn new() -> Self {
        Self {
            new_property: "".to_owned(),
        }
    }
    pub fn draw_ui(
        &mut self,
        ctx: &mut Context,
        assets: &mut Assets,
        ui: &Ui<'_>,
        data: &mut BulletInfo,
        attack_ids: &[String],
    ) -> Option<Mode> {
        let mut ret = None;
        ui.text(im_str!("Animation"));
        if ui.small_button(im_str!("Load")) {
            let path_result = nfd::open_file_multiple_dialog(Some("json"), None);
            match path_result {
                Ok(path) => match path {
                    Response::Cancel => (),
                    Response::Okay(path) => {
                        data.animation =
                            Animation::load_from_json(ctx, assets, PathBuf::from(path)).unwrap();
                    }
                    Response::OkayMultiple(paths) => {
                        data.animation =
                            Animation::load_from_json(ctx, assets, PathBuf::from(&paths[0]))
                                .unwrap();
                    }
                },
                Err(err) => {
                    dbg!(err);
                }
            }
        }
        ui.same_line(0.0);
        if ui.small_button(im_str!("New")) {
            ret = Some(Mode::New);
        }
        ui.same_line(0.0);
        if ui.small_button(im_str!("Edit")) {
            ret = Some(Mode::Edit("self".to_owned()));
        }
        ui.separator();
        ui.text(im_str!("Hitbox"));
        Hitbox::draw_ui(ui, &mut data.hitbox);

        ui.combo_items(
            im_str!("Attack Data##Combo"),
            &mut data.attack_id,
            attack_ids,
            &|item| im_str!("{}", item).into(),
        );
        ui.separator();
        ui.text(im_str!("Properties"));
        let mut to_delete = None;
        for item in data.properties.iter() {
            let id = ui.push_id(&item);
            ui.text(im_str!("{}", item));
            ui.same_line(0.0);
            if ui.small_button(im_str!("Delete")) {
                to_delete = Some(item.clone());
            }
            id.pop(ui);
        }
        if let Some(item) = to_delete {
            data.properties.remove(&item);
        }
        ui.input_string(im_str!("##Property Name"), &mut self.new_property);
        ui.same_line(0.0);
        if ui.small_button(im_str!("Add##Property")) && self.new_property != "" {
            let new = std::mem::replace(&mut self.new_property, "".to_owned());
            data.properties.insert(new);
        }

        ret
    }
}
