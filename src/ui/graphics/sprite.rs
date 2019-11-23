use crate::assets::Assets;
use crate::graphics::Sprite;
use crate::imgui_extra::UiExtensions;
use ggez::{Context, GameResult};
use imgui::*;
use nfd::Response;

pub struct SpriteUi;

impl SpriteUi {
    pub fn draw_ui(
        ctx: &mut Context,
        assets: &mut Assets,
        ui: &Ui<'_>,
        sprite: &mut Sprite,
    ) -> GameResult<()> {
        //ui.label_text(im_str!("Name##Frame"), &im_str!("{}", sprite.image.clone()));
        ui.input_vec2_float(im_str!("Offset"), &mut sprite.offset);
        ui.separator();

        ui.input_vec2_float(im_str!("Scale"), &mut sprite.scale);
        ui.separator();

        ui.input_float(im_str!("Rotation"), &mut sprite.rotation)
            .build();
        ui.separator();

        if ui.small_button(im_str!("Load New Image")) {
            let result = nfd::open_file_dialog(Some("png"), None);
            if let Ok(Response::Okay(path)) = result {
                Sprite::load(ctx, assets, sprite, &path)?;
            }
        }
        Ok(())
    }
}
