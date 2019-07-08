use crate::game::{GameState, MessageData, Mode, Transition};

use ggez::{Context, GameResult};

use crate::assets::Assets;
use crate::imgui_wrapper::ImGuiWrapper;


use crate::character_state::{
    AnimationData, CancelSetUi, CharacterState, CharacterStateUi, FlagsUi, MovementData,
};

use crate::game::AnimationEditor;

use crate::animation::Animation;

use crate::imgui_extra::UiExtensions;
use imgui::*;

use crate::character::PlayerCharacter;

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
                ui.window(im_str!("Editor"))
                    .size([300.0, 526.0], Condition::Always)
                    .position([0.0, 20.0], Condition::Always)
                    .resizable(false)
                    .movable(false)
                    .collapsible(false)
                    .build(|| {
                        
                    });
                ui.main_menu_bar(|| {
                    ui.menu(im_str!("Player Editor")).build(|| {
                        if ui.menu_item(im_str!("New")).build() {
                            self.resource = PlayerCharacter::new();
                        }
                        if ui.menu_item(im_str!("Save")).build() {
                            if let Ok(nfd::Response::Okay(path)) = nfd::open_pick_folder(None) {
                                editor_result = PlayerCharacter::save(
                                    ctx,
                                    assets,
                                    &self.resource,
                                    PathBuf::from(path),
                                );
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
                                    Ok(state) => {
                                        self.resource = state;
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

impl Into<GameState> for CharacterEditor {
    fn into(self) -> GameState {
        GameState::CharacterEditor(self)
    }
}
