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

#[derive(Debug, Serialize, Deserialize)]
pub struct States {
    pub idle: CharacterState,
    #[serde(flatten)]
    pub rest: HashMap<String, CharacterState>,
    #[serde(skip)]
    _secret: (),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Properties {
    pub health: u32,
    pub name: String,
    #[serde(skip)]
    _secret: (),
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
            CharacterState::load($ctx, $assets, &$obj.$field, $path.clone())?;
        )*
    };
    () => {
    };
}

impl PlayerCharacter {
    pub fn new() -> Self {
        PlayerCharacter {
            states: States {
                idle: CharacterState::new(),
                rest: HashMap::new(),
                _secret: (),
            },
            properties: Properties {
                health: 1,
                name: "new_chara".to_owned(),
                _secret: (),
            },
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
        path.pop();
        Self::load(ctx, assets, &player_character, path)?;
        Ok(player_character)
    }
    pub fn load(
        ctx: &mut Context,
        assets: &mut Assets,
        player_character: &Self,
        mut path: PathBuf,
    ) -> GameResult<()> {
        path.push(&player_character.properties.name);

        _load_fields!(ctx, assets, path, player_character.states => [idle]);

        for state in player_character.states.rest.values() {
            CharacterState::load(ctx, assets, state, path.clone())?;
        }
        Ok(())
    }
    pub fn save(
        ctx: &mut Context,
        assets: &mut Assets,
        player_character: &Self,
        mut path: PathBuf,
    ) -> GameResult<()> {
        std::fs::create_dir_all(&path)?;

        path.push(format!("{}.json", &player_character.properties.name));
        let mut json = File::create(&path)?;
        serde_json::to_writer(&mut json, &player_character)
            .map_err(|err| GameError::FilesystemError(format!("{}", err)))?;

        path.pop();
        path.push(&player_character.properties.name);
        std::fs::create_dir_all(&path)?;

        _save_fields!(ctx, assets, path, player_character.states => [idle]);

        for state in player_character.states.rest.values() {
            CharacterState::save(ctx, assets, state, path.clone())?;
        }
        Ok(())
    }
}
