use crate::editor::{EditorState, MessageData, Mode, StateEditor, Transition};

use ggez::{Context, GameResult};

use crate::assets::Assets;
use crate::imgui_wrapper::ImGuiWrapper;

use crate::character_state::CharacterState;

use imgui::*;

use crate::character::{PlayerCharacter, PropertiesUi, StatesUi};

use std::path::PathBuf;

pub struct CharacterEditor {
    resource: PlayerCharacter,
    transition: Transition,
}

impl CharacterEditor {
    pub fn new() -> Self {
        Self {
            resource: PlayerCharacter::new(),
            transition: Transition::None,
        }
    }

    pub fn handle_message(&mut self, data: MessageData, mode: Mode) {
        if let MessageData::State(state) = data {
            match mode {
                Mode::Standalone => (),
                Mode::New => {
                    self.resource.states.rest.insert(
                        self.resource.states.guarentee_unique_key("new state"),
                        state,
                    );
                }
                Mode::Edit(name) => {
                    self.resource.states.replace_state(name, state);
                }
            }
        }
    }

    pub fn update(&mut self) -> GameResult<Transition> {
        let ret = std::mem::replace(&mut self.transition, Transition::None);
        Ok(ret)
    }

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        assets: &mut Assets,
        imgui: &mut ImGuiWrapper,
    ) -> GameResult<()> {
        let mut editor_result = Ok(());
        imgui
            .frame()
            .run(|ui| {
                ui.window(im_str!("Fields"))
                    .size([300.0, 526.0], Condition::Always)
                    .position([0.0, 20.0], Condition::Always)
                    .resizable(false)
                    .movable(false)
                    .collapsible(false)
                    .build(|| {
                        PropertiesUi::draw_ui(ui, &mut self.resource.properties);
                    });
                ui.window(im_str!("States"))
                    .size([300.0, 526.0], Condition::Always)
                    .position([300.0, 20.0], Condition::Always)
                    .resizable(false)
                    .movable(false)
                    .collapsible(false)
                    .build(|| {
                        let edit_result =
                            StatesUi::new().draw_ui(ctx, assets, ui, &mut self.resource.states);
                        if let Ok(Some(mode)) = &edit_result {
                            let state = match mode {
                                Mode::Edit(key) => self.resource.states.get_state(key).clone(),
                                _ => CharacterState::new(),
                            };
                            self.transition = Transition::Push(
                                Box::new(StateEditor::with_state(state).into()),
                                mode.clone(),
                            );
                        }
                        editor_result = edit_result.map(|_| ());
                    });
                ui.main_menu_bar(|| {
                    ui.menu(im_str!("Player Editor")).build(|| {
                        if ui.menu_item(im_str!("New")).build() {
                            self.resource = PlayerCharacter::new();
                        }
                        if ui.menu_item(im_str!("Save")).build() {
                            if let Ok(nfd::Response::Okay(path)) =
                                nfd::open_save_dialog(Some("json"), None)
                            {
                                let mut path = PathBuf::from(path);
                                path.set_extension("json");
                                editor_result =
                                    PlayerCharacter::save(ctx, assets, &self.resource, path);
                            }
                        }
                        if ui.menu_item(im_str!("Open")).build() {
                            if let Ok(nfd::Response::Okay(path)) =
                                nfd::open_file_dialog(Some("json"), None)
                            {
                                match PlayerCharacter::load_from_json(
                                    ctx,
                                    assets,
                                    PathBuf::from(path),
                                ) {
                                    Ok(character) => {
                                        self.resource = character;
                                    }
                                    Err(err) => editor_result = Err(err),
                                }
                            }
                        }
                        ui.separator();

                        if ui.menu_item(im_str!("Back")).build() {
                            self.transition = Transition::Pop(None);
                        }
                    });
                });
            })
            .render(ctx);
        editor_result?;

        Ok(())
    }
}

impl Into<EditorState> for CharacterEditor {
    fn into(self) -> EditorState {
        EditorState::CharacterEditor(self)
    }
}