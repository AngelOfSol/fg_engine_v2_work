use std::net::SocketAddr;

use crate::player_list::PlayerList;
use crate::{
    app_state::{AppContext, AppState, Transition},
    menus::gameplay::controller_select::FromControllerList,
};
use fg_datastructures::roster::RosterCharacter;
use fg_netcode::{lobby::Lobby, player_info::PlayerInfo, NetworkingMessage};
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

enum UiState {
    Main(Option<String>),
    Joining,
    Hosting,
}

pub struct LobbySelect {
    next: NextState,
    state: UiState,
    join_ip: InspectAsText<SocketAddr>,
    main_player: ControllerId,
    user: PlayerInfo,
}

impl LobbySelect {
    pub fn new(main_player: ControllerId) -> GameResult<Self> {
        Ok(Self {
            next: NextState::None,
            state: UiState::Main(None),
            join_ip: InspectAsText::default(),
            main_player,
            user: PlayerInfo {
                name: "THE Angel of Sol".to_string(),
                character: RosterCharacter::default(),
                addr: "192.168.1.1:10800".parse().unwrap(),
            },
        })
    }
}

impl AppState for LobbySelect {
    fn update(
        &mut self,
        ctx: &mut Context,
        AppContext { networking, .. }: &mut AppContext,
    ) -> GameResult<crate::app_state::Transition> {
        while let Some(message) = networking.poll() {
            match message {
                NetworkingMessage::Host(Ok(lobby)) | NetworkingMessage::Join(Ok(lobby)) => {
                    self.next = NextState::Lobby(lobby)
                }
                _ => self.state = UiState::Main(Some("Connection failed.".to_string())),
            }
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
        self.state = UiState::Main(None);
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
                    self.user.name.inspect_mut("Name", &mut (), ui);
                    self.user.character.inspect_mut("Character", &mut (), ui);

                    match &self.state {
                        UiState::Main(text) => {
                            if let Some(text) = text {
                                ui.text(im_str!("{}", text));
                            }
                            if ui.small_button(im_str!("Host")) {
                                networking.request_host(self.user.clone());
                                self.state = UiState::Hosting;
                            }

                            if ui.small_button(im_str!("Join")) {
                                ui.open_popup(im_str!("JoinModal"));
                            }
                        }
                        UiState::Joining => ui.text("Joining..."),
                        UiState::Hosting => ui.text("Hosting..."),
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
                                    networking.request_join(
                                        self.join_ip.take().unwrap(),
                                        self.user.clone(),
                                    );
                                    self.state = UiState::Joining;
                                    ui.close_current_popup();
                                }
                                ui.same_line(0.0);
                            }
                            if ui.small_button(im_str!("Cancel")) {
                                ui.close_current_popup();
                            }
                        });
                });
            })
            .render(ctx);

        graphics::present(ctx)?;

        Ok(())
    }
}
