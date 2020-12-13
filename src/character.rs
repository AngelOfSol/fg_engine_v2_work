pub mod components;
pub mod state;

use crate::assets::Assets;
use crate::graphics::animation_group::AnimationGroup;
use components::{Attacks, EditorStates, Particles, Properties, States};
use ggez::GameError;
use ggez::{Context, GameResult};
use serde::{Deserialize, Serialize};
use state::State;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct PlayerCharacter {
    pub states: EditorStates,
    pub properties: Properties,
    #[serde(default)]
    pub particles: Particles,

    #[serde(default)]
    pub attacks: Attacks,
    #[serde(default)]
    pub sounds: HashSet<String>,

    #[serde(default)]
    pub graphics: HashMap<String, AnimationGroup>,
}

impl PlayerCharacter {
    pub fn new() -> Self {
        PlayerCharacter {
            states: States::new(),
            properties: Properties::new(),
            particles: Particles::new(),
            attacks: Attacks::new(),
            sounds: HashSet::new(),
            graphics: HashMap::new(),
        }
    }

    pub fn load_from_json(
        ctx: &mut Context,
        assets: &mut Assets,
        mut path: PathBuf,
    ) -> GameResult<Self> {
        let file = File::open(&path).unwrap();
        let buf_read = BufReader::new(file);
        let mut player_character = serde_json::from_reader::<_, Self>(buf_read).unwrap();
        let character_file_name = path.file_stem().unwrap().to_str().unwrap().to_owned();
        path.pop();
        Self::load(
            ctx,
            assets,
            &mut player_character,
            &character_file_name,
            path,
        )?;
        Ok(player_character)
    }
    pub fn load(
        ctx: &mut Context,
        assets: &mut Assets,
        player_character: &mut Self,
        character_file_name: &str,
        mut path: PathBuf,
    ) -> GameResult<()> {
        path.push(&character_file_name);
        path.push("states");

        for (name, state) in player_character.states.rest.iter_mut() {
            State::load(ctx, assets, state, name, path.clone())?;
        }
        path.pop();
        path.push("particles");
        for (key, animation) in player_character.particles.particles.iter_mut() {
            path.push(key);
            AnimationGroup::load(ctx, assets, animation, path.clone())?;
            path.pop();
        }
        path.pop();

        path.push("graphics");
        for (key, animation_group) in player_character.graphics.iter_mut() {
            path.push(key);
            AnimationGroup::load(ctx, assets, animation_group, path.clone())?;
            path.pop();
        }
        path.pop();
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
        path.push("states");

        if path.exists() {
            std::fs::remove_dir_all(&path)?;
        }
        std::fs::create_dir_all(&path)?;

        for (state_name, state) in player_character.states.rest.iter() {
            path.push(format!("{}.json", state_name));
            State::save(ctx, assets, state, path.clone())?;
            path.pop();
        }
        path.pop();

        path.push("particles");
        if path.exists() {
            std::fs::remove_dir_all(&path)?;
        }
        std::fs::create_dir_all(&path)?;
        for (name, particle) in player_character.particles.particles.iter() {
            path.push(format!("{}.json", name));
            AnimationGroup::save(ctx, assets, particle, path.clone())?;
            path.pop();
        }
        path.pop();

        path.push("graphics");
        if path.exists() {
            std::fs::remove_dir_all(&path)?;
        }
        std::fs::create_dir_all(&path)?;
        for (name, animation_group) in player_character.graphics.iter() {
            path.push(format!("{}.json", name));
            AnimationGroup::save(ctx, assets, animation_group, path.clone())?;
            path.pop();
        }
        path.pop();
        Ok(())
    }
}
