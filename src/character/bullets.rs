use crate::graphics::Animation;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::hitbox::Hitbox;

use crate::editor::Mode;

use crate::assets::Assets;
use ggez::Context;
use imgui::{im_str, Ui};

use crate::imgui_extra::UiExtensions;

use nfd::Response;

use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bullets {
    #[serde(flatten)]
    pub bullets: HashMap<String, BulletInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BulletInfo {
    pub animation: Animation,
    pub hitbox: Hitbox,
    pub properties: HashSet<String>,
}

impl BulletInfo {
    pub fn new(key: String) -> Self {
        Self {
            animation: Animation::new(key),
            hitbox: Hitbox::new(),
            properties: HashSet::new(),
        }
    }
}

pub struct BulletInfoUi {
    new_property: String,
}

impl BulletInfoUi {
    pub fn new() -> Self {
        Self {
            new_property: "".to_owned(),
        }
    }
    pub fn draw_ui(
        &mut self,
        ctx: &mut Context,
        assets: &mut Assets,
        ui: &Ui<'_>,
        data: &mut BulletInfo,
    ) -> Option<Mode> {
        let mut ret = None;
        ui.text(im_str!("Animation"));
        if ui.small_button(im_str!("Load")) {
            let path_result = nfd::open_file_multiple_dialog(Some("json"), None);
            match path_result {
                Ok(path) => match path {
                    Response::Cancel => (),
                    Response::Okay(path) => {
                        data.animation =
                            Animation::load_from_json(ctx, assets, PathBuf::from(path)).unwrap();
                    }
                    Response::OkayMultiple(paths) => {
                        data.animation =
                            Animation::load_from_json(ctx, assets, PathBuf::from(&paths[0]))
                                .unwrap();
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
        ui.same_line(0.0);
        if ui.small_button(im_str!("Edit")) {
            ret = Some(Mode::Edit("self".to_owned()));
        }
        ui.separator();
        ui.text(im_str!("Hitbox"));
        Hitbox::draw_ui(ui, &mut data.hitbox);
        ui.separator();
        ui.text(im_str!("Properties"));
        let mut to_delete = None;
        for item in data.properties.iter() {
            ui.text(im_str!("{}", item));
            ui.same_line(0.0);
            if ui.small_button(im_str!("Delete")) {
                to_delete = Some(item.clone());
            }
        }
        if let Some(item) = to_delete {
            data.properties.remove(&item);
        }
        ui.input_string(im_str!("##Property Name"), &mut self.new_property);
        ui.same_line(0.0);
        if ui.small_button(im_str!("Add##Property")) && self.new_property != "" {
            let new = std::mem::replace(&mut self.new_property, "".to_owned());
            data.properties.insert(new);
        }

        ret
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BulletSpawner {
    pub origin: crate::typedefs::collision::Vec2,
    #[serde(flatten)]
    pub properties: HashMap<String, crate::typedefs::collision::Int>,
}

impl Bullets {
    pub fn new() -> Self {
        Self {
            bullets: HashMap::new(),
        }
    }

    pub fn guarentee_unique_key<S: Into<String>>(&self, key: S) -> String {
        let base = key.into();
        let mut new_key = base.clone();
        let mut counter = 1;
        loop {
            if self.bullets.contains_key(&new_key) {
                new_key = format!("{} ({})", base, counter);
                counter += 1;
            } else {
                break;
            }
        }
        new_key
    }
}

impl Default for Bullets {
    fn default() -> Self {
        Self::new()
    }
}

pub struct BulletsUi {
    bullet_names: Vec<String>,
}
impl BulletsUi {
    pub fn new(data: &Bullets) -> Self {
        Self {
            bullet_names: data.bullets.keys().cloned().collect(),
        }
    }
    pub fn draw_ui(&mut self, ui: &Ui<'_>, data: &mut Bullets) -> Option<Mode> {
        let mut ret = None;
        ui.text(im_str!("Bullets:"));
        ui.same_line(0.0);
        if ui.small_button(im_str!("New")) {
            let key = data.guarentee_unique_key("new bullet");
            data.bullets
                .insert(key.clone(), BulletInfo::new(key.clone()));
            self.bullet_names.insert(0, key);
        }
        ui.separator();
        let mut to_delete = None;
        let mut to_change = None;

        for (idx, name) in self.bullet_names.iter().enumerate() {
            let id = ui.push_id(&format!("Rest {}", idx));
            let mut buffer = name.clone();
            if ui.input_string(im_str!("Name"), &mut buffer) {
                to_change = Some((name.clone(), buffer));
            }
            ui.next_column();
            if ui.small_button(im_str!("Edit")) {
                ret = Some(Mode::Edit(name.clone()));
            }
            ui.same_line(0.0);
            if ui.small_button(im_str!("Delete")) {
                to_delete = Some(name.clone());
            }
            ui.separator();
            id.pop(ui);
        }

        if let Some(key) = to_delete {
            if let Some(idx) = self.bullet_names.iter().position(|item| item == &key) {
                self.bullet_names.remove(idx);
                data.bullets.remove(&key);
            }
        }
        if let Some((old, new)) = to_change {
            let info = data.bullets.remove(&old).unwrap();
            let new = data.guarentee_unique_key(new);
            data.bullets.insert(new.clone(), info);
            if let Some(idx) = self.bullet_names.iter().position(|item| item == &old) {
                self.bullet_names.remove(idx);
                self.bullet_names.insert(idx, new);
            }
        }

        ret
    }
}
