use super::State;
use crate::assets::Assets;
use crate::graphics::Animation;
use ggez::GameError;
use ggez::{Context, GameResult};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs::File;
use std::hash::Hash;
use std::io::BufReader;
use std::path::PathBuf;

pub fn load_from_json<
    Id: DeserializeOwned + Serialize + Eq + Hash + Default,
    ParticleId: DeserializeOwned + Serialize + Default,
    BulletSpawnInfo: DeserializeOwned + Serialize + Default,
    AttackId: DeserializeOwned + Serialize + Default,
>(
    ctx: &mut Context,
    assets: &mut Assets,
    mut path: PathBuf,
) -> GameResult<State<Id, ParticleId, BulletSpawnInfo, AttackId>> {
    let file = File::open(&path).unwrap();
    let buf_read = BufReader::new(file);
    let mut state = serde_json::from_reader::<_, State<_, _, _, _>>(buf_read).unwrap();
    let name = path.file_stem().unwrap().to_str().unwrap().to_owned();
    path.pop();
    State::load(ctx, assets, &mut state, &name, path)?;
    Ok(state)
}
pub fn load<Id, ParticleId, BulletSpawnInfo, AttackId>(
    ctx: &mut Context,
    assets: &mut Assets,
    state: &mut State<Id, ParticleId, BulletSpawnInfo, AttackId>,
    name: &str,
    mut path: PathBuf,
) -> GameResult<()> {
    path.push(name);
    for animation in state.animations.iter_mut() {
        Animation::load(ctx, assets, &mut animation.animation, path.clone())?;
    }
    Ok(())
}
pub fn save<
    Id: Serialize + Eq + Hash,
    ParticleId: Serialize,
    BulletSpawnInfo: Serialize,
    AttackId: Serialize,
>(
    ctx: &mut Context,
    assets: &mut Assets,
    state: &State<Id, ParticleId, BulletSpawnInfo, AttackId>,
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
