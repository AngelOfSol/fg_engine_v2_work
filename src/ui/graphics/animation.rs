use super::sprite::SpriteUi;
use crate::imgui_extra::UiExtensions;
use crate::{assets::Assets, timeline::Timeline};
use crate::{
    graphics::{Animation, BlendMode, Sprite},
    timeline,
};
use ggez::{Context, GameResult};
use imgui::im_str;
use inspect_design::traits::Inspect;
use nfd::Response;

pub struct AnimationUi {
    pub current_sprite: Option<usize>,
    sprite_inspect_state: <Timeline<Sprite> as Inspect>::State,
    inside_state: SpriteUi,
}
impl AnimationUi {
    pub fn new() -> Self {
        Self {
            current_sprite: None,
            sprite_inspect_state: Default::default(),
            inside_state: Default::default(),
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

        let mut result = Ok(());

        let current_sprite = &mut self.current_sprite;
        let inside_state = &mut self.inside_state;

        timeline::inspect::inspect_mut_custom(
            &mut animation.frames,
            "frames",
            &mut self.sprite_inspect_state,
            ui,
            |frame, data| {
                *current_sprite = Some(frame);
                result = inside_state.draw_ui(ctx, assets, ui, data);
            },
        );

        result
    }
}
