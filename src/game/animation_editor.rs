
use ggez::graphics;
use ggez::graphics::Color;
use ggez::{Context, GameResult};

use crate::animation::{Animation, AnimationUi};

use crate::assets::Assets;
use crate::imgui_wrapper::ImGuiWrapper;
use crate::timeline::AtTime;
use crate::game;

use imgui::*;

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

    pub fn update(&mut self) -> GameResult<game::Transition> {
        self.frame += 1;
        if self.done {
            
        Ok(game::Transition::Pop)
        } else {
            
        Ok(game::Transition::None)
        }
    }

    pub fn draw<'a>(
        &mut self,
        ctx: &mut Context,
        assets: &mut Assets,
        imgui: &'a mut ImGuiWrapper,
    ) -> GameResult<()> {
        let editor_height = 526.0;
        let dim = [editor_height / 2.0, editor_height / 2.0];
        let [width, height] = dim;
        let pos = [300.0, 20.0];
        let [x, y] = pos;

        let mut editor_result = Ok(());
        let imgui_render = imgui.frame().run(|ui| {
            // Window
            ui.window(im_str!("Editor"))
                .size([300.0, editor_height], Condition::Always)
                .position([0.0, 20.0], Condition::Always)
                .resizable(false)
                .movable(false)
                .collapsible(false)
                .build(|| {
                    editor_result = self.resource.draw_ui(&ui, ctx, assets, &mut self.ui_data);
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
                ui.menu(im_str!("File")).build(|| {
                    if ui.menu_item(im_str!("New")).build() {
                        self.resource = Animation::new("new animation");
                        self.ui_data = AnimationUi::new();
                    }
                    if ui.menu_item(im_str!("Save")).build() {
                        let path_result = nfd::open_save_dialog(Some("tar"), None);
                        match path_result {
                            Ok(path) => match path {
                                nfd::Response::Cancel => (),
                                nfd::Response::Okay(path) => {
                                    editor_result = self.resource.save_tar(ctx, assets, &path);
                                }
                                nfd::Response::OkayMultiple(_) => (),
                            },
                            Err(err) => {
                                dbg!(err);
                            }
                        }
                    }
                    if ui.menu_item(im_str!("Open")).build() {
                        let path_result =
                            nfd::open_dialog(Some("tar"), None, nfd::DialogType::SingleFile);
                        match path_result {
                            Ok(path) => match path {
                                nfd::Response::Cancel => (),
                                nfd::Response::Okay(path) => {
                                    self.resource =
                                        Animation::load_tar(ctx, assets, &path).unwrap();
                                }
                                nfd::Response::OkayMultiple(_) => (),
                            },
                            Err(err) => {
                                dbg!(err);
                            }
                        }
                    }
                    ui.separator();
                    if ui.menu_item(im_str!("Back")).build() {
                        self.done = true;
                    }
                });
            });
        });
        imgui_render.render(ctx);
        editor_result?;


        let vertical = graphics::Mesh::new_line(
            ctx,
            &[[0.0, -10.0], [0.0, 10.0]],
            1.0,
            Color::new(0.0, 1.0, 0.0, 1.0),
        )?;

        let horizontal = graphics::Mesh::new_line(
            ctx,
            &[[-10.0, 0.0], [10.0, 0.0]],
            1.0,
            Color::new(0.0, 1.0, 0.0, 1.0),
        )?;


        let dim = (256.0, 256.0);
        let (width, height) = dim;

        let padding = 15.0;

        let draw_cross = |ctx: &mut Context, origin: (f32, f32)| {
            graphics::draw(
                ctx,
                &vertical,
                graphics::DrawParam::default().dest([origin.0, origin.1]),
            )?;
            graphics::draw(
                ctx,
                &horizontal,
                graphics::DrawParam::default().dest([origin.0, origin.1]),
            )
        };

        if self.resource.frames.duration() > 0 {
            {
                // normal animation
                let pos = (300.0, 20.0);
                let (x, y) = pos;
                let origin = (x + width / 2.0, y + height - padding);

                Animation::draw_at_time(
                    ctx,
                    assets,
                    &self.resource,
                    self.frame % self.resource.frames.duration(),
                    nalgebra::Translation2::new(origin.0, origin.1).to_homogeneous(),
                )?;
                draw_cross(ctx, origin)?;
            }

            {
                // current_frame
                let pos = (300.0, 20.0 + height);
                let (x, y) = pos;
                let origin = (x + width / 2.0, y + height - padding);

                Animation::draw_every_frame(
                    ctx,
                    assets,
                    &self.resource,
                    nalgebra::Translation2::new(origin.0, origin.1).to_homogeneous(),
                )?;
            }

            if let Some(frame) = self.ui_data.current_sprite {
                {
                    // current_frame
                    let pos = (300.0 + width, 20.0);
                    let (x, y) = pos;
                    let origin = (x + width / 2.0, y + height - padding);
                    Animation::draw_frame(
                        ctx,
                        assets,
                        &self.resource,
                        frame,
                        nalgebra::Translation2::new(origin.0, origin.1).to_homogeneous(),
                    )?;
                    draw_cross(ctx, origin)?;
                }
                {
                    // current_frame
                    let pos = (300.0, 20.0 + height);
                    let (x, y) = pos;
                    let origin = (x + width / 2.0, y + height - padding);

                    Animation::draw_frame(
                        ctx,
                        assets,
                        &self.resource,
                        frame,
                        nalgebra::Translation2::new(origin.0, origin.1).to_homogeneous(),
                    )?;
                    draw_cross(ctx, origin)?;
                }
            }
        }
        Ok(())
    }
}


impl Into<game::GameState> for AnimationEditor {
    fn into(self) -> game::GameState {
        game::GameState::Animating(self)
    }
}