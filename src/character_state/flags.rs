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

pub struct FlagsUi {}

impl FlagsUi {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw_ui(&mut self, ui: &Ui<'_>, data: &mut Flags) {
        ui.checkbox(im_str!("Can Block"), &mut data.can_block);
        ui.checkbox(im_str!("Airborne"), &mut data.airborne);
        ui.checkbox(im_str!("Reset Velocity"), &mut data.reset_velocity);
        if ui.collapsing_header(im_str!("Melee")).build() {
            ui.push_id("Melee");
            ui.radio_button(im_str!("Hit"), &mut data.melee, MeleeHittable::Hit);
            ui.radio_button(im_str!("Invuln"), &mut data.melee, MeleeHittable::Invuln);
            ui.pop_id();
        }
        if ui.collapsing_header(im_str!("Magic")).build() {
            ui.push_id("Magic");
            ui.radio_button(im_str!("Hit"), &mut data.bullet, MagicHittable::Hit);
            ui.radio_button(im_str!("Graze"), &mut data.bullet, MagicHittable::Graze);
            ui.radio_button(im_str!("Invuln"), &mut data.bullet, MagicHittable::Invuln);
            ui.pop_id();
        }
        if ui.collapsing_header(im_str!("Acceleration")).build() {
            ui.push_id("Acceleration");
            let _ = ui.input_whole(im_str!("X"), &mut data.accel.x);
            let _ = ui.input_whole(im_str!("Y"), &mut data.accel.y);
            ui.pop_id();
        }
    }

    pub fn draw_display_ui(ui: &Ui<'_>, data: &Flags, movement: &MovementData) {
        ui.push_id("Display");
        if ui
            .collapsing_header(im_str!("Boolean Flags"))
            .default_open(true)
            .build()
        {
            ui.text(&im_str!("Can Block: {}", data.can_block));
            ui.text(&im_str!("Airborne: {}", data.airborne));
            ui.text(&im_str!("Reset Velocity: {}", data.reset_velocity));
        }

        if ui
            .collapsing_header(im_str!("Invuln Data"))
            .default_open(true)
            .build()
        {
            ui.text(&im_str!("Melee Invuln: {:?}", data.melee));
            ui.text(&im_str!("Magic Invuln: {:?}", data.bullet));
        }

        if ui
            .collapsing_header(im_str!("Movement"))
            .default_open(true)
            .build()
        {
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
        }
        ui.pop_id();
    }
}