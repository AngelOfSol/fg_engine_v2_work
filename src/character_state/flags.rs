use serde::{Deserialize, Serialize};

use crate::typedefs::collision::Vec2;

use crate::imgui_extra::UiExtensions;
use imgui::*;
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum MeleeHittable {
    Invuln,
    Hit,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum MagicHittable {
    Hit,
    Graze,
    Invuln,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct Flags {
    pub melee: MeleeHittable,
    pub bullet: MagicHittable,
    pub can_block: bool,
    pub airborne: bool,
    pub reset_velocity: bool,
    pub accel: Vec2,
}

#[derive(Debug, Clone, Copy)]
pub struct MovementData {
    pub accel: Vec2,
    pub vel: Vec2,
    pub pos: Vec2,
}
impl MovementData {
    pub fn new() -> Self {
        Self {
            accel: Vec2::zeros(),
            vel: Vec2::zeros(),
            pos: Vec2::zeros(),
        }
    }
}

impl Flags {
    pub fn new() -> Self {
        Self {
            melee: MeleeHittable::Hit,
            bullet: MagicHittable::Hit,
            can_block: false,
            airborne: false,
            reset_velocity: true,
            accel: Vec2::zeros(),
        }
    }

    pub fn apply_movement(&self, mut value: MovementData) -> MovementData {
        if self.reset_velocity {
            value.vel = Vec2::zeros();
        }
        value.vel += self.accel;
        value.pos += value.vel;
        value
    }
}

pub struct FlagsUi;

impl FlagsUi {
    pub fn draw_ui(ui: &Ui<'_>, data: &mut Flags) {
        ui.checkbox(im_str!("Can Block"), &mut data.can_block);
        ui.checkbox(im_str!("Airborne"), &mut data.airborne);
        ui.checkbox(im_str!("Reset Velocity"), &mut data.reset_velocity);

        ui.separator();

        ui.text(im_str!("Melee"));
        ui.push_id("Melee");
        ui.radio_button(im_str!("Hit"), &mut data.melee, MeleeHittable::Hit);
        ui.same_line(0.0);
        ui.radio_button(im_str!("Invuln"), &mut data.melee, MeleeHittable::Invuln);
        ui.pop_id();

        ui.separator();

        ui.text(im_str!("Magic"));
        ui.push_id("Magic");
        ui.radio_button(im_str!("Hit"), &mut data.bullet, MagicHittable::Hit);
        ui.same_line(0.0);
        ui.radio_button(im_str!("Graze"), &mut data.bullet, MagicHittable::Graze);
        ui.same_line(0.0);
        ui.radio_button(im_str!("Invuln"), &mut data.bullet, MagicHittable::Invuln);
        ui.pop_id();

        ui.separator();

        ui.input_vec2_int(im_str!("Acceleration"), &mut data.accel);
    }

    pub fn draw_display_ui(ui: &Ui<'_>, data: &Flags, movement: &MovementData) {
        ui.push_id("Display");

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
        ui.pop_id();
    }
}
