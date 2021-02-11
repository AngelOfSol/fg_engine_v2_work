use crate::app_state::{AppContext, AppState, Transition};
use fg_netcode::lobby::Lobby;
use ggez::{graphics, Context, GameResult};
use imgui::{im_str, Condition};

use sdl_controller_backend::ControllerId;

use super::lobby_select;

enum NextState {
    Back,
    None,
}
pub struct LobbyView {
    next: NextState,
    lobby: Lobby,
    main_player: ControllerId,
}

impl LobbyView {
    pub fn new(main_player: ControllerId, lobby: Lobby) -> Self {
        Self {
            next: NextState::None,
            lobby,
            main_player,
        }
    }
}

impl AppState for LobbyView {
    fn update(
        &mut self,
        ctx: &mut Context,
        AppContext { .. }: &mut AppContext,
    ) -> GameResult<crate::app_state::Transition> {
        while ggez::timer::check_update_time(ctx, 60) {}

        match std::mem::replace(&mut self.next, NextState::None) {
            NextState::Back => Ok(Transition::Pop),
            NextState::None => Ok(Transition::None),
        }
    }
    fn on_enter(&mut self, _: &mut Context, _: &mut AppContext) -> GameResult<()> {
        Ok(())
    }
    fn draw(
        &mut self,
        ctx: &mut Context,
        AppContext { imgui, .. }: &mut AppContext,
    ) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        let frame = imgui.frame();

        frame
            .run(|ui| {
                imgui::Window::new(im_str!("Lobby"))
                    .save_settings(false)
                    .size([0.0, 0.0], Condition::Appearing)
                    .resizable(false)
                    .build(ui, || {
                        let lobby_state = self.lobby.state();
                        ui.text(im_str!("Players:"));
                        ui.indent();
                        for player in lobby_state.player_list.iter() {}
                        ui.unindent();

                        ui.separator();

                        if ui.small_button(im_str!("Leave")) {
                            self.next = NextState::Back;
                        }
                    });
            })
            .render(ctx);

        graphics::present(ctx)?;

        Ok(())
    }
}
