use crate::app_state::{AppState, Transition};
use crate::imgui_wrapper::ImGuiWrapper;
use crate::ui::editor::GameEditor;
use ggez::graphics;
use ggez::{Context, GameResult};
use imgui::im_str;

enum NextState {
    Quit,
    Editor,
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
    fn update(&mut self, ctx: &mut Context) -> GameResult<crate::app_state::Transition> {
        match std::mem::replace(&mut self.next, None) {
            Some(state) => match state {
                NextState::Quit => Ok(Transition::Pop),
                NextState::Editor => Ok(Transition::Push(Box::new(GameEditor::new(ctx)?))),
            },
            None => Ok(Transition::None),
        }
    }
    fn on_enter(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::set_mode(
            ctx,
            ggez::conf::WindowMode::default().dimensions(1280.0, 720.0),
        )?;
        graphics::set_screen_coordinates(ctx, ggez::graphics::Rect::new(0.0, 0.0, 1280.0, 720.0))
    }
    fn draw(&mut self, ctx: &mut Context, imgui: &mut ImGuiWrapper) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        let frame = imgui.frame();
        frame
            .run(|ui| {
                imgui::Window::new(im_str!("Main Menu")).build(ui, || {
                    if ui.small_button(im_str!("Editor")) {
                        self.next = Some(NextState::Editor);
                    }
                    if ui.small_button(im_str!("Button Check")) {
                        self.next = Some(NextState::Quit);
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
