use super::super::sprite::{Sprite, SpriteUi};

use super::{Animation, BlendMode};

use crate::assets::Assets;
use crate::imgui_extra::UiExtensions;

use imgui::im_str;

use ggez::{Context, GameResult};

use nfd::Response;

use crate::timeline::AtTime;

pub struct AnimationUi {
    pub current_sprite: Option<usize>,
}

impl AnimationUi {
    pub fn new() -> Self {
        Self {
            current_sprite: None,
        }
    }

    pub fn draw_ui(
        &mut self,
        ui: &imgui::Ui,
        ctx: &mut Context,
        assets: &mut Assets,
        animation: &mut Animation,
    ) -> GameResult<()> {
        ui.input_string(im_str!("Name"), &mut animation.name);

        ui.label_text(
            im_str!("Duration"),
            &im_str!("{}", animation.frames.duration()),
        );
        ui.separator();

        ui.text(im_str!("Blend Mode"));
        ui.radio_button(
            im_str!("Alpha"),
            &mut animation.blend_mode,
            BlendMode::Alpha,
        );
        ui.same_line(0.0);
        ui.radio_button(im_str!("Add"), &mut animation.blend_mode, BlendMode::Add);
        ui.separator();

        ui.rearrangable_list_box(
            im_str!("Frame List"),
            &mut self.current_sprite,
            &mut animation.frames,
            |item| im_str!("{}", item.0.image.clone()),
            5,
        );

        if ui.small_button(im_str!("Normalize All Names")) {
            for (idx, ref mut sprite) in animation
                .frames
                .iter_mut()
                .map(|item| &mut item.0)
                .enumerate()
            {
                sprite.rename(format!("{}-{:03}.png", animation.name, idx), assets)
            }
        }
        if ui.small_button(im_str!("From Files")) {
            let result = nfd::open_file_multiple_dialog(Some("png"), None);
            if let Ok(response) = result {
                match response {
                    Response::Cancel => (),
                    Response::Okay(path) => {
                        animation.frames.push((Sprite::new(path), 1));
                        animation.load_images(ctx, assets)?;
                    }
                    Response::OkayMultiple(paths) => {
                        for path in paths {
                            animation.frames.push((Sprite::new(path), 1));
                        }
                        animation.load_images(ctx, assets)?;
                    }
                }
            }
        }
        if let Some(current_sprite) = self.current_sprite {
            ui.same_line(0.0);
            if ui.small_button(im_str!("Delete")) {
                animation.frames.remove(current_sprite);
                if animation.frames.is_empty() {
                    self.current_sprite = None;
                } else {
                    self.current_sprite =
                        Some(std::cmp::min(animation.frames.len() - 1, current_sprite));
                }
            }
        }
        ui.same_line(0.0);
        if ui.small_button(im_str!("Delete All")) {
            animation.frames.clear();
            self.current_sprite = None;
        }

        if let Some(current_sprite) = self.current_sprite {
            let (ref mut sprite, ref mut duration) = animation.frames[current_sprite];
            let _ = ui.input_whole(im_str!("Duration"), duration);
            *duration = std::cmp::max(1, *duration);
            ui.separator();

            SpriteUi::draw_ui(ctx, assets, ui, sprite)?;
        }

        Ok(())
    }
}
