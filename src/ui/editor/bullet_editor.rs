use crate::assets::Assets;
use crate::character::components::BulletInfo;
use crate::graphics::Animation;
use crate::imgui_wrapper::ImGuiWrapper;
use crate::timeline::AtTime;
use crate::typedefs::graphics::{Matrix4, Vec3};
use crate::ui::character::components::BulletInfoUi;
use crate::ui::editor::{AnimationEditor, EditorState, MessageData, Mode, Transition};
use ggez::graphics;
use ggez::graphics::{Color, DrawParam, Mesh};
use ggez::{Context, GameResult};
use imgui::*;

enum Status {
    DoneAndSave,
    DoneAndQuit,
    NotDone,
}

pub struct BulletInfoEditor {
    frame: usize,
    resource: BulletInfo,
    ui_data: BulletInfoUi,
    done: Status,
    transition: Transition,
    attack_ids: Vec<String>,
}

impl BulletInfoEditor {
    pub fn with_bullet(data: BulletInfo, attack_ids: Vec<String>) -> Self {
        Self {
            frame: 0,
            resource: data,
            ui_data: BulletInfoUi::new(),
            done: Status::NotDone,
            transition: Transition::None,
            attack_ids,
        }
    }

    pub fn handle_message(&mut self, data: MessageData, mode: Mode) {
        if let MessageData::Animation(animation) = data {
            match mode {
                Mode::Standalone => (),
                Mode::New => {
                    self.resource.animation = animation;
                }
                Mode::Edit(_) => {
                    self.resource.animation = animation;
                }
            }
        }
    }
    pub fn update(&mut self) -> GameResult<Transition> {
        self.frame = self.frame.wrapping_add(1);

        match self.done {
            Status::NotDone => Ok(std::mem::replace(&mut self.transition, Transition::None)),
            Status::DoneAndSave => {
                let ret = std::mem::replace(
                    &mut self.resource,
                    BulletInfo::new("none".to_owned(), "none".to_owned()),
                );
                Ok(Transition::Pop(Some(MessageData::BulletInfo(ret))))
            }
            Status::DoneAndQuit => Ok(Transition::Pop(None)),
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
                            assets,
                            &ui,
                            &mut self.resource,
                            &self.attack_ids,
                        );
                        if let Some(mode) = &edit_change {
                            let animation = match &mode {
                                Mode::Edit(_) => self.resource.animation.clone(),
                                _ => Animation::new("new"),
                            };
                            self.transition = Transition::Push(
                                Box::new(AnimationEditor::with_animation(animation).into()),
                                mode.clone(),
                            );
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
                            self.resource = BulletInfo::new(
                                "new bullet".to_owned(),
                                self.attack_ids[0].clone(),
                            );
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

        self.resource
            .hitbox
            .draw(ctx, offset, Color::new(1.0, 1.0, 1.0, 0.15))?;
        if self.resource.animation.frames.duration() > 0 {
            {
                self.resource.animation.draw_at_time(
                    ctx,
                    assets,
                    self.frame % self.resource.animation.frames.duration(),
                    offset,
                )?;
            }
        }
        self.resource
            .hitbox
            .draw(ctx, offset, Color::new(1.0, 0.0, 0.0, 0.35))?;

        graphics::set_transform(ctx, Matrix4::identity());
        graphics::apply_transformations(ctx)?;
        draw_cross(ctx, origin)?;
        Ok(())
    }
}

impl Into<EditorState> for BulletInfoEditor {
    fn into(self) -> EditorState {
        EditorState::BulletInfoEditor(self)
    }
}
