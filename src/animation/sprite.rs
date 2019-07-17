use ggez::error::GameError;
use ggez::graphics;
use ggez::graphics::{Color, DrawMode, DrawParam, Mesh, Rect};
use ggez::{Context, GameResult};

use serde::{Deserialize, Serialize};

use crate::assets::Assets;

use std::io::Read;
use std::path::{Path, PathBuf};

use crate::typedefs::graphics::{up_dimension, Matrix4, Vec2, Vec3};

use crate::imgui_extra::UiExtensions;
use imgui::*;
use nfd::Response;

use image::imageops::flip_vertical;
use image::png::PNGEncoder;
use image::{ColorType, ImageBuffer, Rgba};

use std::fs::File;
use std::io::BufWriter;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sprite {
    pub offset: Vec2,
    pub image: String,
    pub rotation: f32,
    #[serde(default = "default_scale")]
    pub scale: Vec2,
}

fn default_scale() -> Vec2 {
    Vec2::new(1.0, 1.0)
}

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

impl Sprite {
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

    pub fn new<S: Into<String>>(path: S) -> Self {
        Self {
            offset: nalgebra::zero(),
            image: path.into(),
            rotation: 0.0,
            scale: default_scale(),
        }
    }

    pub fn load_image(&self, ctx: &mut Context, assets: &mut Assets) -> GameResult<()> {
        if assets.images.contains_key(&self.image) {
            return Ok(());
        }
        load_image(self.image.clone(), &self.image, ctx, assets)
    }

    pub fn draw_ex(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        world: Matrix4,
        debug: bool,
    ) -> GameResult<()> {
        let image = assets
            .images
            .get(&self.image)
            .ok_or_else(|| GameError::ResourceNotFound(self.image.clone(), Vec::new()))?;

        let image_offset = Matrix4::new_translation(&Vec3::new(
            -f32::from(image.width()) / 2.0,
            -f32::from(image.height()) / 2.0,
            0.0,
        ));

        let sprite_scale = Matrix4::new_nonuniform_scaling(&up_dimension(self.scale));

        let sprite_offset = Matrix4::new_translation(&Vec3::new(self.offset.x, self.offset.y, 0.0));

        let transform = world * sprite_scale * image_offset * sprite_offset;

        graphics::set_transform(ctx, transform);
        graphics::apply_transformations(ctx)?;

        graphics::draw(ctx, image, DrawParam::default())?;

        if debug {
            let rectangle = Mesh::new_rectangle(
                ctx,
                DrawMode::stroke(1.0),
                Rect::new(
                    0.0,
                    0.0,
                    f32::from(image.width()),
                    f32::from(image.height()),
                ),
                Color::new(1.0, 0.0, 0.0, 1.0),
            )?;
            graphics::draw(ctx, &rectangle, DrawParam::default())?;
        }
        graphics::set_transform(ctx, Matrix4::identity());
        graphics::apply_transformations(ctx)?;

        Ok(())
    }
    pub fn draw_debug(&self, ctx: &mut Context, assets: &Assets, world: Matrix4) -> GameResult<()> {
        self.draw_ex(ctx, assets, world, true)
    }

    pub fn draw(&self, ctx: &mut Context, assets: &Assets, world: Matrix4) -> GameResult<()> {
        self.draw_ex(ctx, assets, world, false)
    }
}

pub struct SpriteUi;

impl SpriteUi {
    pub fn new() -> Self {
        Self
    }

    pub fn draw_ui(
        &mut self,
        ctx: &mut Context,
        assets: &mut Assets,
        ui: &Ui<'_>,
        sprite: &mut Sprite,
    ) -> GameResult<()> {
        if ui
            .collapsing_header(im_str!("Offset"))
            .default_open(true)
            .build()
        {
            ui.push_id("Offset");
            ui.input_float(im_str!("X"), &mut sprite.offset.x).build();
            ui.input_float(im_str!("Y"), &mut sprite.offset.y).build();
            ui.separator();
            ui.pop_id();
        }
        if ui
            .collapsing_header(im_str!("Scale"))
            .default_open(true)
            .build()
        {
            ui.push_id("Scale");
            ui.input_float(im_str!("X"), &mut sprite.scale.x).build();
            ui.input_float(im_str!("Y"), &mut sprite.scale.y).build();
            ui.separator();
            ui.pop_id();
        }
        ui.input_float(im_str!("Rotation"), &mut sprite.rotation)
            .build();
        ui.separator();
        if ui
            .collapsing_header(im_str!("Image"))
            .default_open(true)
            .build()
        {
            let mut buffer = sprite.image.clone();
            if ui.input_string(im_str!("Name##Frame"), &mut buffer) {
                rename_sprite(buffer, sprite, assets);
            }

            if ui.button(
                im_str!("Load New Image"),
                [
                    ui.get_content_region_avail()[0],
                    ui.get_text_line_height_with_spacing(),
                ],
            ) {
                let result = nfd::open_file_dialog(Some("png"), None);
                match result {
                    Ok(result) => match result {
                        Response::Cancel => (),
                        Response::Okay(path) => {
                            replace_asset(sprite.image.clone(), &path, ctx, assets)?;
                        }
                        Response::OkayMultiple(_) => {
                            println!("Cancelling because multiple images were specified.")
                        }
                    },
                    Err(err) => {
                        dbg!(err);
                    }
                }
            }
        }
        Ok(())
    }
}
fn replace_asset<S: Into<String>>(
    asset: S,
    path: &str,
    ctx: &mut Context,
    assets: &mut Assets,
) -> GameResult<()> {
    load_image(asset, path, ctx, assets)
}
pub fn rename_sprite<S: Into<String>>(new_name: S, sprite: &mut Sprite, assets: &mut Assets) {
    let asset = assets.images.remove(&sprite.image);
    let mut new_name = new_name.into();
    let mut counter = 1;
    while assets.images.contains_key(&new_name) {
        new_name = format!("({}){}", counter, new_name);
        counter += 1;
    }
    if let Some(asset) = asset {
        assets.images.insert(new_name.clone(), asset);
    }
    sprite.image = new_name;
}
