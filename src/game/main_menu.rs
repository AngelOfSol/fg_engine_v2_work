use ggez::{Context, GameResult};

use imgui::*;

use crate::imgui_wrapper::ImGuiWrapper;

use super::{AnimationEditor, StateEditor};
use crate::game::{GameState, Mode, Transition};

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
                    });
                });
            })
            .render(ctx);
        Ok(())
    }
}

impl Into<GameState> for MainMenu {
    fn into(self) -> GameState {
        GameState::MainMenu(self)
    }
}
