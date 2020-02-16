use crate::app_state::{AppState, Transition};
use crate::imgui_wrapper::ImGuiWrapper;
use ggez::graphics;
use ggez::{Context, GameResult};
use imgui::im_str;

enum NextState {
    Quit,
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
    fn update(&mut self, _: &mut Context) -> GameResult<crate::app_state::Transition> {
        match &self.next {
            Some(state) => match state {
                NextState::Quit => Ok(Transition::Pop),
            },
            None => Ok(Transition::None),
        }
    }
    fn draw(&mut self, ctx: &mut Context, imgui: &mut ImGuiWrapper) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        let frame = imgui.frame();
        frame
            .run(|ui| {
                ui.main_menu_bar(|| {
                    ui.menu(im_str!("Main Menu"), true, || {
                        if imgui::MenuItem::new(im_str!("Quit")).build(ui) {
                            self.next = Some(NextState::Quit);
                        }
                    });
                });
            })
            .render(ctx);

        graphics::present(ctx)?;

        Ok(())
    }
}
