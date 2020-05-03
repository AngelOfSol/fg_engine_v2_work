use super::character_editor::ItemResource;
use crate::app_state::{AppContext, AppState, Transition};
use crate::assets::Assets;
use crate::game_match::ValueAlpha;
use crate::graphics::Animation;
use crate::timeline::AtTime;
use crate::typedefs::graphics::{Matrix4, Vec3};
use crate::ui::graphics::animation::AnimationUi;
use ggez::graphics;
use ggez::graphics::{Color, DrawParam, Mesh};
use ggez::{Context, GameResult};
use imgui::*;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

enum Status {
    DoneAndSave,
    DoneAndQuit,
    NotDone,
}

pub struct AnimationEditor {
    frame: usize,
    assets: Rc<RefCell<Assets>>,
    path: Box<dyn ItemResource<Output = Animation>>,
    resource: Animation,
    ui_data: AnimationUi,
    done: Status,
}

impl AppState for AnimationEditor {
    fn update(&mut self, ctx: &mut Context, _: &mut AppContext) -> GameResult<Transition> {
        while ggez::timer::check_update_time(ctx, 60) {
            self.frame = self.frame.wrapping_add(1);
        }

        match std::mem::replace(&mut self.done, Status::NotDone) {
            Status::NotDone => Ok(Transition::None),
            Status::DoneAndSave => {
                let mut overwrite_target = self.path.get_from_mut().unwrap();
                *overwrite_target = std::mem::replace(&mut self.resource, Animation::new("empty"));
                Ok(Transition::Pop)
            }
            Status::DoneAndQuit => Ok(Transition::Pop),
        }
    }
    fn on_enter(&mut self, _: &mut Context, _: &mut AppContext) -> GameResult<()> {
        Ok(())
    }
    fn draw(
        &mut self,
        ctx: &mut Context,
        AppContext { ref mut imgui, .. }: &mut AppContext,
    ) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        let editor_height = 526.0;
        let dim = [editor_height / 2.0, editor_height / 2.0];
        let [width, height] = dim;
        let pos = [300.0, 20.0];
        let [x, y] = pos;
        let mut editor_result = Ok(());
        imgui
            .frame()
            .run(|ui| {
                imgui::Window::new(im_str!("Editor"))
                    .size([300.0, editor_height], Condition::Once)
                    .position([0.0, 20.0], Condition::Once)
                    .build(ui, || {
                        let assets = &mut self.assets.borrow_mut();
                        editor_result = self.ui_data.draw_ui(&ui, ctx, assets, &mut self.resource);
                    });

                if self.resource.frames.duration() > 0 {
                    imgui::Window::new(im_str!("Animation"))
                        .size(dim, Condition::Always)
                        .position(pos, Condition::Always)
                        .resizable(false)
                        .movable(false)
                        .collapsible(false)
                        .build(ui, || {});
                    imgui::Window::new(im_str!("Current Frame"))
                        .size(dim, Condition::Always)
                        .position([x + width, y], Condition::Always)
                        .resizable(false)
                        .movable(false)
                        .collapsible(false)
                        .build(ui, || {});

                    imgui::Window::new(im_str!("Every Frame"))
                        .size(dim, Condition::Always)
                        .position([x, y + height], Condition::Always)
                        .resizable(false)
                        .movable(false)
                        .collapsible(false)
                        .build(ui, || {});
                }
                ui.main_menu_bar(|| {
                    ui.menu(im_str!("Animation Editor"), true, || {
                        if imgui::MenuItem::new(im_str!("Reset")).build(ui) {
                            self.resource = Animation::new("new animation");
                            self.ui_data = AnimationUi::new();
                        }
                        if imgui::MenuItem::new(im_str!("Save to file")).build(ui) {
                            if let Ok(nfd::Response::Okay(path)) =
                                nfd::open_save_dialog(Some("json"), None)
                            {
                                let mut path = PathBuf::from(path);
                                path.set_extension("json");
                                let assets = &mut self.assets.borrow_mut();
                                editor_result = Animation::save(ctx, assets, &self.resource, path);
                            }
                        }
                        if imgui::MenuItem::new(im_str!("Load from file")).build(ui) {
                            if let Ok(nfd::Response::Okay(path)) =
                                nfd::open_file_dialog(Some("json"), None)
                            {
                                let assets = &mut self.assets.borrow_mut();
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
                        if imgui::MenuItem::new(im_str!("Save and back")).build(ui) {
                            self.done = Status::DoneAndSave;
                        }
                        if imgui::MenuItem::new(im_str!("Back without save")).build(ui) {
                            self.done = Status::DoneAndQuit;
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
        let assets = &mut self.assets.borrow_mut();

        if self.resource.frames.duration() > 0 {
            {
                // normal animation
                let pos = (300.0, 20.0);
                let (x, y) = pos;
                let origin = (x + width / 2.0, y + height / 2.0);

                {
                    let _lock = graphics::set_shader(ctx, &assets.shader);

                    self.resource.draw_at_time(
                        ctx,
                        assets,
                        self.frame % self.resource.frames.duration(),
                        Matrix4::new_translation(&Vec3::new(origin.0, origin.1, 0.0)),
                        ValueAlpha {
                            alpha: 1.0,
                            value: 1.0,
                        },
                    )?;
                }
                draw_cross(ctx, origin)?;
            }

            {
                // current_frame
                let pos = (300.0, 20.0 + height);
                let (x, y) = pos;
                let origin = (x + width / 2.0, y + height / 2.0);

                {
                    let _lock = graphics::set_shader(ctx, &assets.shader);

                    self.resource.draw_every_frame(
                        ctx,
                        assets,
                        Matrix4::new_translation(&Vec3::new(origin.0, origin.1, 0.0)),
                    )?;
                }
            }

            if let Some(frame) = self.ui_data.current_sprite {
                // current_frame
                let pos = (300.0 + width, 20.0);
                let (x, y) = pos;
                let origin = (x + width / 2.0, y + height / 2.0);
                {
                    let _lock = graphics::set_shader(ctx, &assets.shader);

                    self.resource.draw_frame(
                        ctx,
                        assets,
                        frame,
                        Matrix4::new_translation(&Vec3::new(origin.0, origin.1, 0.0)),
                    )?;
                }
                draw_cross(ctx, origin)?;
            }
        }
        graphics::present(ctx)
    }
}

impl AnimationEditor {
    pub fn new(
        assets: Rc<RefCell<Assets>>,
        path: Box<dyn ItemResource<Output = Animation>>,
    ) -> Option<Self> {
        let resource = path.get_from()?.clone();
        Some(Self {
            frame: 0,
            assets,
            resource,
            path,
            ui_data: AnimationUi::new(),
            done: Status::NotDone,
        })
    }
}
