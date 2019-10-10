use crate::imgui_extra::UiExtensions;
use crate::typedefs::collision::Int;
use imgui::{im_str, Ui};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Default)]
pub struct BulletSpawn {
    pub bullet_id: String,
    pub frame: usize,
    pub properties: HashMap<String, Int>,
}

impl BulletSpawn {
    pub fn new(bullet_id: String, properties: &HashSet<String>) -> Self {
        Self {
            bullet_id,
            frame: 0,
            properties: properties.iter().map(|key| (key.clone(), 0)).collect(),
        }
    }
}

pub struct BulletSpawnUi;

impl BulletSpawnUi {
    pub fn draw_ui(
        ui: &Ui<'_>,
        data: &mut BulletSpawn,
        bullets: &HashMap<String, HashSet<String>>,
    ) {
        if ui.combo_items(
            im_str!("ID"),
            &mut data.bullet_id,
            &bullets.keys().cloned().collect::<Vec<_>>(),
            &|item| im_str!("{}", item).into(),
        ) {
            data.properties = bullets[&data.bullet_id]
                .iter()
                .map(|key| (key.clone(), data.properties.remove(key).unwrap_or(0)))
                .collect();
        }

        let _ = ui.input_whole(im_str!("Spawn Frame"), &mut data.frame);
        ui.separator();
        for (key, value) in data.properties.iter_mut() {
            ui.input_whole(&im_str!("{}", key), value).unwrap();
        }
    }
}
