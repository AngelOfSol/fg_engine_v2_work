use crate::assets::Assets;
use crate::character::components::Particles;
use crate::graphics::particle::Particle;
use crate::imgui_extra::UiExtensions;
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
    ) -> Option<String> {
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
                        let path = PathBuf::from(path);
                        let name = path
                            .file_name()
                            .and_then(|path| path.to_str())
                            .unwrap()
                            .to_owned();
                        let animation = Particle::load_from_json(ctx, assets, path).unwrap();
                        self.particle_keys.push(name.clone());
                        data.particles.insert(name, animation);
                    }
                    Response::OkayMultiple(paths) => {
                        for path in paths {
                            let path = PathBuf::from(path);
                            let name = path
                                .file_name()
                                .and_then(|path| path.to_str())
                                .unwrap()
                                .to_owned();
                            let animation = Particle::load_from_json(ctx, assets, path).unwrap();
                            self.particle_keys.push(name.clone());
                            data.particles.insert(name.clone(), animation);
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
            data.particles
                .insert("new_particle".to_owned(), Particle::new());
            self.particle_keys.push("new_particle".to_owned());
            self.particle_keys.sort();
        }
        // TODO add deletion, and have it iterate through all the states removing every reference to the particle
        // TODO fix editing the name
        if let Some(particle) = self.current_particle {
            let particle_key = &self.particle_keys[particle];
            let id = ui.push_id(&particle_key);
            let mut new_key = particle_key.to_owned();
            let fix_name = if data.particles.contains_key(particle_key) {
                ui.same_line(0.0);
                if ui.small_button(im_str!("Edit")) {
                    ret = Some(particle_key.clone());
                }
                ui.input_string(im_str!("Name"), &mut new_key)
            } else {
                false
            };
            if fix_name {
                let particle_data = data.particles.remove(particle_key).unwrap();
                data.particles.insert(new_key.clone(), particle_data);
                self.particle_keys[particle] = new_key;
            }
            id.pop(ui)
        }
        id.pop(ui);
        ret
    }
}
