use crate::typedefs::collision::Vec2;
use serde::{Deserialize, Serialize};

use crate::imgui_extra::UiExtensions;
use imgui::*;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct ParticleSpawn {
    pub particle_id: String,
    pub frame: usize,
    pub offset: Vec2,
}

impl ParticleSpawn {
    pub fn new() -> Self {
        Self {
            particle_id: "CHANGE ID".to_owned(),
            frame: 0,
            offset: Vec2::zeros(),
        }
    }
}

pub struct ParticleSpawnUi;

impl ParticleSpawnUi {
    pub fn draw_ui(ui: &Ui<'_>, data: &mut ParticleSpawn) {
        ui.input_string(im_str!("ID"), &mut data.particle_id);

        let _ = ui.input_whole(im_str!("Spawn Frame"), &mut data.frame);
        data.offset /= 100;
        ui.input_vec2_int(im_str!("Offset"), &mut data.offset);
        data.offset *= 100;
    }
}
