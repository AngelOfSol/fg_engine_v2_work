use super::{
    typed_animation_editor::TypedAnimationEditor, typed_character_editor::EDITOR_BACKGROUND,
};
use crate::{
    app_state::{AppContext, AppState, Transition},
    ui::graphics::animations::AnimationsUi,
};
use crate::{assets::Assets, graphics::keyframe::Modifiers};
use crate::{graphics::animation_group::AnimationGroup, roster::character::typedefs::Character};
use crate::{
    roster::character::data::Data,
    typedefs::graphics::{Matrix4, Vec3},
};
use ggez::graphics;
use ggez::graphics::{Color, DrawParam, Mesh};
use ggez::{Context, GameResult};
use imgui::*;
use inspect_design::traits::*;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

enum Status {
    DoneAndSave,
    DoneAndQuit,
    NotDone,
}

pub struct TypedAnimationGroupEditor<C: Character> {
    frame: usize,
    path: C::Graphic,
    resource: AnimationGroup,
    character_data: Rc<RefCell<Data<C>>>,
    ui_data: AnimationsUi,
    done: Status,
    transition: Transition,
    assets: Rc<RefCell<Assets>>,
    modifiers_state: <Modifiers as Inspect>::State,
}

impl<C: Character> AppState for TypedAnimationGroupEditor<C> {
    fn update(&mut self, ctx: &mut Context, _: &mut AppContext) -> GameResult<Transition> {
        while ggez::timer::check_update_time(ctx, 60) {
            self.frame = self.frame.wrapping_add(1);
            self.resource.fix_durations();
        }

        match self.done {
            Status::NotDone => Ok(std::mem::replace(&mut self.transition, Transition::None)),
            Status::DoneAndSave => {
                self.resource.fix_durations();
                self.character_data
                    .borrow_mut()
                    .graphics
                    .insert(self.path, self.resource.clone());
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
        graphics::clear(ctx, EDITOR_BACKGROUND.into());

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
                        imgui::TabBar::new(im_str!("Main")).build(ui, || {
                            imgui::TabItem::new(im_str!("Animations")).build(ui, || {
                                let edit_change = self.ui_data.draw_ui(
                                    ctx,
                                    &mut self.assets.borrow_mut(),
                                    &ui,
                                    &mut self.resource.animations,
                                );
                                if let Some(name) = edit_change {
                                    self.transition =
                                        Transition::Push(Box::new(TypedAnimationEditor::new(
                                            self.assets.clone(),
                                            self.character_data.clone(),
                                            self.path,
                                            self.resource
                                                .animations
                                                .iter()
                                                .position(|item| item.name == name)
                                                .unwrap(),
                                        )));
                                }
                            });
                            imgui::TabItem::new(im_str!("Modifiers")).build(ui, || {
                                self.resource.modifiers.inspect_mut(
                                    "modifiers",
                                    &mut self.modifiers_state,
                                    ui,
                                );
                            });
                        });
                    });

                ui.main_menu_bar(|| {
                    ui.menu(im_str!("Particle Info Editor"), true, || {
                        if imgui::MenuItem::new(im_str!("New")).build(ui) {
                            self.resource = AnimationGroup::new();
                            self.ui_data = AnimationsUi::new();
                        }
                        ui.separator();
                        if imgui::MenuItem::new(im_str!("Save to file")).build(ui) {
                            if let Ok(nfd::Response::Okay(path)) =
                                nfd::open_save_dialog(Some("json"), None)
                            {
                                let mut path = PathBuf::from(path);
                                path.set_extension("json");
                                let assets = &mut self.assets.borrow_mut();
                                // TODO: use this error
                                let _ = AnimationGroup::save(ctx, assets, &self.resource, path);
                            }
                        }
                        if imgui::MenuItem::new(im_str!("Load from file")).build(ui) {
                            if let Ok(nfd::Response::Okay(path)) =
                                nfd::open_file_dialog(Some("json"), None)
                            {
                                let assets = &mut self.assets.borrow_mut();
                                let animation_group = AnimationGroup::load_from_json(
                                    ctx,
                                    assets,
                                    PathBuf::from(path),
                                );
                                if let Ok(animation_group) = animation_group {
                                    self.resource = animation_group;
                                    self.ui_data = AnimationsUi::new();
                                } else if let Err(err) = animation_group {
                                    dbg!(err);
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

        // normal bullet
        let pos = (0.0, 20.0);
        let (x, y) = pos;
        let origin = (x + width / 2.0, y + height / 2.0);
        let offset = Matrix4::new_translation(&Vec3::new(origin.0, origin.1, 0.0));

        if self.resource.duration() > 0 {
            let _lock = graphics::use_shader(ctx, &self.assets.borrow().shader);

            self.resource.draw_at_time(
                ctx,
                &self.assets.borrow(),
                self.frame % self.resource.duration(),
                offset,
            )?;
        }

        graphics::set_transform(ctx, Matrix4::identity());
        graphics::apply_transformations(ctx)?;
        draw_cross(ctx, origin)?;
        graphics::present(ctx)
    }
}

impl<C: Character> TypedAnimationGroupEditor<C> {
    pub fn new(
        assets: Rc<RefCell<Assets>>,
        character_data: Rc<RefCell<Data<C>>>,
        path: C::Graphic,
    ) -> Self {
        let resource = character_data.borrow().graphics[&path].clone();
        Self {
            assets,
            path,
            frame: 0,
            resource,
            character_data,
            ui_data: AnimationsUi::new(),
            done: Status::NotDone,
            transition: Transition::None,
            modifiers_state: Default::default(),
        }
    }
}
