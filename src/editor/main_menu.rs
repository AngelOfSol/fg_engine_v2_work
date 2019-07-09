use ggez::{Context, GameResult};

use imgui::*;

use crate::imgui_wrapper::ImGuiWrapper;

use super::{AnimationEditor, CharacterEditor, StateEditor};
use crate::editor::{EditorState, Mode, Transition};

pub struct MainMenu {
    next: Transition,
}

impl MainMenu {
    pub fn new() -> Self {
        Self {
            next: Transition::None,
        }
    }

    pub fn update(&mut self) -> GameResult<Transition> {
        let next = std::mem::replace(&mut self.next, Transition::None);
        Ok(next)
    }

    pub fn draw(&mut self, ctx: &mut Context, imgui: &mut ImGuiWrapper) -> GameResult<()> {
        imgui
            .frame()
            .run(|ui| {
                // Window
                ui.main_menu_bar(|| {
                    ui.menu(im_str!("Main Menu")).build(|| {
                        if ui.menu_item(im_str!("Edit Animations")).build() {
                            self.next = Transition::Push(
                                Box::new(AnimationEditor::new().into()),
                                Mode::Standalone,
                            );
                        }

                        if ui.menu_item(im_str!("Edit States")).build() {
                            self.next = Transition::Push(
                                Box::new(StateEditor::new().into()),
                                Mode::Standalone,
                            );
                        }
                        if ui.menu_item(im_str!("Edit Character")).build() {
                            self.next = Transition::Push(
                                Box::new(CharacterEditor::new().into()),
                                Mode::Standalone,
                            );
                        }
                        ui.separator();
                        if ui.menu_item(im_str!("Quit")).build() {
                            self.next = Transition::Pop(None);
                        }
                    });
                });
            })
            .render(ctx);
        Ok(())
    }
}

impl Into<EditorState> for MainMenu {
    fn into(self) -> EditorState {
        EditorState::MainMenu(self)
    }
}
