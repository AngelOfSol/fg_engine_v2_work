use crate::character::components::{AttackInfo, GroundAction, Guard};
use crate::imgui_extra::UiExtensions;
use imgui::{im_str, Ui};

pub struct AttackInfoUi {}

impl AttackInfoUi {
    pub fn new() -> Self {
        Self {}
    }
    pub fn draw_ui(&mut self, ui: &Ui<'_>, data: &mut AttackInfo) {
        ui.checkbox(im_str!("Melee"), &mut data.melee);
        ui.checkbox(im_str!("Magic"), &mut data.magic);
        ui.checkbox(im_str!("Grazeable"), &mut data.grazeable);
        ui.checkbox(im_str!("Air Unblockable"), &mut data.air_unblockable);
        ui.checkbox(im_str!("Can CH"), &mut data.can_counter_hit);

        ui.text(im_str!("Guard As:"));
        ui.radio_button(im_str!("Low"), &mut data.guard, Guard::Low);
        ui.same_line(0.0);
        ui.radio_button(im_str!("Mid"), &mut data.guard, Guard::Mid);
        ui.same_line(0.0);
        ui.radio_button(im_str!("High"), &mut data.guard, Guard::High);
        let can_wrongblock = data.guard != Guard::Mid;

        if ui.collapsing_header(im_str!("On Hit")).build() {
            let info = &mut data.on_hit;

            let id = ui.push_id("On Hit");
            ui.checkbox(im_str!("Launches"), &mut info.launcher);
            ui.separator();
            ui.input_whole(im_str!("Hitstun"), &mut info.stun).unwrap();
            ui.input_whole(im_str!("Air Hitstun"), &mut info.air_stun)
                .unwrap();
            ui.input_whole(im_str!("Attacker Stop"), &mut info.attacker_stop)
                .unwrap();
            ui.input_whole(im_str!("Defender Stop"), &mut info.defender_stop)
                .unwrap();

            ui.separator();
            ui.text(im_str!("Forces"));
            ui.input_vec2_whole(im_str!("Air"), &mut info.air_force);
            if !info.launcher {
                ui.input_whole(im_str!("Ground"), &mut info.ground_pushback)
                    .unwrap();
            }

            ui.separator();
            ui.text(im_str!("Meter"));
            ui.input_whole(im_str!("Attacker Gain"), &mut info.attacker_meter)
                .unwrap();
            ui.input_whole(im_str!("Defender Gain"), &mut info.defender_meter)
                .unwrap();

            ui.separator();
            ui.input_whole(im_str!("Starter Limit"), &mut info.starter_limit)
                .unwrap();

            ui.input_whole(im_str!("Limit Cost"), &mut info.limit_cost)
                .unwrap();
            ui.input_whole(im_str!("Hit Damage"), &mut info.damage)
                .unwrap();
            ui.checkbox(im_str!("Lethal"), &mut info.lethal);
            ui.input_whole(im_str!("Proration (%)"), &mut info.proration)
                .unwrap();
            ui.separator();

            ui.input_whole(im_str!("Spirit Cost"), &mut info.spirit_cost)
                .unwrap();
            ui.input_whole(im_str!("Spirit Delay"), &mut info.spirit_delay)
                .unwrap();
            ui.checkbox(im_str!("Reset Spirit Delay"), &mut info.reset_spirit_delay);
            ui.separator();

            ui.text(im_str!("Knockdown As:"));
            ui.radio_button(
                im_str!("Knockdown"),
                &mut info.ground_action,
                GroundAction::Knockdown,
            );
            ui.radio_button(
                im_str!("Ground Slam"),
                &mut info.ground_action,
                GroundAction::GroundSlam,
            );
            ui.radio_button(
                im_str!("OTG"),
                &mut info.ground_action,
                GroundAction::OnTheGround,
            );

            id.pop(ui);
        }
        if data.can_counter_hit && ui.collapsing_header(im_str!("On Counter Hit")).build() {
            let info = &mut data.on_counter_hit;

            let id = ui.push_id("On Counter Hit");
            ui.checkbox(im_str!("Launches"), &mut info.launcher);
            ui.separator();
            ui.input_whole(im_str!("Hitstun"), &mut info.stun).unwrap();
            ui.input_whole(im_str!("Air Hitstun"), &mut info.air_stun)
                .unwrap();
            ui.input_whole(im_str!("Attacker Stop"), &mut info.attacker_stop)
                .unwrap();
            ui.input_whole(im_str!("Defender Stop"), &mut info.defender_stop)
                .unwrap();

            ui.separator();
            ui.text(im_str!("Forces"));
            ui.input_vec2_whole(im_str!("Air"), &mut info.air_force);
            if !info.launcher {
                ui.input_whole(im_str!("Ground"), &mut info.ground_pushback)
                    .unwrap();
            }

            ui.separator();
            ui.text(im_str!("Meter"));
            ui.input_whole(im_str!("Attacker Gain"), &mut info.attacker_meter)
                .unwrap();
            ui.input_whole(im_str!("Defender Gain"), &mut info.defender_meter)
                .unwrap();

            ui.separator();
            ui.input_whole(im_str!("Starter Limit"), &mut info.starter_limit)
                .unwrap();

            ui.input_whole(im_str!("Hit Damage"), &mut info.damage)
                .unwrap();
            ui.checkbox(im_str!("Lethal"), &mut info.lethal);
            ui.input_whole(im_str!("Proration (%)"), &mut info.proration)
                .unwrap();
            ui.separator();

            ui.input_whole(im_str!("Spirit Cost"), &mut info.spirit_cost)
                .unwrap();
            ui.input_whole(im_str!("Spirit Delay"), &mut info.spirit_delay)
                .unwrap();
            ui.checkbox(im_str!("Reset Spirit Delay"), &mut info.reset_spirit_delay);
            ui.separator();

            ui.text(im_str!("Knockdown As:"));
            ui.radio_button(
                im_str!("Knockdown"),
                &mut info.ground_action,
                GroundAction::Knockdown,
            );
            ui.radio_button(
                im_str!("Ground Slam"),
                &mut info.ground_action,
                GroundAction::GroundSlam,
            );
            ui.radio_button(
                im_str!("OTG"),
                &mut info.ground_action,
                GroundAction::OnTheGround,
            );
            id.pop(ui);
        }

        let can_crush =
            data.on_block.spirit_cost > 0 || (can_wrongblock && data.on_wrongblock.spirit_cost > 0);

        if can_crush && ui.collapsing_header(im_str!("On Guard Crush")).build() {
            let info = &mut data.on_guard_crush;

            let id = ui.push_id("On Guard Hit");
            ui.checkbox(im_str!("Launches"), &mut info.launcher);
            ui.separator();
            ui.input_whole(im_str!("Hitstun"), &mut info.stun).unwrap();
            ui.input_whole(im_str!("Air Hitstun"), &mut info.air_stun)
                .unwrap();
            ui.input_whole(im_str!("Attacker Stop"), &mut info.attacker_stop)
                .unwrap();
            ui.input_whole(im_str!("Defender Stop"), &mut info.defender_stop)
                .unwrap();

            ui.separator();
            ui.text(im_str!("Forces"));
            ui.input_vec2_whole(im_str!("Air"), &mut info.air_force);
            if !info.launcher {
                ui.input_whole(im_str!("Ground"), &mut info.ground_pushback)
                    .unwrap();
            }

            ui.separator();
            ui.text(im_str!("Meter"));
            ui.input_whole(im_str!("Attacker Gain"), &mut info.attacker_meter)
                .unwrap();
            ui.input_whole(im_str!("Defender Gain"), &mut info.defender_meter)
                .unwrap();

            ui.separator();
            ui.input_whole(im_str!("Starter Limit"), &mut info.starter_limit)
                .unwrap();

            ui.input_whole(im_str!("Hit Damage"), &mut info.damage)
                .unwrap();
            ui.checkbox(im_str!("Lethal"), &mut info.lethal);
            ui.input_whole(im_str!("Proration (%)"), &mut info.proration)
                .unwrap();
            ui.separator();

            ui.text(im_str!("Knockdown As:"));
            ui.radio_button(
                im_str!("Knockdown"),
                &mut info.ground_action,
                GroundAction::Knockdown,
            );
            ui.radio_button(
                im_str!("Ground Slam"),
                &mut info.ground_action,
                GroundAction::GroundSlam,
            );
            ui.radio_button(
                im_str!("OTG"),
                &mut info.ground_action,
                GroundAction::OnTheGround,
            );
            id.pop(ui);
        }

        if ui.collapsing_header(im_str!("On Block")).build() {
            let id = ui.push_id("On Block");
            let info = &mut data.on_block;

            ui.input_whole(im_str!("Attacker Stop"), &mut info.attacker_stop)
                .unwrap();
            ui.input_whole(im_str!("Defender Stop"), &mut info.defender_stop)
                .unwrap();
            ui.input_whole(im_str!("Blockstun"), &mut info.stun)
                .unwrap();

            if !data.air_unblockable {
                ui.input_whole(im_str!("Air Hitstun"), &mut info.air_stun)
                    .unwrap();
            }

            ui.separator();
            ui.text(im_str!("Forces"));
            if !data.air_unblockable {
                ui.input_vec2_whole(im_str!("Air"), &mut info.air_force);
            }
            ui.input_whole(im_str!("Ground"), &mut info.ground_pushback)
                .unwrap();

            ui.separator();
            ui.text(im_str!("Meter"));
            ui.input_whole(im_str!("Attacker Gain"), &mut info.attacker_meter)
                .unwrap();
            ui.input_whole(im_str!("Defender Gain"), &mut info.defender_meter)
                .unwrap();

            ui.separator();

            ui.input_whole(im_str!("Spirit Cost"), &mut info.spirit_cost)
                .unwrap();
            ui.input_whole(im_str!("Spirit Delay"), &mut info.spirit_delay)
                .unwrap();
            ui.checkbox(im_str!("Reset Spirit Delay"), &mut info.reset_spirit_delay);
            ui.separator();
            ui.input_whole(im_str!("Chip Damage"), &mut info.damage)
                .unwrap();

            id.pop(ui);
        }
        if can_wrongblock && ui.collapsing_header(im_str!("On Wrongblock")).build() {
            let id = ui.push_id("On Wrongblock");
            let info = &mut data.on_wrongblock;

            ui.input_whole(im_str!("Attacker Stop"), &mut info.attacker_stop)
                .unwrap();
            ui.input_whole(im_str!("Defender Stop"), &mut info.defender_stop)
                .unwrap();
            ui.input_whole(im_str!("Blockstun"), &mut info.stun)
                .unwrap();

            ui.separator();
            ui.text(im_str!("Forces"));
            ui.input_whole(im_str!("Ground"), &mut info.ground_pushback)
                .unwrap();

            ui.separator();
            ui.text(im_str!("Meter"));
            ui.input_whole(im_str!("Attacker Gain"), &mut info.attacker_meter)
                .unwrap();
            ui.input_whole(im_str!("Defender Gain"), &mut info.defender_meter)
                .unwrap();

            ui.separator();

            ui.input_whole(im_str!("Spirit Cost"), &mut info.spirit_cost)
                .unwrap();
            ui.input_whole(im_str!("Spirit Delay"), &mut info.spirit_delay)
                .unwrap();
            ui.checkbox(im_str!("Reset Spirit Delay"), &mut info.reset_spirit_delay);
            ui.separator();
            ui.input_whole(im_str!("Chip Damage"), &mut info.damage)
                .unwrap();

            id.pop(ui);
        }

        if data.grazeable && ui.collapsing_header(im_str!("On Graze")).build() {
            let id = ui.push_id("On Graze");
            let info = &mut data.on_graze;

            ui.input_whole(im_str!("Defender Stop"), &mut info.defender_stop)
                .unwrap();

            ui.separator();
            ui.text(im_str!("Meter"));
            ui.input_whole(im_str!("Attacker Gain"), &mut info.attacker_meter)
                .unwrap();
            ui.input_whole(im_str!("Defender Gain"), &mut info.defender_meter)
                .unwrap();

            ui.separator();

            ui.input_whole(im_str!("Spirit Cost"), &mut info.spirit_cost)
                .unwrap();
            ui.input_whole(im_str!("Spirit Delay"), &mut info.spirit_delay)
                .unwrap();
            ui.checkbox(im_str!("Reset Spirit Delay"), &mut info.reset_spirit_delay);
            ui.separator();
            ui.input_whole(im_str!("Chip Damage"), &mut info.damage)
                .unwrap();

            id.pop(ui);
        }
    }
}
