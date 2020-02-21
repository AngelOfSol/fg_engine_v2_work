use crate::character::state::components::ParticleSpawn;
use crate::imgui_extra::UiExtensions;
use imgui::*;

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
            &particle_list_ids,
            &|item| im_str!("{}", item).into(),
        );

        let _ = ui.input_whole(im_str!("Spawn Frame"), &mut data.frame);
        data.offset /= 100;
        ui.input_vec2_whole(im_str!("Offset"), &mut data.offset);
        data.offset *= 100;
    }
}
