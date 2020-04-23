use crate::character::state::components::{GlobalParticle, ParticlePath, ParticleSpawn};
use crate::imgui_extra::UiExtensions;
use imgui::*;
use strum::IntoEnumIterator;

pub struct ParticleSpawnUi {}

impl ParticleSpawnUi {
    pub fn new() -> Self {
        Self {}
    }
    pub fn draw_ui(
        &mut self,
        ui: &Ui<'_>,
        particle_list_ids: &[String],
        data: &mut ParticleSpawn<String>,
    ) {
        ui.combo_items(
            im_str!("ID"),
            &mut data.particle_id,
            &particle_list_ids
                .iter()
                .map(|item| ParticlePath::Local(item.clone()))
                .chain(GlobalParticle::iter().map(|item| item.into()))
                .collect::<Vec<_>>(),
            &|item| im_str!("{}", item).into(),
        );

        let _ = ui.input_whole(im_str!("Spawn Frame"), &mut data.frame);
        data.offset /= 100;
        ui.input_vec2_whole(im_str!("Offset"), &mut data.offset);
        data.offset *= 100;
    }
}
