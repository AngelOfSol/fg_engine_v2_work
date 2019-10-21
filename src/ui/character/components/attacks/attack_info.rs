use crate::character::components::{AttackInfo, AttackLevel, Guard};
use crate::imgui_extra::UiExtensions;
use imgui::{im_str, Ui};

pub struct AttackInfoUi {}

impl AttackInfoUi {
    pub fn new() -> Self {
        Self {}
    }
    pub fn draw_ui(&mut self, ui: &Ui<'_>, data: &mut AttackInfo) {
        ui.text(im_str!("Attack Level:"));
        ui.radio_button(im_str!("A"), &mut data.level, AttackLevel::A);
        ui.same_line(0.0);
        ui.radio_button(im_str!("B"), &mut data.level, AttackLevel::B);
        ui.same_line(0.0);
        ui.radio_button(im_str!("C"), &mut data.level, AttackLevel::C);
        ui.same_line(0.0);
        ui.radio_button(im_str!("D"), &mut data.level, AttackLevel::D);

        ui.checkbox(im_str!("Grazeable"), &mut data.grazeable);
        ui.checkbox(im_str!("Melee"), &mut data.melee);
        ui.checkbox(im_str!("Air Unblockable"), &mut data.air_unblockable);

        ui.text(im_str!("Guard As:"));
        ui.radio_button(im_str!("Low"), &mut data.guard, Guard::Low);
        ui.same_line(0.0);
        ui.radio_button(im_str!("Mid"), &mut data.guard, Guard::Mid);
        ui.same_line(0.0);
        ui.radio_button(im_str!("High"), &mut data.guard, Guard::High);

        if ui.collapsing_header(im_str!("On Hit")).build() {
            ui.input_whole(im_str!("Attacker Stop"), &mut data.on_hit.attacker_stop)
                .unwrap();
            ui.input_whole(im_str!("Defender Stop"), &mut data.on_hit.defender_stop)
                .unwrap();
            ui.text(im_str!("Forces"));
            ui.input_vec2_whole(im_str!("Air"), &mut data.on_hit.air_force);
            ui.input_whole(im_str!("Ground"), &mut data.on_hit.ground_pushback)
                .unwrap();
        }

        if ui.collapsing_header(im_str!("On Block")).build() {
            ui.input_whole(im_str!("Attacker Stop"), &mut data.on_block.attacker_stop)
                .unwrap();
            ui.input_whole(im_str!("Defender Stop"), &mut data.on_block.defender_stop)
                .unwrap();
            ui.text(im_str!("Forces"));
            ui.input_vec2_whole(im_str!("Air"), &mut data.on_block.air_force);
            ui.input_whole(im_str!("Ground"), &mut data.on_block.ground_pushback)
                .unwrap();
        }
    }
}
