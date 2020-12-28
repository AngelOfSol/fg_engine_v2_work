use crate::character::state::components::{CancelSet, CommandType};
use crate::imgui_extra::UiExtensions;
use imgui::*;

pub struct CancelSetUi {}

const GREEN: [f32; 4] = [0.2, 1.0, 0.2, 1.0];
const BLUE: [f32; 4] = [0.7, 0.7, 1.0, 1.0];
const RED: [f32; 4] = [1.0, 0.2, 0.2, 1.0];

impl CancelSetUi {
    pub fn new() -> CancelSetUi {
        CancelSetUi {}
    }
    pub fn draw_ui(&mut self, ui: &Ui<'_>, data: &mut CancelSet) {
        for move_type in CommandType::all() {
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

        ui.checkbox(im_str!("Self Gatling"), &mut data.self_gatling);
    }
    pub fn draw_display_ui(ui: &Ui<'_>, data: &CancelSet) {
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
    }
}
