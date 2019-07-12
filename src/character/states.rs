use std::collections::HashMap;

use crate::character_state::CharacterState;

use serde::{Deserialize, Serialize};

use imgui::*;

use crate::imgui_extra::UiExtensions;

use std::path::PathBuf;

use ggez::{Context, GameResult};

use crate::assets::Assets;

use crate::editor::Mode;

use crate::typedefs::HashId;

#[derive(Debug, Serialize, Deserialize)]
pub struct States<Id>
where
    Id: HashId,
{
    #[serde(flatten)]
    pub rest: HashMap<String, CharacterState<Id>>,
    #[serde(skip)]
    _secret: (),
}

impl<Id: HashId> States<Id> {
    pub fn new() -> Self {
        Self {
            rest: HashMap::new(),
            _secret: (),
        }
    }

    pub fn get_state(&self, key: &str) -> &CharacterState<Id> {
        match key {
            _ => &self.rest[key],
        }
    }
    pub fn replace_state(&mut self, key: String, data: CharacterState<Id>) {
        match key.as_str() {
            _ => {
                self.rest.insert(key, data);
            }
        }
    }
    pub fn guarentee_unique_key<S: Into<String>>(&self, key: S) -> String {
        let base = key.into();
        let mut new_key = base.clone();
        let mut counter = 1;
        loop {
            if self.rest.contains_key(&new_key) {
                new_key = format!("{} ({})", base, counter);
                counter += 1;
            } else {
                break;
            }
        }
        new_key
    }
}

pub struct StatesUi {}

impl StatesUi {
    pub fn new() -> Self {
        StatesUi {}
    }
    pub fn draw_ui(
        &mut self,
        ctx: &mut Context,
        assets: &mut Assets,
        ui: &Ui<'_>,
        data: &mut States<String>,
    ) -> GameResult<Option<Mode>> {
        let mut ret = None;
        ui.text(im_str!("States:"));
        ui.same_line(0.0);
        if ui.small_button(im_str!("Load")) {
            if let Ok(nfd::Response::Okay(path)) = nfd::open_file_dialog(Some("json"), None) {
                let path = PathBuf::from(path);
                let name = path.file_stem().unwrap().to_str().unwrap().to_owned();
                let name = data.guarentee_unique_key(name);
                data.rest
                    .insert(name, CharacterState::load_from_json(ctx, assets, path)?);
            }
        }
        ui.same_line(0.0);
        if ui.small_button(im_str!("New")) {
            data.rest.insert(
                data.guarentee_unique_key("new state"),
                CharacterState::new(),
            );
        }
        ui.separator();
        let mut to_delete = None;
        let mut to_change = None;

        let mut state_list = data.rest.iter_mut().collect::<Vec<_>>();
        state_list.sort_by(|lhs, rhs| lhs.0.cmp(rhs.0));

        for (idx, (name, value)) in state_list.into_iter().enumerate() {
            ui.push_id(&format!("Rest {} {}", idx, value.duration()));
            let mut buffer = name.clone();
            if ui.input_string(im_str!("Name"), &mut buffer) {
                to_change = Some((name.clone(), buffer));
            }
            ui.next_column();
            if ui.small_button(im_str!("Edit")) {
                ret = Some(Mode::Edit(name.to_owned()));
            }
            ui.same_line(0.0);
            if ui.small_button(im_str!("Load")) {
                if let Ok(nfd::Response::Okay(path)) = nfd::open_file_dialog(Some("json"), None) {
                    *value = CharacterState::load_from_json(ctx, assets, PathBuf::from(path))?;
                }
            }
            ui.same_line(0.0);
            if ui.small_button(im_str!("Delete")) {
                to_delete = Some(name.clone());
            }
            ui.separator();
            ui.pop_id();
        }

        if let Some(key) = to_delete {
            data.rest.remove(&key);
        }
        if let Some((old, new)) = to_change {
            let state = data.rest.remove(&old).unwrap();
            data.rest.insert(data.guarentee_unique_key(new), state);
        }

        Ok(ret)
    }
}

fn _required_state_helper(
    ui: &Ui<'_>,
    ctx: &mut Context,
    assets: &mut Assets,
    name: &str,
    data: &mut CharacterState<String>,
) -> GameResult<Option<Mode>> {
    ui.push_id(name);
    let mut ret = None;
    ui.text(im_str!("{}", name));
    ui.same_line(0.0);
    if ui.small_button(im_str!("Edit")) {
        ret = Some(Mode::Edit(name.to_owned()));
    }
    ui.same_line(0.0);
    if ui.small_button(im_str!("Load")) {
        if let Ok(nfd::Response::Okay(path)) = nfd::open_file_dialog(Some("json"), None) {
            *data = CharacterState::load_from_json(ctx, assets, PathBuf::from(path))?;
        }
    }
    ui.pop_id();

    Ok(ret)
}
