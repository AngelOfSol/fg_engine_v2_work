use super::sprite::SpriteUi;
use crate::assets::Assets;
use crate::graphics::{Animation, BlendMode, Sprite};
use crate::imgui_extra::UiExtensions;
use ggez::{Context, GameResult};
use imgui::im_str;
use nfd::Response;

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

        let mut counter = 0;
        // TODO(TL_UI)
        // ui.rearrangable_list_box(
        //     im_str!("Frame List"),
        //     &mut self.current_sprite,
        //     &mut animation.frames,
        //     |_| {
        //         let ret = counter;
        //         counter += 1;
        //         im_str!("Frame {}", ret)
        //     },
        //     5,
        // );
        if ui.small_button(im_str!("From Files")) {
            let result = nfd::open_file_multiple_dialog(Some("png"), None);
            if let Ok(response) = result {
                match response {
                    Response::Cancel => (),
                    Response::Okay(path) => {
                        animation.frames.insert_force(
                            animation.frames.duration(),
                            Sprite::load_new(ctx, assets, path)?,
                        );
                    }
                    Response::OkayMultiple(paths) => {
                        for path in paths {
                            animation.frames.insert_force(
                                animation.frames.duration(),
                                Sprite::load_new(ctx, assets, path)?,
                            );
                        }
                    }
                }
            }
        }
        if let Some(current_sprite) = self.current_sprite {
            // TODO(TL_UI)
            // ui.same_line(0.0);
            // if ui.small_button(im_str!("Delete")) {
            //     animation.frames.remove_frame(current_sprite);
            //     if animation.frames.is_empty() {
            //         self.current_sprite = None;
            //     } else {
            //         self.current_sprite =
            //             Some(std::cmp::min(animation.frames.len() - 1, current_sprite));
            //     }
            // }
        }
        ui.same_line(0.0);
        if ui.small_button(im_str!("Delete All")) {
            // TODO(TL_UI)
            //animation.frames.clear();
            self.current_sprite = None;
        }

        if let Some(current_sprite) = self.current_sprite {
            // TODO(TL_UI)
            // let (ref mut sprite, ref mut duration) = animation.frames[current_sprite];
            // let _ = ui.input_whole(im_str!("Duration"), duration);
            // *duration = std::cmp::max(1, *duration);
            // ui.separator();

            // SpriteUi::draw_ui(ctx, assets, ui, sprite)?;
        }

        Ok(())
    }
}
