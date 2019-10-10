use crate::imgui_extra::UiExtensions;
use crate::typedefs::collision::Vec2;
use imgui::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct ParticleSpawn<Id> {
    pub particle_id: Id,
    pub frame: usize,
    pub offset: Vec2,
}

impl ParticleSpawn<String> {
    pub fn new(particle_id: String) -> Self {
        Self {
            particle_id,
            frame: 0,
            offset: Vec2::zeros(),
        }
    }
}

pub struct ParticleSpawnUi {
    particle_list_ids: Vec<String>,
}

impl ParticleSpawnUi {
    pub fn new(particle_list: Vec<String>) -> Self {
        Self {
            particle_list_ids: particle_list.clone(),
        }
    }
    pub fn draw_ui(&mut self, ui: &Ui<'_>, data: &mut ParticleSpawn<String>) {
        ui.combo_items(
            im_str!("ID"),
            &mut data.particle_id,
            &self.particle_list_ids,
            &|item| im_str!("{}", item).into(),
        );

        let _ = ui.input_whole(im_str!("Spawn Frame"), &mut data.frame);
        data.offset /= 100;
        ui.input_vec2_int(im_str!("Offset"), &mut data.offset);
        data.offset *= 100;
    }
}
