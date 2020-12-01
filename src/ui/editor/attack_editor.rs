use super::character_editor::{AttackResource, ItemResource};
use crate::app_state::{AppContext, AppState, Transition};
use crate::character::components::AttackInfo;
use crate::ui::character::components::AttackInfoUi;
use ggez::{graphics, Context, GameResult};
use imgui::*;

enum Status {
    DoneAndSave,
    DoneAndQuit,
    NotDone,
}

pub struct AttackInfoEditor {
    path: AttackResource,
    frame: usize,
    resource: AttackInfo,
    ui_data: AttackInfoUi,
    done: Status,
    transition: Transition,
}
impl AppState for AttackInfoEditor {
    fn update(&mut self, ctx: &mut Context, _: &mut AppContext) -> GameResult<Transition> {
        while ggez::timer::check_update_time(ctx, 60) {
            self.frame = self.frame.wrapping_add(1);
        }

        match self.done {
            Status::NotDone => Ok(std::mem::replace(&mut self.transition, Transition::None)),
            Status::DoneAndSave => {
                let mut overwrite_target = self.path.get_from_mut().unwrap();
                *overwrite_target = std::mem::take(&mut self.resource);
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
        imgui
            .frame()
            .run(|ui| {
                imgui::Window::new(im_str!("Editor"))
                    .size([300.0, editor_height], Condition::Once)
                    .position([0.0, 20.0], Condition::Once)
                    .build(ui, || {
                        self.ui_data.draw_ui(&ui, &mut self.resource);
                    });

                ui.main_menu_bar(|| {
                    ui.menu(im_str!("Attack Info Editor"), true, || {
                        if imgui::MenuItem::new(im_str!("Reset")).build(ui) {
                            self.resource = AttackInfo::default();
                            self.ui_data = AttackInfoUi::new();
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

        graphics::present(ctx)
    }
}

impl AttackInfoEditor {
    pub fn new(path: AttackResource) -> Option<Self> {
        let resource = path.get_from()?.clone();
        Some(Self {
            path,
            frame: 0,
            resource,
            ui_data: AttackInfoUi::new(),
            done: Status::NotDone,
            transition: Transition::None,
        })
    }
}
