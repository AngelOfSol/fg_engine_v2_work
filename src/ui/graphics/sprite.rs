use crate::graphics::Sprite;
use crate::{assets::Assets, graphics::keyframe::Modifiers};
use ggez::{Context, GameResult};
use imgui::*;
use inspect_design::traits::{Inspect, InspectMut};
use nfd::Response;

#[derive(Default)]
pub struct SpriteUi {
    state: <Modifiers as Inspect>::State,
}

impl SpriteUi {
    pub fn draw_ui(
        &mut self,
        ctx: &mut Context,
        assets: &mut Assets,
        ui: &Ui<'_>,
        sprite: &mut Sprite,
    ) -> GameResult<()> {
        sprite
            .modifiers
            .inspect_mut("modifiers", &mut self.state, ui);

        if ui.small_button(im_str!("Load New Image")) {
            let result = nfd::open_file_dialog(Some("png"), None);
            if let Ok(Response::Okay(path)) = result {
                Sprite::load(ctx, assets, sprite, &path)?;
            }
        }
        Ok(())
    }
}
