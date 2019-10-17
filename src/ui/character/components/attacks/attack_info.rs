use crate::character::components::{AttackInfo, AttackLevel};
use crate::imgui_extra::UiExtensions;
use imgui::{im_str, Ui};

pub struct AttackInfoUi {}

impl AttackInfoUi {
    pub fn new() -> Self {
        Self {}
    }
    pub fn draw_ui(&mut self, ui: &Ui<'_>, data: &mut AttackInfo) {
        ui.text(im_str!("Attack Level"));
        ui.radio_button(im_str!("A"), &mut data.level, AttackLevel::A);
        ui.radio_button(im_str!("B"), &mut data.level, AttackLevel::B);
        ui.radio_button(im_str!("C"), &mut data.level, AttackLevel::C);
        ui.radio_button(im_str!("D"), &mut data.level, AttackLevel::D);

        ui.separator();
        ui.text(im_str!("Attacker"));
        ui.input_whole(im_str!("Hit Stop"), &mut data.attacker_hitstop)
            .unwrap();
        ui.input_whole(im_str!("Block Stop"), &mut data.attacker_blockstop)
            .unwrap();

        ui.separator();
        ui.text(im_str!("Defender"));
        ui.input_whole(im_str!("Hit Stop"), &mut data.defender_hitstop)
            .unwrap();
        ui.input_whole(im_str!("Block Stop"), &mut data.defender_blockstop)
            .unwrap();

        ui.separator();
        ui.text(im_str!("Forces"));
        ui.input_vec2_whole(im_str!("Air"), &mut data.air_force);
        ui.input_whole(im_str!("Ground"), &mut data.ground_pushback)
            .unwrap();
    }
}
