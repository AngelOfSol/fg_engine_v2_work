use ggez::{Context, GameResult};

use crate::assets::Assets;

use crate::imgui_extra::UiExtensions;
use imgui::*;
use nfd::Response;

use super::{load_image, Sprite};

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
                sprite.rename(buffer, assets);
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
pub fn rename<S: Into<String>>(sprite: &mut Sprite, new_name: S, assets: &mut Assets) {
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
