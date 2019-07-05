use serde::{Deserialize, Serialize};

use ggez::GameResult;

use crate::typedefs::collision::Vec2;


use crate::imgui_extra::UiExtensions;
use imgui::*;
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum MeleeHittable {
    Invuln,
    Hit,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum BulletHittable {
    Hit,
    Graze,
    Invuln,
}


#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Flags {
    melee: MeleeHittable,
    bullet: BulletHittable,
    can_block: bool,
    airbourne: bool,
    reset_velocity: bool,
    accel: Vec2,
}

impl Flags {
    pub fn new() -> Self {
        Self {
            melee: MeleeHittable::Hit,
            bullet: BulletHittable::Hit,
            can_block: false,
            airbourne: false,
            reset_velocity: true,
            accel: Vec2::zeros(),

        }
    }
}

pub struct FlagsUi {}


impl FlagsUi {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw_ui(&mut self, ui: &Ui<'_>, data: &mut Flags) -> GameResult<()> {

        ui.checkbox(im_str!("Can Block"), &mut data.can_block);
        ui.checkbox(im_str!("Airbourne"), &mut data.airbourne);
        ui.checkbox(im_str!("Reset Velocity"), &mut data.reset_velocity);
        if ui.collapsing_header(im_str!("Melee")).build() {
            ui.push_id("Melee");
            ui.radio_button(im_str!("Hit"), &mut data.melee, MeleeHittable::Hit);
            ui.radio_button(im_str!("Invuln"), &mut data.melee, MeleeHittable::Invuln);
            ui.pop_id();
        }
        if ui.collapsing_header(im_str!("Bullet")).build() {
            ui.push_id("Bullet");
            ui.radio_button(im_str!("Hit"), &mut data.bullet, BulletHittable::Hit);
            ui.radio_button(im_str!("Graze"), &mut data.bullet, BulletHittable::Graze);
            ui.radio_button(im_str!("Invuln"), &mut data.bullet, BulletHittable::Invuln);
            ui.pop_id();
        }
        if ui.collapsing_header(im_str!("Acceleration")).build() {
            ui.push_id("Acceleration");
            let _ = ui.input_whole(im_str!("X"), &mut data.accel.x);
            let _ = ui.input_whole(im_str!("Y"), &mut data.accel.y);
            ui.pop_id();
        }
        Ok(())
    }
}