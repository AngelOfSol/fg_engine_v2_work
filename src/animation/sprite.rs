use ggez::error::GameError;
use ggez::graphics;
use ggez::graphics::{Color, DrawMode, DrawParam, Mesh, Rect};
use ggez::{Context, GameResult};

use serde::{Deserialize, Serialize};

use crate::assets::Assets;

use std::io::Read;
use std::path::Path;

use crate::typedefs::graphics::{Matrix4, Vec2, Vec3};

use crate::imgui_extra::UiExtensions;
use imgui::*;
use nfd::Response;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Sprite {
    pub offset: Vec2,
    pub image: String,
    pub rotation: f32,
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
    pub fn new<S: Into<String>>(path: S) -> Self {
        Self {
            offset: nalgebra::zero(),
            image: path.into(),
            rotation: 0.0,
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

        let sprite_offset = Matrix4::new_translation(&Vec3::new(self.offset.x, self.offset.y, 0.0));

        let transform = world * image_offset * sprite_offset;

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
            ui.input_float(im_str!("X"), &mut sprite.offset.x).build();
            ui.input_float(im_str!("Y"), &mut sprite.offset.y).build();
            ui.separator();
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
    let new_name = new_name.into();
    if let Some(asset) = asset {
        assets.images.insert(new_name.clone(), asset);
    }
    sprite.image = new_name;
}
