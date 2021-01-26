use crate::character::state::components::{CancelSet, CommandType};
use crate::imgui_extra::UiExtensions;
use imgui::*;

#[derive(Default)]
pub struct CancelSetUi {}

const GREEN: [f32; 4] = [0.2, 1.0, 0.2, 1.0];
const BLUE: [f32; 4] = [0.7, 0.7, 1.0, 1.0];
const RED: [f32; 4] = [1.0, 0.2, 0.2, 1.0];

impl CancelSetUi {
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
}
