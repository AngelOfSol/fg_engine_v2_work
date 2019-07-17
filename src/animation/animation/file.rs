use crate::animation::Sprite;

use crate::assets::Assets;

use ggez::{Context, GameError, GameResult};

use super::Animation;

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub fn load_from_json(
    ctx: &mut Context,
    assets: &mut Assets,
    mut path: PathBuf,
) -> GameResult<Animation> {
    let file = File::open(&path).unwrap();
    let buf_read = BufReader::new(file);
    let animation = serde_json::from_reader::<_, Animation>(buf_read).unwrap();
    path.pop();
    Animation::load(ctx, assets, &animation, path)?;
    Ok(animation)
}
pub fn load(
    ctx: &mut Context,
    assets: &mut Assets,
    animation: &Animation,
    mut path: PathBuf,
) -> GameResult<()> {
    path.push(&animation.name);
    for (sprite, _) in &animation.frames {
        Sprite::load(ctx, assets, sprite, path.clone())?;
    }
    Ok(())
}

pub fn save(
    ctx: &mut Context,
    assets: &mut Assets,
    animation: &Animation,
    mut path: PathBuf,
) -> GameResult<()> {
    let mut json = File::create(&path)?;
    serde_json::to_writer(&mut json, &animation)
        .map_err(|err| GameError::FilesystemError(format!("{}", err)))?;
    path.pop();
    path.push(&animation.name);
    std::fs::create_dir_all(&path)?;
    for (sprite, _) in &animation.frames {
        Sprite::save(ctx, assets, sprite, path.clone())?;
    }
    Ok(())
}
