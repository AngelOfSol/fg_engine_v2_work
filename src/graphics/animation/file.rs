use super::Animation;
use crate::assets::Assets;
use crate::graphics::Sprite;
use ggez::{Context, GameError, GameResult};
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
    let mut animation = serde_json::from_reader::<_, Animation>(buf_read).unwrap();
    path.pop();
    load(ctx, assets, &mut animation, path)?;
    Ok(animation)
}

pub fn load(
    ctx: &mut Context,
    assets: &mut Assets,
    animation: &mut Animation,
    mut path: PathBuf,
) -> GameResult<()> {
    path.push(&animation.name);
    let paths: Vec<_> = animation
        .frames
        .iter()
        .enumerate()
        .map(|(idx, _)| animation.get_path_to_image(idx))
        .collect();
    for (file_name, (sprite, _)) in paths.iter().zip(animation.frames.iter_mut()) {
        path.push(file_name);
        Sprite::load(ctx, assets, sprite, path.clone())?;
        path.pop();
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
    for (idx, (sprite, _)) in animation.frames.iter().enumerate() {
        path.push(animation.get_path_to_image(idx));
        Sprite::save(ctx, assets, sprite, path.clone())?;
        path.pop();
    }
    Ok(())
}
