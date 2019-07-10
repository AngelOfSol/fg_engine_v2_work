use ggez::graphics;
use ggez::graphics::{Color, DrawParam, Mesh};
use ggez::{Context, GameResult};

use crate::animation::{Animation, AnimationUi};

use crate::assets::Assets;
use crate::editor::{EditorState, MessageData, Transition};
use crate::timeline::AtTime;

use crate::imgui_wrapper::ImGuiWrapper;

use crate::typedefs::graphics::{Matrix4, Vec3};

use imgui::*;

use std::path::PathBuf;

pub struct AnimationEditor {
    frame: usize,
    resource: Animation,
    ui_data: AnimationUi,
    done: bool,
}

impl AnimationEditor {
    pub fn new() -> Self {
        Self {
            frame: 0,
            resource: Animation::new("new_animation"),
            ui_data: AnimationUi::new(),
            done: false,
        }
    }
    pub fn with_animation(data: Animation) -> Self {
        Self {
            frame: 0,
            resource: data,
            ui_data: AnimationUi::new(),
            done: false,
        }
    }

    pub fn update(&mut self) -> GameResult<Transition> {
        self.frame = self.frame.wrapping_add(1);

        if self.done {
            let ret = std::mem::replace(&mut self.resource, Animation::new("none"));
            Ok(Transition::Pop(Some(MessageData::Animation(ret))))
        } else {
            Ok(Transition::None)
        }
    }

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        assets: &mut Assets,
        imgui: &mut ImGuiWrapper,
    ) -> GameResult<()> {
        let editor_height = 526.0;
        let dim = [editor_height / 2.0, editor_height / 2.0];
        let [width, height] = dim;
        let pos = [300.0, 20.0];
        let [x, y] = pos;

        let mut editor_result = Ok(());
        imgui
            .frame()
            .run(|ui| {
                // Window
                ui.window(im_str!("Editor"))
                    .size([300.0, editor_height], Condition::Always)
                    .position([0.0, 20.0], Condition::Always)
                    .resizable(false)
                    .movable(false)
                    .collapsible(false)
                    .build(|| {
                        editor_result = self.ui_data.draw_ui(&ui, ctx, assets, &mut self.resource);
                    });

                if self.resource.frames.duration() > 0 {
                    ui.window(im_str!("Animation"))
                        .size(dim, Condition::Always)
                        .position(pos, Condition::Always)
                        .resizable(false)
                        .movable(false)
                        .collapsible(false)
                        .build(|| {});
                    ui.window(im_str!("Current Frame"))
                        .size(dim, Condition::Always)
                        .position([x + width, y], Condition::Always)
                        .resizable(false)
                        .movable(false)
                        .collapsible(false)
                        .build(|| {});

                    ui.window(im_str!("Every Frame"))
                        .size(dim, Condition::Always)
                        .position([x, y + height], Condition::Always)
                        .resizable(false)
                        .movable(false)
                        .collapsible(false)
                        .build(|| {});
                }
                ui.main_menu_bar(|| {
                    ui.menu(im_str!("Animation Editor")).build(|| {
                        if ui.menu_item(im_str!("New")).build() {
                            self.resource = Animation::new("new animation");
                            self.ui_data = AnimationUi::new();
                        }
                        if ui.menu_item(im_str!("Save")).build() {
                            if let Ok(nfd::Response::Okay(path)) =
                                nfd::open_save_dialog(Some("json"), None)
                            {
                                let mut path = PathBuf::from(path);
                                path.set_extension("json");
                                editor_result = Animation::save(ctx, assets, &self.resource, path);
                            }
                        }
                        if ui.menu_item(im_str!("Open")).build() {
                            if let Ok(nfd::Response::Okay(path)) =
                                nfd::open_file_dialog(Some("json"), None)
                            {
                                match Animation::load_from_json(ctx, assets, PathBuf::from(path)) {
                                    Ok(animation) => {
                                        self.resource = animation;
                                        self.ui_data = AnimationUi::new();
                                    }
                                    Err(err) => editor_result = Err(err),
                                }
                            }
                        }
                        ui.separator();
                        if ui.menu_item(im_str!("Back")).build() {
                            self.done = true;
                        }
                    });
                });
            })
            .render(ctx);
        editor_result?;

        let dim = (256.0, 256.0);
        let (width, height) = dim;

        let draw_cross = |ctx: &mut Context, origin: (f32, f32)| {
            let vertical = Mesh::new_line(
                ctx,
                &[[0.0, -10.0], [0.0, 10.0]],
                1.0,
                Color::new(0.0, 1.0, 0.0, 1.0),
            )?;

            let horizontal = Mesh::new_line(
                ctx,
                &[[-10.0, 0.0], [10.0, 0.0]],
                1.0,
                Color::new(0.0, 1.0, 0.0, 1.0),
            )?;
            graphics::draw(
                ctx,
                &vertical,
                DrawParam::default().dest([origin.0, origin.1]),
            )?;
            graphics::draw(
                ctx,
                &horizontal,
                DrawParam::default().dest([origin.0, origin.1]),
            )
        };

        if self.resource.frames.duration() > 0 {
            {
                // normal animation
                let pos = (300.0, 20.0);
                let (x, y) = pos;
                let origin = (x + width / 2.0, y + height / 2.0);

                self.resource.draw_at_time(
                    ctx,
                    assets,
                    self.frame % self.resource.frames.duration(),
                    Matrix4::new_translation(&Vec3::new(origin.0, origin.1, 0.0)),
                )?;
                draw_cross(ctx, origin)?;
            }

            {
                // current_frame
                let pos = (300.0, 20.0 + height);
                let (x, y) = pos;
                let origin = (x + width / 2.0, y + height / 2.0);

                self.resource.draw_every_frame(
                    ctx,
                    assets,
                    Matrix4::new_translation(&Vec3::new(origin.0, origin.1, 0.0)),
                )?;
            }

            if let Some(frame) = self.ui_data.current_sprite {
                // current_frame
                let pos = (300.0 + width, 20.0);
                let (x, y) = pos;
                let origin = (x + width / 2.0, y + height / 2.0);
                self.resource.draw_frame(
                    ctx,
                    assets,
                    frame,
                    Matrix4::new_translation(&Vec3::new(origin.0, origin.1, 0.0)),
                )?;
                draw_cross(ctx, origin)?;
            }
        }
        Ok(())
    }
}

impl Into<EditorState> for AnimationEditor {
    fn into(self) -> EditorState {
        EditorState::Animating(self)
    }
}