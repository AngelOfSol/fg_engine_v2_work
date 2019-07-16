use crate::animation::Animation;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::assets::Assets;
use ggez::Context;

use crate::imgui_extra::UiExtensions;
use imgui::*;

use crate::editor::Mode;

use std::path::PathBuf;

use nfd::Response;

#[derive(Debug, Serialize, Deserialize)]
pub struct Particles {
    #[serde(flatten)]
    pub particles: HashMap<String, Animation>,
}
impl Particles {
    pub fn new() -> Self {
        Self {
            particles: HashMap::new(),
        }
    }

    pub fn guarentee_unique_key<S: Into<String>>(&self, key: S) -> String {
        let base = key.into();
        let mut new_key = base.clone();
        let mut counter = 1;
        loop {
            if self.particles.contains_key(&new_key) {
                new_key = format!("{} ({})", base, counter);
                counter += 1;
            } else {
                break;
            }
        }
        new_key
    }
}

impl Default for Particles {
    fn default() -> Self {
        Self::new()
    }
}

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
        ui.push_id("Particles");
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
        ui.pop_id();
        ret
    }
}
