use crate::app_state::{AppContext, AppState, Transition};
use crate::imgui_extra::UiExtensions;
use crate::input::pads_context::{Button, EventType, GamepadId};
use ggez::{graphics, Context, GameResult};
use imgui::im_str;
use laminar::{Config, Packet, Socket, SocketEvent};
use std::fmt::Display;
use std::net::{SocketAddr, ToSocketAddrs};
use std::time::{Duration, Instant};

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
    socket: Socket,
    mode: Mode,
    target_addr: PotentialAddress,
    connected: bool,
    next_state: Box<dyn FnOnce(bool, GamepadId, Socket, SocketAddr) -> Transition>,
}

impl NetworkConnect {
    pub fn new(
        next_state: Box<dyn FnOnce(bool, GamepadId, Socket, SocketAddr) -> Transition>,
    ) -> GameResult<Self> {
        let adapter = ipconfig::get_adapters()
            .unwrap()
            .into_iter()
            .find(|x| x.friendly_name() == "Ethernet");
        Ok(Self {
            // add connected field
            // when connected is true wait for a gaempad to press start
            // and move urself to character select with that pad as the local player
            // and the networked player as the socket
            next: None,
            socket: Socket::bind_with_config(
                adapter
                    .and_then(|adapter| {
                        adapter
                            .ip_addresses()
                            .iter()
                            .find(|item| item.is_ipv4())
                            .cloned()
                    })
                    .map(|ip| vec![ip.to_string() + ":10800", ip.to_string() + ":10801"])
                    .unwrap_or(vec!["127.0.0.1:10800".to_owned()])
                    .into_iter()
                    .flat_map(|item| item.to_socket_addrs().ok())
                    .flatten()
                    .collect::<Vec<SocketAddr>>()
                    .as_slice(),
                Config {
                    blocking_mode: false,
                    rtt_max_value: 2000,
                    idle_connection_timeout: Duration::from_secs(5),
                    heartbeat_interval: Some(Duration::from_secs(1)),
                    ..Config::default()
                },
            )
            .map_err(|_| {
                ggez::GameError::EventLoopError("Could not connect to socket.".to_owned())
            })?,
            connected: false,
            target_addr: PotentialAddress::Almost(String::with_capacity(30)),
            mode: Mode::Host,
            next_state,
        })
    }
}

impl AppState for NetworkConnect {
    fn update(
        &mut self,
        ctx: &mut Context,
        AppContext { ref mut pads, .. }: &mut AppContext,
    ) -> GameResult<crate::app_state::Transition> {
        self.socket.manual_poll(Instant::now());

        while let Some(packet) = self.socket.recv() {
            match packet {
                SocketEvent::Packet(_) => self.connected = true,
                SocketEvent::Connect(addr) => {
                    if self.mode == Mode::Host {
                        self.target_addr = PotentialAddress::Address(addr);
                    }
                    self.connected = true;
                    self.socket
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
                NextState::Next(id) => {
                    let next_state = std::mem::replace(
                        &mut self.next_state,
                        Box::new(|_, _, _, _| Transition::Pop),
                    );
                    let socket = std::mem::replace(&mut self.socket, Socket::bind_any().unwrap());
                    Ok(next_state(
                        self.mode == Mode::Host,
                        id,
                        socket,
                        match self.target_addr {
                            PotentialAddress::Address(addr) => addr,
                            _ => unreachable!(),
                        },
                    ))
                }
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
        AppContext { ref mut imgui, .. }: &mut AppContext,
    ) -> GameResult<()> {
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
                        self.socket
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
                            let _ = dbg!(self
                                .socket
                                .send(Packet::reliable_sequenced(addr, vec![], None))
                                .map_err(|_| {
                                    ggez::GameError::EventLoopError(
                                        "Could not send packet".to_owned(),
                                    )
                                }));
                        }
                    }
                });
            })
            .render(ctx);

        graphics::present(ctx)?;

        Ok(())
    }
}
