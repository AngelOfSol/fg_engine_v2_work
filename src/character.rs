mod properties;
mod states;

pub use properties::Properties;
pub use states::States;

use crate::character_state::CharacterState;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::animation::Animation;
use crate::assets::Assets;

use crate::imgui_extra::UiExtensions;

use imgui::*;

use ggez::{Context, GameResult};

use std::cmp;

use nfd::Response;

use crate::timeline::{AtTime, Timeline};

use crate::typedefs::graphics::Matrix4;

use crate::game::Mode;

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use ggez::GameError;

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerCharacter {
    pub states: States,
    pub properties: Properties,
}

macro_rules! _save_fields {
    ($ctx:expr, $assets:expr, $path:expr, $obj:expr => [ $( $field:ident ),* ] ) => {
        $(
            CharacterState::save($ctx, $assets, &$obj.$field, $path.clone())?;
        )*
    };
    () => {
    };
}
macro_rules! _load_fields {
    ($ctx:expr, $assets:expr, $path:expr, $obj:expr => [ $( $field:ident ),* ] ) => {
        $(
            CharacterState::load($ctx, $assets, &$obj.$field, stringify!($field), $path.clone())?;
        )*
    };
    () => {
    };
}

impl PlayerCharacter {
    pub fn new() -> Self {
        PlayerCharacter {
            states: States::new(),
            properties: Properties::new(),
        }
    }

    pub fn load_from_json(
        ctx: &mut Context,
        assets: &mut Assets,
        mut path: PathBuf,
    ) -> GameResult<Self> {
        let file = File::open(&path).unwrap();
        let buf_read = BufReader::new(file);
        let player_character = serde_json::from_reader::<_, Self>(buf_read).unwrap();
        let character_file_name = path.file_stem().unwrap().to_str().unwrap().to_owned();
        path.pop();
        Self::load(ctx, assets, &player_character, &character_file_name, path)?;
        Ok(player_character)
    }
    pub fn load(
        ctx: &mut Context,
        assets: &mut Assets,
        player_character: &Self,
        character_file_name: &str,
        mut path: PathBuf,
    ) -> GameResult<()> {
        path.push(&character_file_name);

        _load_fields!(ctx, assets, path, player_character.states => [idle]);

        for (name, state) in player_character.states.rest.iter() {
            CharacterState::load(ctx, assets, state, name, path.clone())?;
        }
        Ok(())
    }
    pub fn save(
        ctx: &mut Context,
        assets: &mut Assets,
        player_character: &Self,
        mut path: PathBuf,
    ) -> GameResult<()> {
        let character_file_name = path.file_stem().unwrap().to_str().unwrap().to_owned();
        let mut json = File::create(&path)?;
        serde_json::to_writer(&mut json, &player_character)
            .map_err(|err| GameError::FilesystemError(format!("{}", err)))?;

        path.pop();
        path.push(&character_file_name);
        std::fs::create_dir_all(&path)?;

        _save_fields!(ctx, assets, path, player_character.states => [idle]);

        for (state_name, state) in player_character.states.rest.iter() {
            path.push(state_name);
            CharacterState::save(ctx, assets, state, path.clone())?;
            path.pop();
        }
        Ok(())
    }
}
