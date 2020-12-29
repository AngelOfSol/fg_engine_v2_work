use crate::character::state::State;
use crate::imgui_extra::UiExtensions;
use ggez::GameResult;
use imgui::*;
use std::path::PathBuf;

use crate::character::components::EditorStates;

pub struct StatesUi {
    state_name_keys: Vec<String>,
}

impl StatesUi {
    pub fn new(state: &EditorStates) -> Self {
        let mut state_name_keys: Vec<_> = state.rest.keys().cloned().collect();
        state_name_keys.sort();
        StatesUi { state_name_keys }
    }
    pub fn draw_ui(&mut self, ui: &Ui<'_>, data: &mut EditorStates) -> GameResult<Option<String>> {
        let mut ret = None;
        ui.text(im_str!("States:"));
        ui.same_line(0.0);
        if ui.small_button(im_str!("Load")) {
            if let Ok(nfd::Response::Okay(path)) = nfd::open_file_dialog(Some("json"), None) {
                let path = PathBuf::from(path);
                let name = path.file_stem().unwrap().to_str().unwrap().to_owned();
                let name = data.guarentee_unique_key(name);
                data.rest.insert(name, State::load_from_json(path));
            }
        }
        ui.same_line(0.0);
        if ui.small_button(im_str!("New")) {
            let key = data.guarentee_unique_key("new state");
            data.rest.insert(key.clone(), State::new());
            self.state_name_keys.insert(0, key);
        }
        ui.separator();
        let mut to_delete = None;
        let mut to_change = None;

        let mut sort = false;

        for (idx, name) in self.state_name_keys.iter().enumerate() {
            let value = data.rest.get_mut(name).unwrap();

            let id = ui.push_id(&format!("Rest {} {:?}", idx, value.on_expire));
            let mut buffer = name.clone();
            if ui.input_string(im_str!("Name"), &mut buffer) {
                to_change = Some((name.clone(), buffer));
            }
            if ui.is_item_deactivated_after_edit() {
                sort = true;
            }

            ui.next_column();
            if ui.small_button(im_str!("Edit")) {
                ret = Some(name.clone());
            }
            ui.same_line(0.0);
            if ui.small_button(im_str!("Load")) {
                if let Ok(nfd::Response::Okay(path)) = nfd::open_file_dialog(Some("json"), None) {
                    *value = State::load_from_json(PathBuf::from(path));
                }
            }
            ui.same_line(0.0);
            if ui.small_button(im_str!("Delete")) {
                to_delete = Some(name.clone());
            }
            ui.separator();
            id.pop(ui);
        }

        if let Some(key) = to_delete {
            if let Some(idx) = self.state_name_keys.iter().position(|item| item == &key) {
                self.state_name_keys.remove(idx);
                data.rest.remove(&key);
                // TODO have deleting a state delete all instances of it from the rest of hte state references
            }
        }
        if let Some((old, new)) = to_change {
            let state = data.rest.remove(&old).unwrap();

            for fix_state in data.rest.values_mut() {
                if fix_state.on_expire.state_id == old {
                    fix_state.on_expire.state_id = new.clone();
                }
            }

            let new = data.guarentee_unique_key(new);
            data.rest.insert(new.clone(), state);
            if let Some(idx) = self.state_name_keys.iter().position(|item| item == &old) {
                self.state_name_keys.remove(idx);
                self.state_name_keys.insert(idx, new);
            }
        }

        if sort {
            self.state_name_keys.sort();
        }
        Ok(ret)
    }
}
