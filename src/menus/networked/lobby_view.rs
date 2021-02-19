use crate::app_state::{AppContext, AppState, Transition};
use fg_netcode::{lobby::Lobby, player_info::PlayerInfo};
use ggez::{graphics, Context, GameResult};
use imgui::{im_str, Condition};

use inspect_design::traits::InspectMut;
use sdl_controller_backend::ControllerId;

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
        enum Action {
            None,
            CreateGame,
            JoinGame(usize),
            UpdateUser(PlayerInfo),
        }
        let mut action = Action::None;
        frame
            .run(|ui| {
                imgui::Window::new(im_str!("Lobby"))
                    .save_settings(false)
                    .size([0.0, 0.0], Condition::Always)
                    .resizable(false)
                    .build(ui, || {
                        let lobby_state = self.lobby.state();
                        ui.text(im_str!("Host: {}", lobby_state.host().name));
                        ui.separator();
                        ui.text(im_str!("Clients:"));
                        ui.indent();
                        for player in lobby_state.clients().iter() {
                            ui.text(im_str!("{}", player.name));
                        }
                        ui.unindent();
                        ui.separator();
                        ui.text(im_str!("Players:"));
                        ui.indent();
                        for player in lobby_state.players().iter() {
                            ui.text(im_str!("{}", player.name));
                        }
                        ui.unindent();
                        ui.separator();

                        if ui.small_button(im_str!("Create Game")) {
                            action = Action::CreateGame;
                        }

                        ui.separator();
                        ui.indent();
                        for (idx, game) in lobby_state.games().iter().enumerate() {
                            ui.text(im_str!("Game {}:", idx + 1));
                            ui.indent();

                            ui.text(im_str!("Players:"));
                            ui.indent();
                            for player in game.players().iter().map(|player| &lobby_state[*player])
                            {
                                ui.text(im_str!("{}", player.name));
                            }
                            ui.unindent();

                            ui.text(im_str!("Spectators:"));
                            ui.indent();
                            for player in
                                game.spectators().iter().map(|player| &lobby_state[*player])
                            {
                                ui.text(im_str!("{}", player.name));
                            }
                            ui.unindent();

                            ui.unindent();
                            if ui.small_button(im_str!("Join")) {
                                action = Action::JoinGame(idx);
                            }
                            ui.separator();
                        }
                        ui.unindent();

                        ui.separator();
                        let mut user = lobby_state.user().clone();
                        user.name.inspect_mut("Name", &mut (), ui);
                        user.character.inspect_mut("Character", &mut (), ui);
                        if &user != lobby_state.user() {
                            action = Action::UpdateUser(user);
                        }

                        ui.separator();

                        if ui.small_button(im_str!("Leave")) {
                            self.next = NextState::Back;
                        }
                    });
            })
            .render(ctx);

        match action {
            Action::JoinGame(idx) => self.lobby.join_game(idx).unwrap(),
            Action::CreateGame => self.lobby.create_game(),
            Action::None => (),
            Action::UpdateUser(user) => self.lobby.update_player_data(move |data| *data = user),
        }

        graphics::present(ctx)?;

        Ok(())
    }
}
