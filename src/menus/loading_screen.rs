use crate::app_state::{AppContext, AppState, Transition};
use ggez::{graphics, Context, GameResult};
use imgui::im_str;

pub struct LoadingScreen {
    next: Transition,
}

impl LoadingScreen {
    pub fn new(next: Transition) -> Self {
        Self { next }
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

        crate::graphics::prepare_screen_for_game(ctx)?;

        let mut prerender_text =
            ggez::graphics::Text::new("0123456789 hits\n0123456789 damage\n0123456789 limit");

        prerender_text.set_bounds([400.0, 400.0], ggez::graphics::Align::Left);
        prerender_text.set_font(Default::default(), ggez::graphics::Scale::uniform(30.0));
        ggez::graphics::draw(ctx, &prerender_text, ggez::graphics::DrawParam::default())?;

        prerender_text.set_bounds([400.0, 400.0], ggez::graphics::Align::Right);
        ggez::graphics::draw(ctx, &prerender_text, ggez::graphics::DrawParam::default())?;

        let mut prerender_text = ggez::graphics::Text::new("0123456789");
        prerender_text.set_font(Default::default(), ggez::graphics::Scale::uniform(38.0));
        ggez::graphics::draw(ctx, &prerender_text, ggez::graphics::DrawParam::default())?;

        crate::graphics::prepare_screen_for_editor(ctx)?;

        let frame = imgui.frame();
        frame
            .run(|ui| {
                imgui::Window::new(im_str!("Loading...")).build(ui, || {});
            })
            .render(ctx);

        graphics::present(ctx)?;

        Ok(())
    }
}
