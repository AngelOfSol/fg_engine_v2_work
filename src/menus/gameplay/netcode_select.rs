use super::controller_select::FromControllerList;
use super::netplay_versus::NetplayVersus;
use super::CharacterSelect;
use crate::app_state::{AppContext, AppState, Transition};
use crate::imgui_extra::UiExtensions;
use crate::input::pads_context::{Button, EventType, GamepadId};
use crate::player_list::{PlayerList, PlayerType};
use ggez::{graphics, Context, GameResult};
use imgui::im_str;
use laminar::{Packet, SocketEvent};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::net::SocketAddr;
use std::time::Instant;
use strum_macros::Display;

enum NextState {
    Next(GamepadId),
    Back,
}

#[derive(Debug, Clone)]
enum PotentialAddress {
    Almost(String),
    Address(SocketAddr),
}

impl Display for PotentialAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PotentialAddress::Almost(data) => write!(f, "{}", data),
            PotentialAddress::Address(data) => write!(f, "{}", data),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Display)]
enum Mode {
    Host,
    Client,
}

impl FromControllerList for NetworkConnect {
    fn from_controllers(data: PlayerList) -> GameResult<Box<Self>> {
        Ok(Box::new(Self::new(data)?))
    }
}

pub struct NetworkConnect {
    next: Option<NextState>,
    mode: Mode,
    target_addr: PotentialAddress,
    connected: bool,
    player_list: PlayerList,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
enum NetPacket {
    Close,
    RequestJoin,
    DenyJoin,
    ConfirmJoin,
    MoveToCharacterSelect,
}

impl NetworkConnect {
    pub fn new(player_list: PlayerList) -> GameResult<Self> {
        Ok(Self {
            next: None,
            connected: false,
            target_addr: PotentialAddress::Almost(String::with_capacity(30)),
            mode: Mode::Host,
            player_list,
        })
    }
}

impl AppState for NetworkConnect {
    fn update(
        &mut self,
        ctx: &mut Context,
        AppContext {
            ref mut pads,
            ref mut socket,
            ..
        }: &mut AppContext,
    ) -> GameResult<crate::app_state::Transition> {
        let socket = socket
            .as_mut()
            .expect("expected to have a socket during netcode select");
        socket.manual_poll(Instant::now());

        while let Some(packet) = socket.recv() {
            match packet {
                SocketEvent::Packet(_) => self.connected = true,
                SocketEvent::Connect(addr) => {
                    if self.mode == Mode::Host {
                        self.target_addr = PotentialAddress::Address(addr);
                        *self.player_list.current_players.p2_mut() = PlayerType::Networked(addr);
                    }
                    self.connected = true;
                    socket
                        .send(Packet::reliable_sequenced(addr, vec![], None))
                        .map_err(|_| {
                            ggez::GameError::EventLoopError("Could not send packet".to_owned())
                        })?;
                }
                SocketEvent::Timeout(_) => self.connected = false,
            }
        }

        while ggez::timer::check_update_time(ctx, 2) {}

        while let Some(event) = pads.next_event() {
            match event.event {
                EventType::ButtonPressed(button) => {
                    if button == Button::Start && self.connected {
                        self.next = Some(NextState::Next(event.id));
                    }
                }
                _ => (),
            }
        }

        match std::mem::replace(&mut self.next, None) {
            Some(state) => match state {
                NextState::Next(id) => match self.mode {
                    Mode::Host => Ok(Transition::Replace(Box::new(CharacterSelect::<
                        NetplayVersus,
                    >::new(
                        PlayerList::new(
                            [
                                id.into(),
                                match self.target_addr {
                                    PotentialAddress::Address(addr) => addr,
                                    _ => unreachable!(),
                                }
                                .into(),
                            ]
                            .into(),
                        ),
                    )))),
                    Mode::Client => Ok(Transition::Replace(Box::new(CharacterSelect::<
                        NetplayVersus,
                    >::new(
                        PlayerList::new(
                            [
                                match self.target_addr {
                                    PotentialAddress::Address(addr) => addr,
                                    _ => unreachable!(),
                                }
                                .into(),
                                id.into(),
                            ]
                            .into(),
                        ),
                    )))),
                },
                NextState::Back => Ok(Transition::Pop),
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
        AppContext {
            ref mut imgui,
            ref mut socket,
            ..
        }: &mut AppContext,
    ) -> GameResult<()> {
        let socket = socket
            .as_mut()
            .expect("expected to have a socket during netcode select");

        graphics::clear(ctx, graphics::BLACK);

        let frame = imgui.frame();

        frame
            .run(|ui| {
                imgui::Window::new(im_str!("Network")).build(ui, || {
                    if ui.small_button(im_str!("Back")) {
                        self.next = Some(NextState::Back);
                    }

                    if self.connected {
                        ui.text(im_str!("Mode: {}", self.mode));
                    } else {
                        if ui.combo_items(
                            im_str!("Mode"),
                            &mut self.mode,
                            &[Mode::Host, Mode::Client],
                            &|item| match item {
                                Mode::Host => im_str!("Host").into(),
                                Mode::Client => im_str!("Client").into(),
                            },
                        ) {
                            self.player_list.swap_players();
                        }
                    }

                    ui.text(im_str!(
                        "Current Address: {}",
                        socket
                            .local_addr()
                            .map(|item| item.to_string())
                            .unwrap_or("Error".to_owned())
                    ));

                    if self.mode == Mode::Host || self.connected {
                        ui.text(&im_str!("IP: {}", self.target_addr));

                        if let PotentialAddress::Address(addr) = self.target_addr {
                            if ui.small_button(im_str!("Disconnect")) {
                                self.connected = false;
                                let _ = socket.send(Packet::reliable_sequenced(
                                    addr,
                                    bincode::serialize(&NetPacket::Close).unwrap(),
                                    None,
                                ));
                            }
                        }

                        if self.mode == Mode::Host {
                            ui.text("Press start to move to character select.");
                        }
                    } else {
                        let mut buffer = self.target_addr.to_string();

                        if ui.input_string(im_str!("IP"), &mut buffer) {
                            self.target_addr = match buffer.parse() {
                                Ok(addr) => PotentialAddress::Address(addr),
                                Err(_) => PotentialAddress::Almost(buffer),
                            };
                        }
                        if let PotentialAddress::Address(addr) = self.target_addr {
                            if ui.small_button(im_str!("Connect")) {
                                let _ = socket.send(Packet::reliable_sequenced(
                                    addr,
                                    bincode::serialize(&NetPacket::RequestJoin).unwrap(),
                                    None,
                                ));
                            }
                        }
                    }
                });
            })
            .render(ctx);

        graphics::present(ctx)?;

        Ok(())
    }
}
