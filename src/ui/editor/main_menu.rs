use crate::assets::Assets;
use crate::character::PlayerCharacter;
use crate::imgui_wrapper::ImGuiWrapper;
use crate::ui::editor::{CharacterEditor, EditorState, Mode, Transition};
use ggez::{Context, GameResult};
use imgui::*;
use std::path::PathBuf;

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
                let id = ui.push_id("Editor Main Menu");
                // Window
                imgui::Window::new(im_str!("Editor Menu")).build(ui, || {
                    if ui.small_button(im_str!("New Character")) {
                        self.next = Transition::Push(
                            Box::new(CharacterEditor::new(PlayerCharacter::new()).into()),
                            Mode::Standalone,
                        );
                    }
                    if ui.small_button(im_str!("Load Character")) {
                        if let Ok(nfd::Response::Okay(path)) =
                            nfd::open_file_dialog(Some("json"), None)
                        {
                            let result =
                                PlayerCharacter::load_from_json(ctx, assets, PathBuf::from(path));
                            if result.is_err() {
                                dbg!(result.as_ref().unwrap_err());
                            }
                            if let Ok(character) = result {
                                self.next = Transition::Push(
                                    Box::new(CharacterEditor::new(character).into()),
                                    Mode::Standalone,
                                );
                            }
                        }
                    }
                    if ui.small_button(im_str!("Quit")) {
                        self.next = Transition::Pop(None);
                    }
                });
                id.pop(ui);
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
