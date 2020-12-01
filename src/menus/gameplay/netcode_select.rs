use super::controller_select::FromControllerList;
use super::netplay_versus::NetplayVersus;
use super::CharacterSelect;
use crate::app_state::{AppContext, AppState, Transition};
use crate::imgui_extra::UiExtensions;
use crate::input::pads_context::{Button, EventType};
use crate::player_list::{PlayerList, PlayerType};
use ggez::{graphics, Context, GameResult};
use imgui::im_str;
use laminar::{Packet, SocketEvent};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::net::SocketAddr;
use std::time::Instant;
use strum::IntoEnumIterator;
use strum_macros::Display;
use strum_macros::EnumIter;

enum NextState {
    Next,
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Display, EnumIter)]
enum Mode {
    Host,
    Client,
    Spectate,
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
    local_player: PlayerType,
    connected: bool,
    player_list: PlayerList,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
enum NetPacket {
    Ping,
    Close,
    CloseSpectate,
    RequestJoin,
    DenyJoin,
    DenySpectate,
    RequestSpectate,
    UpdatePlayer(SocketAddr),
    ConfirmSpectate,
    AddSpectate,
    RemoveSpectate(SocketAddr),
    ConfirmJoin(Vec<SocketAddr>),
    MoveToCharacterSelect,
}

impl NetworkConnect {
    pub fn new(player_list: PlayerList) -> GameResult<Self> {
        Ok(Self {
            next: None,
            connected: false,
            target_addr: PotentialAddress::Almost("192.168.1.155:10800".to_owned()),
            mode: Mode::Host,
            local_player: *player_list
                .current_players
                .iter()
                .find(|item| !item.is_dummy() && !item.is_networked())
                .unwrap(),
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

        'packets: while let Some(packet) = socket.recv() {
            match packet {
                SocketEvent::Packet(packet) => {
                    match bincode::deserialize(packet.payload()).unwrap() {
                        NetPacket::RequestJoin => {
                            if self.mode == Mode::Host && !self.connected {
                                self.target_addr = PotentialAddress::Address(packet.addr());
                                *self.player_list.current_players.p2_mut() =
                                    PlayerType::Networked(packet.addr());
                                self.connected = true;
                                let _ = socket.send(Packet::reliable_ordered(
                                    packet.addr(),
                                    bincode::serialize(&NetPacket::ConfirmJoin(
                                        self.player_list
                                            .spectators
                                            .iter()
                                            .filter_map(PlayerType::addr)
                                            .collect(),
                                    ))
                                    .unwrap(),
                                    None,
                                ));
                                for addr in self
                                    .player_list
                                    .spectators
                                    .iter()
                                    .filter_map(PlayerType::addr)
                                {
                                    let _ = socket.send(Packet::reliable_ordered(
                                        addr,
                                        bincode::serialize(&NetPacket::UpdatePlayer(packet.addr()))
                                            .unwrap(),
                                        None,
                                    ));
                                }
                            } else {
                                let _ = socket.send(Packet::reliable_ordered(
                                    packet.addr(),
                                    bincode::serialize(&NetPacket::DenyJoin).unwrap(),
                                    None,
                                ));
                            }
                        }
                        NetPacket::UpdatePlayer(player) => {
                            *self.player_list.current_players.p2_mut() = player.into();

                            let _ = socket.send(Packet::reliable_ordered(
                                player,
                                bincode::serialize(&NetPacket::AddSpectate).unwrap(),
                                None,
                            ));
                        }
                        NetPacket::ConfirmJoin(spectators) => {
                            self.connected = true;
                            *self.player_list.current_players.p1_mut() =
                                PlayerType::Networked(packet.addr());
                            self.player_list.spectators =
                                spectators.into_iter().map(Into::into).collect();
                        }
                        NetPacket::MoveToCharacterSelect => {
                            self.next = Some(NextState::Next);
                            break 'packets;
                        }
                        NetPacket::RemoveSpectate(addr) => {
                            self.player_list.spectators.retain(|item| {
                                item.addr().map(|item| item != addr).unwrap_or(true)
                            });
                        }
                        NetPacket::AddSpectate => {
                            self.player_list
                                .spectators
                                .push(PlayerType::Networked(packet.addr()));
                            let _ = socket.send(Packet::unreliable(
                                packet.addr(),
                                bincode::serialize(&NetPacket::Ping).unwrap(),
                            ));
                        }

                        NetPacket::RequestSpectate => {
                            if self.mode == Mode::Host {
                                self.player_list
                                    .spectators
                                    .push(PlayerType::Networked(packet.addr()));

                                let _ = socket.send(Packet::reliable_ordered(
                                    packet.addr(),
                                    bincode::serialize(&NetPacket::ConfirmSpectate).unwrap(),
                                    None,
                                ));

                                if let Some(addr) = self.player_list.current_players.p2().addr() {
                                    let _ = socket.send(Packet::reliable_ordered(
                                        packet.addr(),
                                        bincode::serialize(&NetPacket::UpdatePlayer(addr)).unwrap(),
                                        None,
                                    ));
                                }
                            } else {
                                let _ = socket.send(Packet::reliable_ordered(
                                    packet.addr(),
                                    bincode::serialize(&NetPacket::DenySpectate).unwrap(),
                                    None,
                                ));
                            }
                        }

                        NetPacket::ConfirmSpectate => {
                            self.player_list.current_players =
                                [packet.addr().into(), PlayerType::Dummy].into();
                            self.connected = true;
                        }
                        NetPacket::CloseSpectate => {
                            let addr = packet.addr();

                            self.player_list.spectators.retain(|item| {
                                item.addr().map(|item| item != addr).unwrap_or(true)
                            });

                            for update_addr in self
                                .player_list
                                .current_players
                                .iter()
                                .filter_map(PlayerType::addr)
                            {
                                let _ = socket.send(Packet::reliable_ordered(
                                    update_addr,
                                    bincode::serialize(&NetPacket::RemoveSpectate(addr)).unwrap(),
                                    None,
                                ));
                            }
                        }
                        NetPacket::DenySpectate | NetPacket::DenyJoin | NetPacket::Close => {
                            self.connected = false;

                            for player in self.player_list.current_players.iter_mut() {
                                if *player == packet.addr().into() {
                                    *player = PlayerType::Dummy;
                                }
                            }
                        }
                        NetPacket::Ping => {}
                    }
                }
                SocketEvent::Connect(_) => (),
                SocketEvent::Timeout(addr) => {
                    if self
                        .player_list
                        .current_players
                        .iter()
                        .filter_map(PlayerType::addr)
                        .any(|item| item == addr)
                    {
                        self.connected = false;
                    } else if self.mode != Mode::Spectate {
                        self.player_list
                            .spectators
                            .retain(|item| item.addr().map(|item| item != addr).unwrap_or(true));

                        for update_addr in self
                            .player_list
                            .current_players
                            .iter()
                            .filter_map(PlayerType::addr)
                        {
                            let _ = socket.send(Packet::reliable_ordered(
                                update_addr,
                                bincode::serialize(&NetPacket::RemoveSpectate(addr)).unwrap(),
                                None,
                            ));
                        }
                    }
                }
            }
        }

        while ggez::timer::check_update_time(ctx, 2) {}

        while let Some(event) = pads.next_event() {
            if let EventType::ButtonPressed(button) = event.event {
                if button == Button::Start
                    && self.connected
                    && self.mode == Mode::Host
                    && self
                        .player_list
                        .current_players
                        .p1()
                        .gamepad_id()
                        .map(|id| id == event.id)
                        .unwrap_or(false)
                {
                    for addr in self.player_list.network_addrs() {
                        let _ = socket.send(Packet::reliable_ordered(
                            addr,
                            bincode::serialize(&NetPacket::MoveToCharacterSelect).unwrap(),
                            None,
                        ));
                    }
                    self.next = Some(NextState::Next);
                }
            }
        }

        match std::mem::replace(&mut self.next, None) {
            Some(state) => match state {
                NextState::Next => Ok(Transition::Replace(Box::new(CharacterSelect::<
                    NetplayVersus,
                >::new(
                    self.player_list.clone(),
                    None,
                )))),
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
                    } else if ui.combo_items(
                        im_str!("Mode"),
                        &mut self.mode,
                        &Mode::iter().collect::<Vec<_>>(),
                        &|item| im_str!("{}", item).into(),
                    ) {
                        match self.mode {
                            Mode::Host => {
                                self.player_list.current_players =
                                    [self.local_player, PlayerType::Dummy].into()
                            }
                            Mode::Client => {
                                self.player_list.current_players =
                                    [PlayerType::Dummy, self.local_player].into()
                            }
                            Mode::Spectate => {
                                self.player_list.current_players = [PlayerType::Dummy; 2].into()
                            }
                        }
                    }

                    ui.text(im_str!(
                        "Current Address: {}",
                        socket
                            .local_addr()
                            .map(|item| item.to_string())
                            .unwrap_or_else(|_| "Error".to_owned())
                    ));

                    if self.mode != Mode::Spectate {
                        ui.text(im_str!(
                            "Spectators: {}",
                            self.player_list
                                .spectators
                                .iter()
                                .filter_map(PlayerType::addr)
                                .fold("".to_owned(), |acc, item| acc + &item.to_string()),
                        ));
                    } else {
                        ui.text(im_str!(
                            "Watching: {}, {}",
                            self.player_list
                                .current_players
                                .p1()
                                .addr()
                                .map(|item| item.to_string())
                                .unwrap_or_else(|| "None".to_owned()),
                            self.player_list
                                .current_players
                                .p2()
                                .addr()
                                .map(|item| item.to_string())
                                .unwrap_or_else(|| "None".to_owned()),
                        ));
                    }

                    if self.mode == Mode::Host || self.connected {
                        ui.text(&im_str!("IP: {}", self.target_addr));

                        if let PotentialAddress::Address(_) = self.target_addr {
                            if ui.small_button(im_str!("Disconnect")) {
                                self.connected = false;

                                for addr in self.player_list.network_addrs() {
                                    let _ = socket.send(Packet::reliable_sequenced(
                                        addr,
                                        bincode::serialize(&match self.mode {
                                            Mode::Spectate => NetPacket::CloseSpectate,
                                            Mode::Host | Mode::Client => NetPacket::Close,
                                        })
                                        .unwrap(),
                                        None,
                                    ));
                                }
                                self.player_list.spectators.clear();
                                match self.mode {
                                    Mode::Host => {
                                        self.player_list.current_players =
                                            [self.local_player, PlayerType::Dummy].into()
                                    }
                                    Mode::Client => {
                                        self.player_list.current_players =
                                            [PlayerType::Dummy, self.local_player].into()
                                    }
                                    Mode::Spectate => {
                                        self.player_list.current_players =
                                            [PlayerType::Dummy; 2].into()
                                    }
                                }
                            }
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
                                    bincode::serialize(&match self.mode {
                                        Mode::Client => NetPacket::RequestJoin,
                                        Mode::Spectate => NetPacket::RequestSpectate,
                                        Mode::Host => unreachable!(),
                                    })
                                    .unwrap(),
                                    None,
                                ));
                            }
                        }
                    }

                    if self.mode == Mode::Host && self.connected {
                        ui.text("Press start to move to character select.");
                    }
                });
            })
            .render(ctx);

        graphics::present(ctx)?;

        Ok(())
    }
}
