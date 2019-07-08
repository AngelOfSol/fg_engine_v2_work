use std::collections::HashMap;

use crate::character_state::CharacterState;

use serde::{Deserialize, Serialize};

use imgui::*;

use std::path::PathBuf;

use ggez::{Context, GameResult};

use crate::assets::Assets;

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
}

pub struct StatesUi {}

impl StatesUi {
    pub fn new() -> Self {
        StatesUi {}
    }
    pub fn draw_ui(&mut self, ctx:&mut Context, assets:&mut Assets, ui: &Ui<'_>, data: &mut States) -> GameResult<()> {
        _required_state_helper(ui, ctx, assets,  im_str!("idle"), &mut data.idle)?;

        ui.separator();

        if ui.small_button(im_str!("New State")) {}

        Ok(())
    }
}

fn _required_state_helper(ui: &Ui<'_>, ctx:&mut Context, assets:&mut Assets, name: &ImStr, data: &mut CharacterState) -> GameResult<()>{
    ui.text(name);
    ui.same_line(0.0);
    if ui.small_button(im_str!("Edit")) {}
    ui.same_line(0.0);
    if ui.small_button(im_str!("Load")) {
        if let Ok(nfd::Response::Okay(path)) = nfd::open_file_dialog(Some("json"), None){
            
            *data = CharacterState::load_from_json(ctx, assets, PathBuf::from(path))?;
        }
    }

    Ok(())
}
