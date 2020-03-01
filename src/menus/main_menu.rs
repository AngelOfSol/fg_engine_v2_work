use super::SettingsMenu;
use crate::app_state::{AppContext, AppState, Transition};
use crate::ui::editor::EditorMenu;
use ggez::graphics;
use ggez::{Context, GameResult};
use imgui::im_str;

enum NextState {
    Quit,
    Editor,
    Settings,
}

pub struct MainMenu {
    next: Option<NextState>,
}

impl MainMenu {
    pub fn new() -> Self {
        Self { next: None }
    }
}

impl AppState for MainMenu {
    fn update(
        &mut self,
        _: &mut Context,
        _: &mut AppContext,
    ) -> GameResult<crate::app_state::Transition> {
        match std::mem::replace(&mut self.next, None) {
            Some(state) => match state {
                NextState::Quit => Ok(Transition::Pop),
                NextState::Editor => Ok(Transition::Push(Box::new(EditorMenu::new()))),
                NextState::Settings => Ok(Transition::Push(Box::new(SettingsMenu::new()))),
            },
            None => Ok(Transition::None),
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

        let frame = imgui.frame();
        frame
            .run(|ui| {
                imgui::Window::new(im_str!("Main Menu")).build(ui, || {
                    if ui.small_button(im_str!("Editor")) {
                        self.next = Some(NextState::Editor);
                    }
                    if ui.small_button(im_str!("Settings")) {
                        self.next = Some(NextState::Settings);
                    }
                    if ui.small_button(im_str!("Quit")) {
                        self.next = Some(NextState::Quit);
                    }
                });
            })
            .render(ctx);

        graphics::present(ctx)?;

        Ok(())
    }
}
