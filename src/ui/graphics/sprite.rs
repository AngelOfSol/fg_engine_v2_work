use super::modifiers::ModifiersUi;
use crate::assets::Assets;
use crate::graphics::Sprite;
use ggez::{Context, GameResult};
use imgui::*;
use nfd::Response;

#[derive(Default)]
pub struct SpriteUi {
    state: ModifiersUi,
}

impl SpriteUi {
    pub fn draw_ui(
        &mut self,
        ctx: &mut Context,
        assets: &mut Assets,
        ui: &Ui<'_>,
        sprite: &mut Sprite,
    ) -> GameResult<()> {
        self.state.draw_ui(ui, &mut sprite.modifiers);

        if ui.small_button(im_str!("Load New Image")) {
            let result = nfd::open_file_dialog(Some("png"), None);
            if let Ok(Response::Okay(path)) = result {
                Sprite::load(ctx, assets, sprite, &path)?;
            }
        }
        Ok(())
    }
}
