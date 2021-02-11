use std::net::SocketAddr;

use crate::player_list::PlayerList;
use crate::{
    app_state::{AppContext, AppState, Transition},
    menus::gameplay::controller_select::FromControllerList,
};
use fg_netcode::{
    error::{HostLobbyError, JoinLobbyError},
    lobby::Lobby,
    query::Query,
    HostLobbyQuery, JoinLobbyQuery,
};
use ggez::{graphics, Context, GameResult};
use imgui::im_str;
use inspect_design::{from_str::InspectAsText, traits::InspectMut};
use sdl_controller_backend::ControllerId;

use super::lobby_view::LobbyView;

enum NextState {
    Lobby(Lobby),
    Back,
    None,
}

impl FromControllerList for LobbySelect {
    fn from_controllers(data: PlayerList) -> GameResult<Box<Self>> {
        Ok(Box::new(Self::new(
            data.current_players.p1().gamepad_id().unwrap(),
        )?))
    }
}

pub struct LobbySelect {
    next: NextState,
    name: String,
    host_query: HostLobbyQuery,
    join_query: JoinLobbyQuery,
    join_ip: InspectAsText<SocketAddr>,
    main_player: ControllerId,
}

impl LobbySelect {
    pub fn new(main_player: ControllerId) -> GameResult<Self> {
        Ok(Self {
            next: NextState::None,
            name: "THE Angel of Sol".to_string(),
            host_query: Query::default(),
            join_query: Query::default(),
            join_ip: InspectAsText::default(),
            main_player,
        })
    }
}

impl AppState for LobbySelect {
    fn update(
        &mut self,
        ctx: &mut Context,
        AppContext { .. }: &mut AppContext,
    ) -> GameResult<crate::app_state::Transition> {
        self.host_query.poll();
        if matches!(self.host_query, Query::Ok(_)) {
            self.next = NextState::Lobby(self.host_query.take().unwrap());
        }
        self.join_query.poll();
        if matches!(self.join_query, Query::Ok(_)) {
            self.next = NextState::Lobby(self.join_query.take().unwrap());
        }

        while ggez::timer::check_update_time(ctx, 60) {}

        match std::mem::replace(&mut self.next, NextState::None) {
            NextState::Lobby(lobby) => Ok(Transition::Push(Box::new(LobbyView::new(
                self.main_player,
                lobby,
            )))),
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
        AppContext {
            imgui, networking, ..
        }: &mut AppContext,
    ) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        let frame = imgui.frame();

        frame
            .run(|ui| {
                imgui::Window::new(im_str!("Lobby Select")).build(ui, || {
                    if ui.small_button(im_str!("Back")) {
                        self.next = NextState::Back;
                    }
                    self.name.inspect_mut("Name", &mut (), ui);

                    if ui.small_button(im_str!("Host")) {
                        self.host_query = networking.request_host(self.name.clone());
                    }
                    ui.text(im_str!(
                        "{}",
                        match &self.host_query {
                            Query::Waiting(_) => "Attempting to host...",
                            Query::Ok(_) => "Host successful!",
                            Query::Err(error) => match error {
                                HostLobbyError::NetworkError =>
                                    "Host request failed due to network error.",
                                HostLobbyError::InLobby => unreachable!(),
                            },
                            Query::Cancelled => "Host request has been cancelled.",
                            Query::NotStarted => "No host query started.",
                        }
                    ));

                    if ui.small_button(im_str!("Join")) {
                        //self.host_query = networking.request_host();
                        ui.open_popup(im_str!("JoinModal"));
                    }

                    ui.popup_modal(im_str!("JoinModal"))
                        .always_auto_resize(true)
                        .always_use_window_padding(true)
                        .movable(false)
                        .resizable(false)
                        .collapsible(false)
                        .build(|| {
                            self.join_ip.inspect_mut("IP", &mut (), ui);
                            if self.join_ip.is_valid() {
                                if ui.small_button(im_str!("Join")) {
                                    self.join_query = networking.request_join(
                                        self.join_ip.take().unwrap(),
                                        self.name.clone(),
                                    );
                                    ui.close_current_popup();
                                }
                                ui.same_line(0.0);
                            }
                            if ui.small_button(im_str!("Cancel")) {
                                ui.close_current_popup();
                            }
                        });
                    ui.text(im_str!(
                        "{}",
                        match &self.join_query {
                            Query::Waiting(_) => "Attempting to join...",
                            Query::Ok(_) => "Join successful!",
                            Query::Err(error) => match error {
                                JoinLobbyError::NetworkError =>
                                    "Join request failed due to network error.",
                                JoinLobbyError::InLobby => unreachable!(),
                                JoinLobbyError::Denied => "Join request denied by host.",
                            },
                            Query::Cancelled => "Join request has been cancelled.",
                            Query::NotStarted => "No join query started.",
                        }
                    ));
                });
            })
            .render(ctx);

        graphics::present(ctx)?;

        Ok(())
    }
}
