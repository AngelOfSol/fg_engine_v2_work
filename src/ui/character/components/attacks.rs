mod attack_info;

use crate::character::components::{AttackInfo, Attacks};
use crate::imgui_extra::UiExtensions;
pub use attack_info::AttackInfoUi;
use imgui::{im_str, Ui};

pub struct AttacksUi {
    attack_names: Vec<String>,
}
impl AttacksUi {
    pub fn new(data: &Attacks) -> Self {
        AttacksUi {
            attack_names: data.attacks.keys().cloned().collect(),
        }
    }
    pub fn draw_ui(&mut self, ui: &Ui<'_>, data: &mut Attacks) -> Option<String> {
        let mut ret = None;
        ui.text(im_str!("Attacks:"));
        ui.same_line(0.0);
        if ui.small_button(im_str!("New")) {
            let key = data.guarentee_unique_key("new attack");
            data.attacks.insert(key.clone(), AttackInfo::default());
            self.attack_names.insert(0, key);
        }
        ui.separator();
        let mut to_delete = None;
        let mut to_change = None;

        for (idx, name) in self.attack_names.iter().enumerate() {
            let id = ui.push_id(&format!("Rest {}", idx));
            let mut buffer = name.clone();
            if ui.input_string(im_str!("Name"), &mut buffer) {
                to_change = Some((name.clone(), buffer));
            }
            ui.next_column();
            if ui.small_button(im_str!("Edit")) {
                ret = Some(name.clone());
            }
            ui.same_line(0.0);
            if ui.small_button(im_str!("Delete")) {
                to_delete = Some(name.clone());
            }
            ui.separator();
            id.pop(ui);
        }

        if let Some(key) = to_delete {
            if let Some(idx) = self.attack_names.iter().position(|item| item == &key) {
                self.attack_names.remove(idx);
                data.attacks.remove(&key);
            }
        }
        if let Some((old, new)) = to_change {
            let info = data.attacks.remove(&old).unwrap();
            let new = data.guarentee_unique_key(new);
            data.attacks.insert(new.clone(), info);
            if let Some(idx) = self.attack_names.iter().position(|item| item == &old) {
                self.attack_names.remove(idx);
                self.attack_names.insert(idx, new);
            }
        }

        ret
    }
}
