use crate::roster::Character;
use crate::typedefs::collision::Vec2;
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Properties {
    pub health: i32,
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
    pub max_spirit_gauge: i32,

    #[serde(default)]
    pub character: Character,
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
fn default_max_spirit_gauge() -> i32 {
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
            character: Default::default(),
        }
    }
}
