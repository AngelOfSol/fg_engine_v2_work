pub mod components;
pub mod state;

use crate::assets::Assets;
use crate::graphics::particle::Particle;
use crate::graphics::Animation;
use components::{Attacks, BulletInfo, Bullets, EditorStates, Particles, Properties, States};
use ggez::GameError;
use ggez::{Context, GameResult};
use serde::{Deserialize, Serialize};
use state::State;
use std::collections::HashSet;
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
    pub bullets: Bullets,
    #[serde(default)]
    pub attacks: Attacks,
    #[serde(default)]
    pub sounds: HashSet<String>,
}

impl PlayerCharacter {
    pub fn new() -> Self {
        PlayerCharacter {
            states: States::new(),
            properties: Properties::new(),
            particles: Particles::new(),
            bullets: Bullets::new(),
            attacks: Attacks::new(),
            sounds: HashSet::new(),
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
            Particle::load(ctx, assets, animation, path.clone())?;
            path.pop();
        }
        path.pop();
        path.push("bullets");
        for (_, BulletInfo { animation, .. }) in player_character.bullets.bullets.iter_mut() {
            Animation::load(ctx, assets, animation, path.clone())?;
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
            Particle::save(ctx, assets, particle, path.clone())?;
            path.pop();
        }
        path.pop();
        path.push("bullets");
        if path.exists() {
            std::fs::remove_dir_all(&path)?;
        }
        std::fs::create_dir_all(&path)?;
        for (key, BulletInfo { animation, .. }) in player_character.bullets.bullets.iter() {
            path.push(format!("{}.json", key));
            Animation::save(ctx, assets, animation, path.clone())?;
            path.pop();
        }
        path.pop();

        Ok(())
    }
}
