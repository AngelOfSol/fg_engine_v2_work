use super::Particle;
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
) -> GameResult<Particle> {
    let file = File::open(&path).unwrap();
    let buf_read = BufReader::new(file);
    let mut particle = serde_json::from_reader::<_, Particle>(buf_read).unwrap();
    path.pop();
    load(ctx, assets, &mut particle, path)?;
    Ok(particle)
}

pub fn load(
    ctx: &mut Context,
    assets: &mut Assets,
    particle: &mut Particle,
    mut path: PathBuf,
) -> GameResult<()> {
    for animation in particle.animations.iter_mut() {
        path.push(&animation.name);
        match Animation::load(ctx, assets, animation, path.clone()) {
            Ok(_) => (),
            Err(_) => {
                let mut path = path.clone();
                path.pop();
                path.pop();
                Animation::load(ctx, assets, animation, path)?
            }
        };
        path.pop();
    }

    Ok(())
}

pub fn save(
    ctx: &mut Context,
    assets: &mut Assets,
    particle: &Particle,
    mut path: PathBuf,
) -> GameResult<()> {
    let mut json = File::create(&path)?;
    serde_json::to_writer(&mut json, &particle)
        .map_err(|err| GameError::FilesystemError(format!("{}", err)))?;
    path.pop();

    for animation in particle.animations.iter() {
        path.push(&animation.name);
        Animation::save(ctx, assets, animation, path.clone())?;
        path.pop();
    }

    Ok(())
}
