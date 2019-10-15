mod bullet_info;

use crate::character::components::{BulletInfo, Bullets};
use crate::imgui_extra::UiExtensions;
use crate::ui::editor::Mode;
pub use bullet_info::BulletInfoUi;
use imgui::{im_str, Ui};

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
