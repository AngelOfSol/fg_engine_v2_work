use super::AnimationGroup;
use crate::assets::Assets;
use crate::graphics::Animation;
use ggez::{Context, GameError, GameResult};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub fn load_from_json(
    ctx: &mut Context,
    assets: &mut Assets,
    mut path: PathBuf,
) -> GameResult<AnimationGroup> {
    let file = File::open(&path).unwrap();
    let buf_read = BufReader::new(file);
    let mut animation_group = serde_json::from_reader::<_, AnimationGroup>(buf_read).unwrap();
    let sub_path = path.file_stem().unwrap().to_owned();
    path.pop();
    path.push(sub_path);
    load(ctx, assets, &mut animation_group, path)?;
    Ok(animation_group)
}

pub fn load(
    ctx: &mut Context,
    assets: &mut Assets,
    animation_group: &mut AnimationGroup,
    path: PathBuf,
) -> GameResult<()> {
    for animation in animation_group.animations.iter_mut() {
        Animation::load(ctx, assets, animation, path.clone())?;
    }

    Ok(())
}

pub fn save(
    ctx: &mut Context,
    assets: &mut Assets,
    animation_group: &AnimationGroup,
    mut path: PathBuf,
) -> GameResult<()> {
    let name = path.file_stem().unwrap().to_str().unwrap().to_owned();
    let mut json = File::create(&path)?;
    serde_json::to_writer(&mut json, &animation_group)
        .map_err(|err| GameError::FilesystemError(format!("{}", err)))?;
    path.pop();
    path.push(&name);
    std::fs::create_dir_all(&path)?;

    for animation in animation_group.animations.iter() {
        path.push(&format!("{}.json", &animation.name));
        Animation::save(ctx, assets, animation, path.clone())?;

        path.pop();
    }

    Ok(())
}
