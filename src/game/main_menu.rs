use ggez::{Context, GameResult};

use imgui::*;

use crate::imgui_wrapper::ImGuiWrapper;

use super::AnimationEditor;
use crate::game;

pub struct MainMenu {
    next: game::Transition,
}

impl MainMenu {

    pub fn new() -> Self {
        Self {
            next: game::Transition::None,
        }
    }

    pub fn update(&mut self) -> GameResult<game::Transition> {
        let next = std::mem::replace(&mut self.next, game::Transition::None);
        Ok(next)
    }

    pub fn draw<'a>(&mut self, ctx: &mut Context, imgui: &'a mut ImGuiWrapper) -> GameResult<()> {
        let imgui_render = imgui.frame().run(|ui| {
            // Window
            ui.main_menu_bar(|| {
                ui.menu(im_str!("Editor")).build(|| {
                    if ui.menu_item(im_str!("Edit Animations")).build() {
                        self.next = game::Transition::Push(Box::new(AnimationEditor::new().into()));
                    }
                });
            });
        });
        imgui_render.render(ctx);
        Ok(())
    }
}

impl Into<game::GameState> for MainMenu {
    fn into(self) -> game::GameState {
        game::GameState::MainMenu(self)
    }
}

/*GameState::Animating(AnimationEditor::new(
    ctx,
    &mut assets,
)?)*/