use crate::assets::Assets;
use crate::graphics::Animation;

use ggez::{Context, GameResult};

use crate::typedefs::StateId;

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use ggez::GameError;

use super::CharacterState;

pub fn load_from_json<Id: StateId, ParticleId: StateId, BulletId: StateId>(
    ctx: &mut Context,
    assets: &mut Assets,
    mut path: PathBuf,
) -> GameResult<CharacterState<Id, ParticleId, BulletId>> {
    let file = File::open(&path).unwrap();
    let buf_read = BufReader::new(file);
    let mut state = serde_json::from_reader::<_, CharacterState<_, _, _>>(buf_read).unwrap();
    let name = path.file_stem().unwrap().to_str().unwrap().to_owned();
    path.pop();
    CharacterState::load(ctx, assets, &mut state, &name, path)?;
    Ok(state)
}
pub fn load<Id: StateId, ParticleId: StateId, BulletId: StateId>(
    ctx: &mut Context,
    assets: &mut Assets,
    state: &mut CharacterState<Id, ParticleId, BulletId>,
    name: &str,
    mut path: PathBuf,
) -> GameResult<()> {
    path.push(name);
    for animation in state.animations.iter_mut() {
        Animation::load(ctx, assets, &mut animation.animation, path.clone())?;
    }
    Ok(())
}
pub fn save<Id: StateId, ParticleId: StateId, BulletId: StateId>(
    ctx: &mut Context,
    assets: &mut Assets,
    state: &CharacterState<Id, ParticleId, BulletId>,
    mut path: PathBuf,
) -> GameResult<()> {
    let name = path.file_stem().unwrap().to_str().unwrap().to_owned();

    let mut json = File::create(&path)?;
    serde_json::to_writer(&mut json, &state)
        .map_err(|err| GameError::FilesystemError(format!("{}", err)))?;

    path.pop();
    path.push(&name);
    std::fs::create_dir_all(&path)?;
    for animation in &state.animations {
        path.push(&format!("{}.json", &animation.animation.name));
        Animation::save(ctx, assets, &animation.animation, path.clone())?;
        path.pop();
    }
    Ok(())
}
