use crate::graphics::Animation;
use crate::typedefs::graphics::{Matrix4, Vec3};
use crate::ui::graphics::animation::AnimationUi;
use crate::{
    app_state::{AppContext, AppState, Transition},
    roster::character::typedefs::Character,
};
use crate::{
    assets::{Assets, ValueAlpha},
    roster::character::data::Data,
};
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

pub struct TypedAnimationEditor<C: Character> {
    frame: usize,
    assets: Rc<RefCell<Assets>>,
    path: C::Graphic,
    character_data: Rc<RefCell<Data<C>>>,
    index: usize,
    resource: Animation,
    ui_data: AnimationUi,
    done: Status,
}

impl<C: Character> AppState for TypedAnimationEditor<C> {
    fn update(&mut self, ctx: &mut Context, _: &mut AppContext) -> GameResult<Transition> {
        while ggez::timer::check_update_time(ctx, 60) {
            self.frame = self.frame.wrapping_add(1);
            self.resource
                .modifiers
                .set_duration(self.resource.frames.duration());

            let durations = self.resource.frames.frame_durations().collect::<Vec<_>>();

            for (frame, duration) in self.resource.frames.frames_mut().zip(durations) {
                frame.modifiers.set_duration(duration)
            }
        }

        match std::mem::replace(&mut self.done, Status::NotDone) {
            Status::NotDone => Ok(Transition::None),
            Status::DoneAndSave => {
                self.character_data
                    .borrow_mut()
                    .graphics
                    .get_mut(&self.path)
                    .unwrap()
                    .animations[self.index] = self.resource.clone();
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
        let mut editor_result = Ok(());
        imgui
            .frame()
            .run(|ui| {
                imgui::Window::new(im_str!("Editor"))
                    .size([1620.0, 1060.0], Condition::Always)
                    .position([300.0, 20.0], Condition::Always)
                    .draw_background(true)
                    .movable(false)
                    .resizable(false)
                    .collapsible(false)
                    .title_bar(false)
                    .build(ui, || {
                        let assets = &mut self.assets.borrow_mut();
                        editor_result = self.ui_data.draw_ui(&ui, ctx, assets, &mut self.resource);
                    });

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

        let height = 256.0;

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

        let width = 300.0;

        if self.resource.frames.duration() > 0 {
            {
                // normal animation
                let pos = (0.0, 20.0);
                let (x, y) = pos;
                let origin = (x + width / 2.0, y + height / 2.0);

                {
                    let _lock = graphics::use_shader(ctx, &assets.shader);

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

            if let Some(frame) = self.ui_data.current_sprite {
                // current_frame
                let pos = (0.0, height + 20.0);
                let (x, y) = pos;
                let origin = (x + width / 2.0, y + height / 2.0);
                {
                    let _lock = graphics::use_shader(ctx, &assets.shader);

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

impl<C: Character> TypedAnimationEditor<C> {
    pub fn new(
        assets: Rc<RefCell<Assets>>,
        character_data: Rc<RefCell<Data<C>>>,
        path: C::Graphic,
        index: usize,
    ) -> Self {
        let resource = character_data.borrow().graphics[&path].animations[index].clone();
        Self {
            frame: 0,
            assets,
            resource,
            index,
            character_data,
            path,
            ui_data: AnimationUi::new(),
            done: Status::NotDone,
        }
    }
}
