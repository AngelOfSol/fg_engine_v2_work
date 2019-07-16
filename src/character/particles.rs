use crate::animation::Animation;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::assets::Assets;
use ggez::Context;

use crate::imgui_extra::UiExtensions;
use imgui::*;

use std::path::PathBuf;

use nfd::Response;

#[derive(Debug, Serialize, Deserialize)]
pub struct Particles {
    #[serde(flatten)]
    particles: HashMap<String, Animation>,
}
impl Particles {
    pub fn new() -> Self {
        Self {
            particles: HashMap::new(),
        }
    }
}

impl Default for Particles {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ParticlesUi {
    current_particle: Option<usize>,
    particle_keys: Vec<String>,
}

impl ParticlesUi {
    pub fn new(reference: &Particles) -> Self {
        Self {
            current_particle: None,
            particle_keys: reference.particles.keys().cloned().collect(),
        }
    }

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        assets: &mut Assets,
        ui: &Ui<'_>,
        data: &mut Particles,
    ) {
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
                        data.particles.insert(animation.name.clone(), animation);
                    }
                    Response::OkayMultiple(paths) => {
                        for path in paths {
                            let animation =
                                Animation::load_from_json(ctx, assets, PathBuf::from(path))
                                    .unwrap();
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
            //ret = Some(Mode::New);
        } /*
          if let Some(animation) = self.current_animation {
              if let Some(animation) = &mut data.animations.get_mut(animation) {
                  ui.same_line(0.0);
                  if ui.small_button(im_str!("Edit")) {
                      ret = Some(Mode::Edit(animation.animation.name.clone()));
                  }
                  AnimationDataUi::new().draw_ui(ui, animation)?;
              }
          }
          ui.separator();*/
        ui.pop_id();
    }
}
