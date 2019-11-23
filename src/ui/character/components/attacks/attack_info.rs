use crate::character::components::{AttackInfo, AttackLevel, GroundAction, Guard};
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

        ui.text(im_str!("Attack Type:"));
        ui.radio_button(im_str!("Melee"), &mut data.melee, true);
        ui.same_line(0.0);
        ui.radio_button(im_str!("Magic"), &mut data.melee, false);

        ui.checkbox(im_str!("Grazeable"), &mut data.grazeable);
        ui.checkbox(im_str!("Air Unblockable"), &mut data.air_unblockable);

        ui.text(im_str!("Guard As:"));
        ui.radio_button(im_str!("Low"), &mut data.guard, Guard::Low);
        ui.same_line(0.0);
        ui.radio_button(im_str!("Mid"), &mut data.guard, Guard::Mid);
        ui.same_line(0.0);
        ui.radio_button(im_str!("High"), &mut data.guard, Guard::High);

        if ui.collapsing_header(im_str!("On Hit")).build() {
            let id = ui.push_id("On Hit");
            ui.checkbox(im_str!("Launches"), &mut data.launcher);
            ui.checkbox(im_str!("Can CH"), &mut data.can_counter_hit);
            ui.separator();
            ui.input_whole(im_str!("Attacker Stop"), &mut data.on_hit.attacker_stop)
                .unwrap();
            ui.input_whole(im_str!("Defender Stop"), &mut data.on_hit.defender_stop)
                .unwrap();
            ui.text(im_str!("Forces"));
            ui.input_vec2_whole(im_str!("Air"), &mut data.on_hit.air_force);
            if !data.launcher {
                ui.input_whole(im_str!("Ground"), &mut data.on_hit.ground_pushback)
                    .unwrap();
            }

            ui.separator();
            ui.input_whole(im_str!("Starter Limit"), &mut data.starter_limit)
                .unwrap();
            if data.can_counter_hit {
                ui.input_whole(im_str!("CH Starter Limit"), &mut data.counter_hit_limit)
                    .unwrap();
            }
            ui.input_whole(im_str!("Limit Cost"), &mut data.limit_cost)
                .unwrap();
            ui.input_whole(im_str!("Hit Damage"), &mut data.hit_damage)
                .unwrap();
            ui.input_whole(im_str!("Proration (%)"), &mut data.proration)
                .unwrap();
            ui.separator();

            ui.text(im_str!("Guard As:"));
            ui.radio_button(
                im_str!("Knockdown"),
                &mut data.ground_action,
                GroundAction::Knockdown,
            );
            ui.radio_button(
                im_str!("Ground Slam"),
                &mut data.ground_action,
                GroundAction::GroundSlam,
            );
            ui.radio_button(
                im_str!("OTG"),
                &mut data.ground_action,
                GroundAction::OnTheGround,
            );

            id.pop(ui);
        }

        if ui.collapsing_header(im_str!("On Block")).build() {
            let id = ui.push_id("On Block");
            ui.input_whole(im_str!("Attacker Stop"), &mut data.on_block.attacker_stop)
                .unwrap();
            ui.input_whole(im_str!("Defender Stop"), &mut data.on_block.defender_stop)
                .unwrap();
            ui.text(im_str!("Forces"));
            if !data.air_unblockable {
                ui.input_vec2_whole(im_str!("Air"), &mut data.on_block.air_force);
            }
            ui.input_whole(im_str!("Ground"), &mut data.on_block.ground_pushback)
                .unwrap();

            ui.separator();

            ui.input_whole(im_str!("Spirit Cost"), &mut data.spirit_cost)
                .unwrap();
            ui.input_whole(im_str!("Spirit Delay"), &mut data.spirit_delay)
                .unwrap();
            ui.checkbox(im_str!("Reset Spirit Delay"), &mut data.reset_spirit_delay);
            ui.separator();
            ui.input_whole(im_str!("Chip Damage"), &mut data.chip_damage)
                .unwrap();

            id.pop(ui);
        }
    }
}
