use crate::assets::Assets;
use crate::character::components::AttackInfo;
use crate::imgui_wrapper::ImGuiWrapper;
use crate::ui::character::components::AttackInfoUi;
use crate::ui::editor::{EditorState, MessageData, Transition};
use ggez::{Context, GameResult};
use imgui::*;

enum Status {
    DoneAndSave,
    DoneAndQuit,
    NotDone,
}

pub struct AttackInfoEditor {
    frame: usize,
    resource: AttackInfo,
    ui_data: AttackInfoUi,
    done: Status,
    transition: Transition,
}

impl AttackInfoEditor {
    pub fn with_attack(data: AttackInfo) -> Self {
        Self {
            frame: 0,
            resource: data,
            ui_data: AttackInfoUi::new(),
            done: Status::NotDone,
            transition: Transition::None,
        }
    }

    pub fn update(&mut self) -> GameResult<Transition> {
        self.frame = self.frame.wrapping_add(1);

        match self.done {
            Status::NotDone => Ok(std::mem::replace(&mut self.transition, Transition::None)),
            Status::DoneAndSave => {
                let ret = std::mem::replace(&mut self.resource, AttackInfo::new());
                Ok(Transition::Pop(Some(MessageData::AttackInfo(ret))))
            }
            Status::DoneAndQuit => Ok(Transition::Pop(None)),
        }
    }

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        _assets: &mut Assets,
        imgui: &mut ImGuiWrapper,
    ) -> GameResult<()> {
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
                            self.resource = AttackInfo::new();
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

        Ok(())
    }
}

impl Into<EditorState> for AttackInfoEditor {
    fn into(self) -> EditorState {
        EditorState::AttackInfoEditor(self)
    }
}
