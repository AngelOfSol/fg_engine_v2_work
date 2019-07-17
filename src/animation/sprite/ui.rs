use ggez::{Context, GameResult};

use crate::assets::Assets;

use crate::imgui_extra::UiExtensions;
use imgui::*;
use nfd::Response;

use super::{load_image, Sprite};

pub struct SpriteUi;

impl SpriteUi {
    pub fn draw_ui(
        ctx: &mut Context,
        assets: &mut Assets,
        ui: &Ui<'_>,
        sprite: &mut Sprite,
    ) -> GameResult<()> {
        ui.input_vec2_float(im_str!("Offset"), &mut sprite.offset);
        ui.separator();

        ui.input_vec2_float(im_str!("Scale"), &mut sprite.scale);
        ui.separator();

        ui.input_float(im_str!("Rotation"), &mut sprite.rotation)
            .build();
        ui.separator();

        let mut buffer = sprite.image.clone();
        if ui.input_string(im_str!("Name##Frame"), &mut buffer) {
            sprite.rename(buffer, assets);
        }

        if ui.small_button(im_str!("Load New Image")) {
            let result = nfd::open_file_dialog(Some("png"), None);
            if let Ok(Response::Okay(path)) = result {
                replace_asset(sprite.image.clone(), &path, ctx, assets)?;
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
