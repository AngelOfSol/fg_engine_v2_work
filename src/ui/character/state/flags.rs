use crate::character::state::components::{Flags, MagicHittable, MeleeHittable, MovementData};
use crate::imgui_extra::UiExtensions;
use imgui::*;

pub struct FlagsUi;

impl FlagsUi {
    pub fn draw_ui(ui: &Ui<'_>, data: &mut Flags) {
        ui.checkbox(im_str!("Can Block"), &mut data.can_block);
        ui.checkbox(im_str!("Airborne"), &mut data.airborne);
        ui.checkbox(im_str!("Reset Velocity"), &mut data.reset_velocity);
        ui.checkbox(im_str!("Jump Start"), &mut data.jump_start);
        ui.checkbox(im_str!("Allow Reface"), &mut data.allow_reface);
        ui.separator();

        ui.input_whole(im_str!("Spirit Cost"), &mut data.spirit_cost)
            .unwrap();
        ui.input_whole(im_str!("Spirit Delay"), &mut data.spirit_delay)
            .unwrap();
        ui.checkbox(im_str!("Reset Spirit Delay"), &mut data.reset_spirit_delay);

        ui.separator();
        {
            ui.text(im_str!("Melee"));
            let id = ui.push_id("Melee");
            ui.radio_button(im_str!("Hit"), &mut data.melee, MeleeHittable::Hit);
            ui.same_line(0.0);
            ui.radio_button(im_str!("Invuln"), &mut data.melee, MeleeHittable::Invuln);

            id.pop(ui);
        }
        ui.separator();
        {
            ui.text(im_str!("Magic"));
            let id = ui.push_id("Magic");
            ui.radio_button(im_str!("Hit"), &mut data.bullet, MagicHittable::Hit);
            ui.same_line(0.0);
            ui.radio_button(im_str!("Graze"), &mut data.bullet, MagicHittable::Graze);
            ui.same_line(0.0);
            ui.radio_button(im_str!("Invuln"), &mut data.bullet, MagicHittable::Invuln);
            id.pop(ui);
        }

        ui.separator();

        ui.input_vec2_whole(im_str!("Acceleration"), &mut data.accel);
        if !data.airborne {
            ui.input_whole(im_str!("Friction"), &mut data.friction)
                .unwrap();
        }
    }

    pub fn draw_display_ui(ui: &Ui<'_>, data: &Flags, movement: &MovementData) {
        let id = ui.push_id("Display");

        ui.text(&im_str!("Can Block: {}", data.can_block));
        ui.text(&im_str!("Airborne: {}", data.airborne));
        ui.text(&im_str!("Reset Velocity: {}", data.reset_velocity));
        ui.separator();

        ui.text(&im_str!("Melee Invuln: {:?}", data.melee));
        ui.text(&im_str!("Magic Invuln: {:?}", data.bullet));
        ui.separator();

        ui.text(&im_str!(
            "Acceleration: [{}, {}]",
            data.accel.x,
            data.accel.y
        ));
        ui.text(&im_str!(
            "Velocity: [{}, {}]",
            movement.vel.x,
            movement.vel.y
        ));
        ui.text(&im_str!(
            "Position: [{}, {}]",
            movement.pos.x,
            movement.pos.y
        ));
        if !data.airborne {
            ui.text(&im_str!("Friction: {}", data.friction));
        }
        id.pop(ui);
    }
}
