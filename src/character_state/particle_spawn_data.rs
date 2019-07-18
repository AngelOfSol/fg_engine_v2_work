use crate::typedefs::collision::Vec2;
use serde::{Deserialize, Serialize};

use crate::imgui_extra::UiExtensions;
use imgui::*;

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
    particle_list_ui: Vec<ImString>,
}

impl ParticleSpawnUi {
    pub fn new(particle_list: Vec<String>) -> Self {
        Self {
            particle_list_ids: particle_list.clone(),
            particle_list_ui: particle_list
                .into_iter()
                .map(|item| im_str!("{}", item))
                .collect(),
        }
    }
    pub fn draw_ui(&mut self, ui: &Ui<'_>, data: &mut ParticleSpawn<String>) {
        let mut buffer_idx = self
            .particle_list_ids
            .iter()
            .position(|item| &data.particle_id == item)
            .map(|item| item as i32);
        if let Some(ref mut buffer_idx) = buffer_idx {
            ui.combo(
                im_str!("ID"),
                buffer_idx,
                &self.particle_list_ui.iter().collect::<Vec<_>>(),
                5,
            );
            if *buffer_idx >= 0 {
                data.particle_id = self.particle_list_ids[*buffer_idx as usize].clone();
            }
        }

        let _ = ui.input_whole(im_str!("Spawn Frame"), &mut data.frame);
        data.offset /= 100;
        ui.input_vec2_int(im_str!("Offset"), &mut data.offset);
        data.offset *= 100;
    }
}
