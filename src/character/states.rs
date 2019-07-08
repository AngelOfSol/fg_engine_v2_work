use std::collections::HashMap;

use crate::character_state::CharacterState;

use serde::{Deserialize, Serialize};

use imgui::*;

use std::path::PathBuf;

use ggez::{Context, GameResult};

use crate::assets::Assets;

use crate::game::Mode;

#[derive(Debug, Serialize, Deserialize)]
pub struct States {
    pub idle: CharacterState,
    #[serde(flatten)]
    pub rest: HashMap<String, CharacterState>,
    #[serde(skip)]
    _secret: (),
}

impl States {
    pub fn new() -> Self {
        Self {
            idle: CharacterState::new(),
            rest: HashMap::new(),
            _secret: (),
        }
    }

    pub fn get_state(&self, key: &str) -> &CharacterState {
        match key {
            "idle" => &self.idle,
            _ => &self.rest[key],
        }
    }
    pub fn replace_state(&mut self, key: String, data: CharacterState) {
        match key.as_str() {
            "idle" => self.idle = data,
            _ => {
                self.rest.insert(key, data);
            }
        }
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
        data: &mut States,
    ) -> GameResult<Option<Mode>> {
        let mut ret = None;
        ret = _required_state_helper(ui, ctx, assets, "idle", &mut data.idle)?.or(ret);

        ui.separator();

        if ui.small_button(im_str!("New State")) {}

        Ok(ret)
    }
}

fn _required_state_helper(
    ui: &Ui<'_>,
    ctx: &mut Context,
    assets: &mut Assets,
    name: &str,
    data: &mut CharacterState,
) -> GameResult<Option<Mode>> {
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

    Ok(ret)
}
