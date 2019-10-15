use crate::imgui_extra::UiExtensions;
use imgui::*;

use crate::character_state::{CancelSet, MoveType};

pub struct CancelSetUi {
    state_list: Vec<String>,
    new_disallow: String,
}

const GREEN: [f32; 4] = [0.2, 1.0, 0.2, 1.0];
const BLUE: [f32; 4] = [0.7, 0.7, 1.0, 1.0];
const RED: [f32; 4] = [1.0, 0.2, 0.2, 1.0];

impl CancelSetUi {
    pub fn new(state_list: Vec<String>) -> CancelSetUi {
        CancelSetUi {
            new_disallow: state_list.get(0).cloned().unwrap_or_else(|| "".to_owned()),
            state_list,
        }
    }
    pub fn draw_ui(&mut self, ui: &Ui<'_>, data: &mut CancelSet<String>) {
        for move_type in MoveType::all() {
            ui.text(&im_str!("{}:", move_type));
            let id = ui.push_id(&format!("{}", move_type));
            let token = ui.push_style_color(StyleColor::Text, GREEN);
            ui.checkbox_hash(im_str!("Always"), move_type, &mut data.always);
            token.pop(ui);
            let token = ui.push_style_color(StyleColor::Text, BLUE);
            ui.same_line(0.0);
            ui.checkbox_hash(im_str!("Block"), move_type, &mut data.block);
            token.pop(ui);
            let token = ui.push_style_color(StyleColor::Text, RED);
            ui.same_line(0.0);
            ui.checkbox_hash(im_str!("Hit"), move_type, &mut data.hit);
            token.pop(ui);
            id.pop(ui);
        }
        ui.separator();

        ui.text(im_str!("Disallowed"));
        let mut to_delete = None;
        for item in data.disallow.iter() {
            {
                let token = ui.push_style_color(StyleColor::Text, RED);
                ui.text(im_str!("{}", item));
                token.pop(ui);
            }
            ui.same_line(0.0);
            if ui.small_button(im_str!("Delete")) {
                to_delete = Some(item.clone());
            }
        }
        if let Some(item) = to_delete {
            data.disallow.remove(&item);
        }
        ui.combo_items(
            im_str!("##Combo"),
            &mut self.new_disallow,
            &self.state_list,
            &|item| im_str!("{}", item).into(),
        );

        if ui.small_button(im_str!("Add")) && self.new_disallow != "" {
            data.disallow.insert(self.new_disallow.clone());
        }
    }
    pub fn draw_display_ui(ui: &Ui<'_>, data: &CancelSet<String>) {
        ui.text(im_str!("Always"));
        let token = ui.push_style_color(StyleColor::Text, GREEN);
        for move_type in data.always.iter() {
            ui.text(im_str!("{}", move_type));
        }
        token.pop(ui);

        ui.separator();
        ui.text(im_str!("On Block"));
        let token = ui.push_style_color(StyleColor::Text, BLUE);
        for move_type in data.block.iter() {
            ui.text(im_str!("{}", move_type));
        }
        token.pop(ui);

        ui.separator();
        ui.text(im_str!("On Hit"));
        let token = ui.push_style_color(StyleColor::Text, RED);
        for move_type in data.hit.iter() {
            ui.text(im_str!("{}", move_type));
        }
        token.pop(ui);

        if !data.disallow.is_empty() {
            ui.separator();
            ui.text(im_str!("Disallowed"));
            let token = ui.push_style_color(StyleColor::Text, RED);
            for item in data.disallow.iter() {
                ui.text(im_str!("{}", item));
            }
            token.pop(ui);
        }
    }
}
