use crate::game::{GameState, Transition};


use ggez::graphics;
use ggez::graphics::Color;
use ggez::{Context, GameResult};

use crate::animation::{Animation, AnimationUi};

use crate::assets::Assets;
use crate::game;
use crate::timeline::AtTime;

use crate::imgui_wrapper::ImGuiWrapper;

use imgui::*;

use crate::character_state::CharacterState;

pub struct StateEditor {
    resource: CharacterState,
    done: bool,
}

impl StateEditor {
    pub fn new() -> Self {
        Self {
            resource: CharacterState::new(),
            done: false,
        }
    }

    pub fn update(&mut self) -> GameResult<Transition> {
        if self.done {
            Ok(Transition::Pop)
        } else {
            Ok(Transition::None)
        }
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
                    ui.menu(im_str!("State Editor")).build(|| {
                        if ui.menu_item(im_str!("New")).build() {}
                        ui.separator();

                        if ui.menu_item(im_str!("Back")).build() {
                            self.done = true;
                        }
                    });
                });
            })
            .render(ctx);
        Ok(())
    }
}

impl Into<GameState> for StateEditor {
    fn into(self) -> GameState {
        GameState::StateEditor(self)
    }
}