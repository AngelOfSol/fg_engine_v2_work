use super::character_editor::{ItemResource, ParticleAnimationResource, ParticleResource};
use crate::app_state::{AppContext, AppState, Transition};
use crate::assets::Assets;
use crate::character::PlayerCharacter;
use crate::graphics::particle::Particle;
use crate::typedefs::graphics::{Matrix4, Vec3};
use crate::ui::editor::AnimationEditor;
use crate::ui::graphics::modifiers::ModifiersUi;
use crate::ui::graphics::particle::ParticleUi;
use ggez::graphics;
use ggez::graphics::{Color, DrawParam, Mesh};
use ggez::{Context, GameResult};
use imgui::*;
use std::cell::RefCell;
use std::rc::Rc;

enum Status {
    DoneAndSave,
    DoneAndQuit,
    NotDone,
}

pub struct ParticleEditor {
    frame: usize,
    path: ParticleResource,
    resource: Rc<RefCell<Particle>>,
    ui_data: ParticleUi,
    done: Status,
    transition: Transition,
    assets: Rc<RefCell<Assets>>,
}

impl AppState for ParticleEditor {
    fn update(&mut self, ctx: &mut Context, _: &mut AppContext) -> GameResult<Transition> {
        while ggez::timer::check_update_time(ctx, 60) {
            self.frame = self.frame.wrapping_add(1);
        }

        match self.done {
            Status::NotDone => Ok(std::mem::replace(&mut self.transition, Transition::None)),
            Status::DoneAndSave => {
                let mut overwrite_target = self.path.get_from_mut().unwrap();
                *overwrite_target =
                    std::mem::replace(&mut self.resource.borrow_mut(), Particle::new());
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
        let pos = [300.0, 20.0];
        imgui
            .frame()
            .run(|ui| {
                imgui::Window::new(im_str!("Editor"))
                    .size([300.0, editor_height], Condition::Once)
                    .position([0.0, 20.0], Condition::Once)
                    .build(ui, || {
                        let edit_change = self.ui_data.draw_ui(
                            ctx,
                            &mut self.assets.borrow_mut(),
                            &ui,
                            &mut self.resource.borrow_mut().animations,
                        );
                        if let Some(name) = &edit_change {
                            self.transition = Transition::Push(Box::new(
                                AnimationEditor::new(
                                    self.assets.clone(),
                                    Box::new(ParticleAnimationResource {
                                        animation: name.clone(),
                                        data: self.resource.clone(),
                                    }),
                                )
                                .unwrap(),
                            ));
                        }
                    });
                imgui::Window::new(im_str!("Modifiers"))
                    .size([300.0, editor_height], Condition::Once)
                    .position([600.0, 20.0], Condition::Once)
                    .build(ui, || {
                        ModifiersUi::draw_ui(ui, &mut self.resource.borrow_mut().modifiers);
                    });

                imgui::Window::new(im_str!("Animation"))
                    .size(dim, Condition::Always)
                    .position(pos, Condition::Always)
                    .resizable(false)
                    .movable(false)
                    .collapsible(false)
                    .build(ui, || {});

                ui.main_menu_bar(|| {
                    ui.menu(im_str!("Bullet Info Editor"), true, || {
                        if imgui::MenuItem::new(im_str!("Reset")).build(ui) {
                            *self.resource.borrow_mut() = Particle::new();
                            self.ui_data = ParticleUi::new();
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
        let pos = (300.0, 20.0);
        let (x, y) = pos;
        let origin = (x + width / 2.0, y + height / 2.0);
        let offset = Matrix4::new_translation(&Vec3::new(origin.0, origin.1, 0.0));

        let resource = self.resource.borrow();

        resource.draw_at_time_debug(
            ctx,
            &self.assets.borrow(),
            self.frame % resource.duration(),
            offset,
        )?;

        graphics::set_transform(ctx, Matrix4::identity());
        graphics::apply_transformations(ctx)?;
        draw_cross(ctx, origin)?;
        graphics::present(ctx)
    }
}

impl ParticleEditor {
    pub fn new(assets: Rc<RefCell<Assets>>, path: ParticleResource) -> Option<Self> {
        let resource = Rc::new(RefCell::new(path.get_from()?.clone()));
        Some(Self {
            assets,
            path,
            frame: 0,
            resource,
            ui_data: ParticleUi::new(),
            done: Status::NotDone,
            transition: Transition::None,
        })
    }
}