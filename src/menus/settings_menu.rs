use crate::app_state::{AppState, Transition};
use crate::imgui_wrapper::ImGuiWrapper;
use ggez::graphics;
use ggez::{Context, GameResult};
use imgui::im_str;

enum NextState {
    Back,
    DisplaySettings,
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
    fn update(&mut self, ctx: &mut Context) -> GameResult<crate::app_state::Transition> {
        match std::mem::replace(&mut self.next, None) {
            Some(state) => match state {
                NextState::Back => Ok(Transition::Pop),
                NextState::DisplaySettings => {
                    Ok(Transition::Push(Box::new(super::DisplaySettings::new(ctx))))
                }
            },
            None => Ok(Transition::None),
        }
    }
    fn on_enter(&mut self, _: &mut Context) -> GameResult<()> {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context, imgui: &mut ImGuiWrapper) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        let frame = imgui.frame();
        frame
            .run(|ui| {
                imgui::Window::new(im_str!("Settings")).build(ui, || {
                    if ui.small_button(im_str!("Display Settings")) {
                        self.next = Some(NextState::DisplaySettings);
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
