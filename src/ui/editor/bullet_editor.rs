use super::character_editor::{BulletAnimationResource, BulletResource, ItemResource};
use crate::app_state::{AppContext, AppState, Transition};
use crate::assets::Assets;
use crate::character::components::BulletInfo;
use crate::character::PlayerCharacter;
use crate::game_match::ValueAlpha;
use crate::timeline::AtTime;
use crate::typedefs::graphics::{Matrix4, Vec3};
use crate::ui::character::components::BulletInfoUi;
use crate::ui::editor::AnimationEditor;
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

pub struct BulletInfoEditor {
    frame: usize,
    path: BulletResource,
    resource: Rc<RefCell<BulletInfo>>,
    ui_data: BulletInfoUi,
    done: Status,
    transition: Transition,
    character_data: Rc<RefCell<PlayerCharacter>>,
    assets: Rc<RefCell<Assets>>,
}

impl AppState for BulletInfoEditor {
    fn update(&mut self, ctx: &mut Context, _: &mut AppContext) -> GameResult<Transition> {
        while ggez::timer::check_update_time(ctx, 60) {
            self.frame = self.frame.wrapping_add(1);
        }

        match self.done {
            Status::NotDone => Ok(std::mem::replace(&mut self.transition, Transition::None)),
            Status::DoneAndSave => {
                let mut overwrite_target = self.path.get_from_mut().unwrap();
                *overwrite_target = std::mem::replace(
                    &mut self.resource.borrow_mut(),
                    BulletInfo::new("none".to_owned(), "none".to_owned()),
                );
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

        let attack_ids: Vec<_> = {
            self.character_data
                .borrow()
                .attacks
                .attacks
                .keys()
                .cloned()
                .collect()
        };

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
                            &mut self.resource.borrow_mut(),
                            &attack_ids,
                        );
                        if let Some(()) = &edit_change {
                            self.transition = Transition::Push(Box::new(
                                AnimationEditor::new(
                                    self.assets.clone(),
                                    Box::new(BulletAnimationResource {
                                        data: self.resource.clone(),
                                    }),
                                )
                                .unwrap(),
                            ));
                        }
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
                            *self.resource.borrow_mut() =
                                BulletInfo::new("new bullet".to_owned(), attack_ids[0].clone());
                            self.ui_data = BulletInfoUi::new();
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

        resource
            .hitbox
            .draw(ctx, offset, Color::new(1.0, 1.0, 1.0, 0.15))?;
        if resource.animation.frames.duration() > 0 {
            {
                let _lock = graphics::use_shader(ctx, &self.assets.borrow().shader);

                {
                    resource.animation.draw_at_time(
                        ctx,
                        &self.assets.borrow(),
                        self.frame % resource.animation.frames.duration(),
                        offset,
                        ValueAlpha {
                            alpha: 1.0,
                            value: 1.0,
                        },
                    )?;
                }
            }
        }
        resource
            .hitbox
            .draw(ctx, offset, Color::new(1.0, 0.0, 0.0, 0.35))?;

        graphics::set_transform(ctx, Matrix4::identity());
        graphics::apply_transformations(ctx)?;
        draw_cross(ctx, origin)?;
        graphics::present(ctx)
    }
}

impl BulletInfoEditor {
    pub fn new(
        character_data: Rc<RefCell<PlayerCharacter>>,
        assets: Rc<RefCell<Assets>>,
        path: BulletResource,
    ) -> Option<Self> {
        let resource = Rc::new(RefCell::new(path.get_from()?.clone()));
        Some(Self {
            character_data,
            assets,
            path,
            frame: 0,
            resource,
            ui_data: BulletInfoUi::new(),
            done: Status::NotDone,
            transition: Transition::None,
        })
    }
}
