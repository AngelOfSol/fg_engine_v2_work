use ggez::{Context, GameResult};

use imgui::*;

use crate::imgui_wrapper::ImGuiWrapper;

use crate::editor::{CharacterEditor, EditorState, Mode, Transition};

use crate::assets::Assets;

use std::path::PathBuf;

use crate::character::PlayerCharacter;

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

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        assets: &mut Assets,
        imgui: &mut ImGuiWrapper,
    ) -> GameResult<()> {
        imgui
            .frame()
            .run(|ui| {
                // Window
                ui.main_menu_bar(|| {
                    ui.menu(im_str!("Main Menu")).build(|| {
                        if ui.menu_item(im_str!("New Character")).build() {
                            self.next = Transition::Push(
                                Box::new(CharacterEditor::new(PlayerCharacter::new()).into()),
                                Mode::Standalone,
                            );
                        }
                        if ui.menu_item(im_str!("Open Character")).build() {
                            if let Ok(nfd::Response::Okay(path)) =
                                nfd::open_file_dialog(Some("json"), None)
                            {
                                if let Ok(character) = PlayerCharacter::load_from_json(
                                    ctx,
                                    assets,
                                    PathBuf::from(path),
                                ) {
                                    self.next = Transition::Push(
                                        Box::new(CharacterEditor::new(character).into()),
                                        Mode::Standalone,
                                    );
                                }
                            }
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
