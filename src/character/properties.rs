use crate::imgui_extra::UiExtensions;
use crate::typedefs::collision::Vec2;
use imgui::*;

use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Properties {
    pub health: u32,
    pub name: String,

    #[serde(default = "default_neutral_jump_accel")]
    pub neutral_jump_accel: Vec2,
    #[serde(default = "default_neutral_super_jump_accel")]
    pub neutral_super_jump_accel: Vec2,

    #[serde(default = "default_directed_jump_accel")]
    pub directed_jump_accel: Vec2,
    #[serde(default = "default_directed_super_jump_accel")]
    pub directed_super_jump_accel: Vec2,

    #[serde(default = "default_max_air_actions")]
    pub max_air_actions: usize,
    #[serde(default = "default_max_spirit_gauge")]
    pub max_spirit_gauge: usize,

    #[serde(skip)]
    _secret: (),
}

fn default_neutral_jump_accel() -> Vec2 {
    Vec2::new(0_00, 8_00)
}
fn default_neutral_super_jump_accel() -> Vec2 {
    Vec2::new(0_00, 10_00)
}
fn default_directed_jump_accel() -> Vec2 {
    Vec2::new(2_00, 7_00)
}
fn default_directed_super_jump_accel() -> Vec2 {
    Vec2::new(4_00, 8_80)
}
fn default_max_air_actions() -> usize {
    2
}
fn default_max_spirit_gauge() -> usize {
    500
}

impl Properties {
    pub fn new() -> Self {
        Self {
            health: 1,
            name: "new_chara".to_owned(),
            neutral_jump_accel: default_neutral_jump_accel(),
            neutral_super_jump_accel: default_neutral_super_jump_accel(),

            directed_jump_accel: default_directed_jump_accel(),
            directed_super_jump_accel: default_directed_super_jump_accel(),
            max_air_actions: default_max_air_actions(),
            max_spirit_gauge: default_max_spirit_gauge(),
            _secret: (),
        }
    }
}

pub struct PropertiesUi {}
impl PropertiesUi {
    pub fn draw_ui(ui: &Ui<'_>, data: &mut Properties) {
        let _ = ui.input_whole(im_str!("Health"), &mut data.health);
        ui.input_string(im_str!("Name"), &mut data.name);

        ui.input_whole(im_str!("Max Air Actions"), &mut data.max_air_actions)
            .unwrap();
        ui.input_whole(im_str!("Max Spirit Gauge"), &mut data.max_spirit_gauge)
            .unwrap();

        ui.separator();

        ui.input_vec2_int(im_str!("Neutral Jump"), &mut data.neutral_jump_accel);
        ui.input_vec2_int(
            im_str!("Neutral Super Jump"),
            &mut data.neutral_super_jump_accel,
        );
        ui.input_vec2_int(im_str!("Directed Jump"), &mut data.directed_jump_accel);
        ui.input_vec2_int(
            im_str!("Directed Super Jump"),
            &mut data.directed_super_jump_accel,
        );
    }
}
