use crate::app_state::{AppContext, AppState, Transition};
use ggez::{graphics, Context, GameResult};
use imgui::im_str;

pub struct LoadingScreen {
    next: Transition,
    currently_loading: String,
}

impl LoadingScreen {
    pub fn new(currently_loading: String, next: Transition) -> Self {
        Self {
            next,
            currently_loading,
        }
    }
}

impl AppState for LoadingScreen {
    fn update(
        &mut self,
        ctx: &mut Context,
        AppContext { .. }: &mut AppContext,
    ) -> GameResult<crate::app_state::Transition> {
        while ggez::timer::check_update_time(ctx, 60) {}

        Ok(std::mem::replace(&mut self.next, Transition::None))
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
                imgui::Window::new(im_str!("Loading...")).build(ui, || {
                    ui.text(im_str!("Current loading: {}", self.currently_loading));
                });
            })
            .render(ctx);

        graphics::present(ctx)?;

        Ok(())
    }
}
