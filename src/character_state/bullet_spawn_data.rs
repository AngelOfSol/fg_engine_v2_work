use crate::imgui_extra::UiExtensions;
use crate::typedefs::collision::{Int, Vec2};
use imgui::{im_str, Ui};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct BulletSpawn {
    pub bullet_id: String,
    pub frame: usize,
    pub offset: Vec2,
    #[serde(flatten)]
    pub properties: HashMap<String, Int>,
}

impl Default for BulletSpawn {
    fn default() -> Self {
        Self {
            bullet_id: "".to_owned(),
            frame: 0,
            offset: Vec2::new(0_00, 0_00),
            properties: HashMap::new(),
        }
    }
}

impl BulletSpawn {
    pub fn new(bullet_id: String, properties: &HashSet<String>) -> Self {
        Self {
            bullet_id,
            frame: 0,
            offset: Vec2::new(0_00, 0_00),
            properties: properties.iter().map(|key| (key.clone(), 0)).collect(),
        }
    }
    pub fn fix_properties(&mut self, properties: &HashSet<String>) {
        self.properties = properties
            .iter()
            .map(|key| (key.clone(), self.properties.remove(key).unwrap_or(0)))
            .collect();
    }
}

pub struct BulletSpawnUi;

impl BulletSpawnUi {
    pub fn draw_ui(
        ui: &Ui<'_>,
        data: &mut BulletSpawn,
        bullets: &HashMap<String, HashSet<String>>,
    ) {
        let id = ui.push_id("bullet");
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

        data.offset /= 100;
        ui.input_vec2_int(im_str!("Offset"), &mut data.offset);
        data.offset *= 100;

        ui.separator();
        imgui::ChildWindow::new(im_str!("child frame"))
            .size([0.0, 0.0])
            .build(ui, || {
                let mut props = data.properties.iter_mut().collect::<Vec<_>>();
                props.sort();
                for (key, value) in props {
                    ui.input_whole(&im_str!("{}", key), value).unwrap();
                }
            });
        id.pop(ui);
    }
}
