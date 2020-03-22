use super::modifiers::ModifiersUi;
use crate::assets::Assets;
use crate::graphics::Sprite;
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
        ModifiersUi::draw_ui(ui, &mut sprite.modifiers);

        if ui.small_button(im_str!("Load New Image")) {
            let result = nfd::open_file_dialog(Some("png"), None);
            if let Ok(Response::Okay(path)) = result {
                Sprite::load(ctx, assets, sprite, &path)?;
            }
        }
        Ok(())
    }
}
