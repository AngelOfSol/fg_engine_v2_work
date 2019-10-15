use crate::assets::Assets;
use crate::character::components::Particles;
use crate::graphics::Animation;
use crate::imgui_extra::UiExtensions;
use crate::ui::editor::Mode;
use ggez::Context;
use imgui::*;
use nfd::Response;
use std::path::PathBuf;

pub struct ParticlesUi {
    current_particle: Option<usize>,
    pub particle_keys: Vec<String>,
}

impl ParticlesUi {
    pub fn new(reference: &Particles) -> Self {
        Self {
            current_particle: None,
            particle_keys: reference.particles.keys().cloned().collect(),
        }
    }

    pub fn draw_ui(
        &mut self,
        ctx: &mut Context,
        assets: &mut Assets,
        ui: &Ui<'_>,
        data: &mut Particles,
    ) -> Option<Mode> {
        let mut ret = None;
        let id = ui.push_id("Particles");
        ui.rearrangable_list_box(
            im_str!("List"),
            &mut self.current_particle,
            &mut self.particle_keys,
            |item| im_str!("{}", item),
            5,
        );
        if ui.small_button(im_str!("Add")) {
            let path_result = nfd::open_file_multiple_dialog(Some("json"), None);
            match path_result {
                Ok(path) => match path {
                    Response::Cancel => (),
                    Response::Okay(path) => {
                        let animation =
                            Animation::load_from_json(ctx, assets, PathBuf::from(path)).unwrap();
                        self.particle_keys.push(animation.name.clone());
                        data.particles.insert(animation.name.clone(), animation);
                    }
                    Response::OkayMultiple(paths) => {
                        for path in paths {
                            let animation =
                                Animation::load_from_json(ctx, assets, PathBuf::from(path))
                                    .unwrap();
                            self.particle_keys.push(animation.name.clone());
                            data.particles.insert(animation.name.clone(), animation);
                        }
                    }
                },
                Err(err) => {
                    dbg!(err);
                }
            }
        }
        ui.same_line(0.0);
        if ui.small_button(im_str!("New")) {
            ret = Some(Mode::New);
        }
        if let Some(particle) = self.current_particle {
            let particle_key = &self.particle_keys[particle];
            let mut new_key = particle_key.to_owned();
            let fix_name = if let Some(particle) = &mut data.particles.get_mut(particle_key) {
                ui.same_line(0.0);
                if ui.small_button(im_str!("Edit")) {
                    ret = Some(Mode::Edit(particle.name.clone()));
                }
                ui.input_string(im_str!("Name"), &mut new_key)
            } else {
                false
            };
            if fix_name {
                let mut particle_data = data.particles.remove(particle_key).unwrap();
                particle_data.name = new_key.clone();
                data.particles.insert(new_key.clone(), particle_data);
                self.particle_keys[particle] = new_key;
            }
        }
        id.pop(ui);
        ret
    }
}
