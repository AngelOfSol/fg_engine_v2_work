use ggez::error::GameError;
use ggez::graphics;
use ggez::{Context, GameResult};

use crate::assets::Assets;

use std::io::Read;
use std::path::{Path, PathBuf};

use image::imageops::flip_vertical;
use image::png::PNGEncoder;
use image::{ColorType, ImageBuffer, Rgba};

use std::fs::File;
use std::io::BufWriter;

use super::Sprite;

pub fn load_image<S: Into<String>, P: AsRef<Path>>(
    key: S,
    path: P,
    ctx: &mut Context,
    assets: &mut Assets,
) -> GameResult<()> {
    let key = key.into();
    let image = match graphics::Image::new(ctx, &path) {
        Ok(result) => result,
        Err(GameError::ResourceNotFound(_, _)) => {
            let img = {
                let mut buf = Vec::new();
                let mut reader = std::fs::File::open(&path)?;
                let _ = reader.read_to_end(&mut buf)?;
                image::load_from_memory(&buf)?.to_rgba()
            };
            let (width, height) = img.dimensions();

            graphics::Image::from_rgba8(ctx, width as u16, height as u16, &img)?
        }
        Err(err) => return Err(err),
    };
    assets.images.insert(key, image);
    Ok(())
}

pub fn load(
    ctx: &mut Context,
    assets: &mut Assets,
    sprite: &Sprite,
    mut path: PathBuf,
) -> GameResult<()> {
    path.push(&sprite.image);
    load_image(sprite.image.clone(), &path, ctx, assets)?;
    Ok(())
}
pub fn save(
    ctx: &mut Context,
    assets: &mut Assets,
    sprite: &Sprite,
    mut path: PathBuf,
) -> GameResult<()> {
    path.push(&sprite.image);
    let image = &assets.images[&sprite.image];
    let output = File::create(&path)?;
    let writer = BufWriter::new(output);
    let png_writer = PNGEncoder::new(writer);

    let image: ImageBuffer<Rgba<_>, _> = ImageBuffer::from_raw(
        u32::from(image.width()),
        u32::from(image.height()),
        image.to_rgba8(ctx)?.to_vec(),
    )
    .unwrap();

    // image buffers are flipped in memory for ggez/OpenGL/gfx, so we have to unflip them
    let image = flip_vertical(&image);

    png_writer.encode(&image, image.width(), image.height(), ColorType::RGBA(8))?;

    Ok(())
}
