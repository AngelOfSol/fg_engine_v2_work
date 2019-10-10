use ggez::error::GameError;
use ggez::graphics;
use ggez::{Context, GameResult};

use crate::assets::Assets;

use std::io::Read;
use std::path::{Path, PathBuf};

use image::png::PNGEncoder;
use image::{ColorType, ImageBuffer, Rgba};

use std::fs::File;
use std::io::BufWriter;

use super::Sprite;

fn load_image_data<P: AsRef<Path>>(
    path: P,
    ctx: &mut Context,
    _assets: &mut Assets,
) -> GameResult<graphics::Image> {
    match graphics::Image::new(ctx, &path) {
        Ok(result) => Ok(result),
        Err(GameError::ResourceNotFound(_, _)) => {
            let img = {
                let mut buf = Vec::new();
                let mut reader = std::fs::File::open(&path)?;
                let _ = reader.read_to_end(&mut buf)?;
                image::load_from_memory(&buf).unwrap().to_rgba()
            };
            let (width, height) = img.dimensions();

            graphics::Image::from_rgba8(ctx, width as u16, height as u16, &img)
        }
        Err(err) => Err(err),
    }
}

pub fn load(
    ctx: &mut Context,
    assets: &mut Assets,
    sprite: &mut Sprite,
    path: PathBuf,
) -> GameResult<()> {
    let image = load_image_data(path, ctx, assets)?;
    sprite.image = Some(image);
    Ok(())
}

pub fn save(
    ctx: &mut Context,
    _assets: &mut Assets,
    sprite: &Sprite,
    path: PathBuf,
) -> GameResult<()> {
    let image = sprite.image.as_ref().unwrap();
    let output = File::create(&path)?;
    let writer = BufWriter::new(output);
    let png_writer = PNGEncoder::new(writer);

    let image: ImageBuffer<Rgba<_>, _> = ImageBuffer::from_raw(
        u32::from(image.width()),
        u32::from(image.height()),
        image.to_rgba8(ctx)?.to_vec(),
    )
    .unwrap();

    png_writer.encode(&image, image.width(), image.height(), ColorType::RGBA(8))?;

    Ok(())
}
