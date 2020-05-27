use super::netplay_versus::NetplayVersus;
use super::CharacterSelect;
use crate::app_state::{AppContext, AppState, Transition};
use crate::imgui_extra::UiExtensions;
use crate::input::pads_context::{Button, EventType, GamepadId};
use crate::player_list::PlayerList;
use ggez::{graphics, Context, GameResult};
use imgui::im_str;
use laminar::{Packet, SocketEvent};
use std::fmt::Display;
use std::net::SocketAddr;
use std::time::Instant;

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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Mode {
    Host,
    Client,
}

pub struct NetworkConnect {
    next: Option<NextState>,
    mode: Mode,
    target_addr: PotentialAddress,
    connected: bool,
}

impl NetworkConnect {
    pub fn new() -> GameResult<Self> {
        Ok(Self {
            next: None,
            connected: false,
            target_addr: PotentialAddress::Almost(String::with_capacity(30)),
            mode: Mode::Host,
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
                    ui.combo_items(
                        im_str!("Mode"),
                        &mut self.mode,
                        &[Mode::Host, Mode::Client],
                        &|item| match item {
                            Mode::Host => im_str!("Host").into(),
                            Mode::Client => im_str!("Client").into(),
                        },
                    );

                    ui.text(im_str!(
                        "Current Address: {}",
                        socket
                            .local_addr()
                            .map(|item| item.to_string())
                            .unwrap_or("Error".to_owned())
                    ));

                    match self.mode {
                        Mode::Host => {
                            ui.text(&im_str!("IP: {}", self.target_addr));
                        }
                        Mode::Client => {
                            let mut buffer = self.target_addr.to_string();

                            if ui.input_string(im_str!("IP"), &mut buffer) {
                                self.target_addr = match buffer.parse() {
                                    Ok(addr) => PotentialAddress::Address(addr),
                                    Err(_) => PotentialAddress::Almost(buffer),
                                };
                            }
                        }
                    }

                    if self.connected {
                        ui.text("Press start on the controller you want to use to continue.");
                    } else if let PotentialAddress::Address(addr) = self.target_addr {
                        if ui.small_button(im_str!("Try to connect!")) {
                            let _ = socket
                                .send(Packet::reliable_sequenced(addr, vec![], None))
                                .map_err(|_| {
                                    ggez::GameError::EventLoopError(
                                        "Could not send packet".to_owned(),
                                    )
                                });
                        }
                    }
                });
            })
            .render(ctx);

        graphics::present(ctx)?;

        Ok(())
    }
}
