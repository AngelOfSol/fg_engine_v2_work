use crate::app_state::{AppContext, AppState, Transition};
use ggez::graphics;
use ggez::{Context, GameResult};
use imgui::im_str;

enum NextState {
    Back,
    DisplaySettings,
    ControlSettings,
}

pub struct SettingsMenu {
    next: Option<NextState>,
}

impl SettingsMenu {
    pub fn new() -> Self {
        Self { next: None }
    }
}

impl AppState for SettingsMenu {
    fn update(
        &mut self,
        ctx: &mut Context,
        _: &mut AppContext,
    ) -> GameResult<crate::app_state::Transition> {
        match std::mem::replace(&mut self.next, None) {
            Some(state) => match state {
                NextState::Back => Ok(Transition::Pop),
                NextState::DisplaySettings => {
                    Ok(Transition::Push(Box::new(super::DisplaySettings::new(ctx))))
                }
                NextState::ControlSettings => {
                    Ok(Transition::Push(Box::new(super::ButtonCheck::new(ctx)?)))
                }
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
                imgui::Window::new(im_str!("Settings")).build(ui, || {
                    if ui.small_button(im_str!("Display Settings")) {
                        self.next = Some(NextState::DisplaySettings);
                    }
                    if ui.small_button(im_str!("Control Settings")) {
                        self.next = Some(NextState::ControlSettings);
                    }
                    if ui.small_button(im_str!("Back")) {
                        self.next = Some(NextState::Back);
                    }
                });
            })
            .render(ctx);

        graphics::present(ctx)?;

        Ok(())
    }
}
