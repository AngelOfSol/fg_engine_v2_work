use super::{FromCharacters, LocalSelect, NetworkSelect};
use crate::app_state::{AppContext, AppState, Transition};
use crate::game_match::{Match, MatchSettings};
use crate::input::control_scheme::PadControlScheme;
use crate::input::pads_context::{Event, EventType, GamepadId};
use crate::input::InputState;
use crate::netcode::{NetcodeClient as Client, Packet as NetcodeClientPacket, PlayerHandle};
use ggez::{graphics, Context, GameResult};
use laminar::{Packet as SocketPacket, Socket, SocketEvent};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::time::Instant;

type NetplayMatch = Match<crate::replay::ReplayWriterFile>;

type NetcodeClient =
    Client<InputState, <NetplayMatch as crate::netcode::RollbackableGameState>::SavedState>;

enum NextState {
    Back,
}

pub struct NetplayVersus {
    next: Option<NextState>,
    local_players: Vec<(GamepadId, InputState, PlayerHandle)>,
    network_players: Vec<(SocketAddr, PlayerHandle, f32)>,

    socket: Socket,

    start_time: Instant,

    game_state: NetplayMatch,
    client: NetcodeClient,
}
impl FromCharacters<LocalSelect, LocalSelect> for NetplayVersus {
    fn from_characters(
        ctx: &mut Context,
        p1: LocalSelect,
        _: LocalSelect,
    ) -> GameResult<Box<Self>> {
        let mut client = NetcodeClient::new(60);

        let socket = Socket::bind_any()
            .map_err(|_| ggez::GameError::EventLoopError("Failed to bind socket.".to_owned()))?;

        let local_players = vec![
            (
                p1.gamepad,
                InputState::default(),
                client.add_local_player(0),
            ),
            (
                p1.gamepad,
                InputState::default(),
                client.add_local_player(1),
            ),
        ];

        Ok(Box::new(NetplayVersus::new(
            ctx,
            local_players,
            vec![],
            socket,
            client,
        )?))
    }
}
impl FromCharacters<LocalSelect, NetworkSelect> for NetplayVersus {
    fn from_characters(
        ctx: &mut Context,
        p1: LocalSelect,
        p2: NetworkSelect,
    ) -> GameResult<Box<Self>> {
        let mut client = NetcodeClient::new(60);

        let socket = p2.socket;

        let local_players = vec![(
            p1.gamepad,
            InputState::default(),
            client.add_local_player(0),
        )];
        let network_players = vec![(p2.target, client.add_network_player(1), 0.0)];

        Ok(Box::new(NetplayVersus::new(
            ctx,
            local_players,
            network_players,
            socket,
            client,
        )?))
    }
}
impl FromCharacters<NetworkSelect, LocalSelect> for NetplayVersus {
    fn from_characters(
        ctx: &mut Context,
        p1: NetworkSelect,
        p2: LocalSelect,
    ) -> GameResult<Box<Self>> {
        let mut client = NetcodeClient::new(60);

        let socket = p1.socket;

        let local_players = vec![(
            p2.gamepad,
            InputState::default(),
            client.add_local_player(1),
        )];
        let network_players = vec![(p1.target, client.add_network_player(0), 0.0)];

        Ok(Box::new(NetplayVersus::new(
            ctx,
            local_players,
            network_players,
            socket,
            client,
        )?))
    }
}

#[derive(Serialize, Deserialize)]
enum NetworkData {
    Client(NetcodeClientPacket<InputState>),
    Ping(u128),
    Pong(u128),
}

impl NetplayVersus {
    pub fn new(
        ctx: &mut Context,
        local_players: Vec<(GamepadId, InputState, PlayerHandle)>,
        network_players: Vec<(SocketAddr, PlayerHandle, f32)>,
        socket: Socket,
        mut client: NetcodeClient,
    ) -> GameResult<Self> {
        client.set_input_delay(3);
        client.set_allowed_rollback(10);
        client.set_packet_buffer_size(13);

        Ok(Self {
            next: None,
            local_players,
            network_players,
            game_state: NetplayMatch::new(
                ctx,
                MatchSettings::new().first_to(2).build(),
                crate::replay::create_new_replay_file("netplay")?,
            )?,
            client,
            socket,
            start_time: Instant::now(),
        })
    }
}

impl AppState for NetplayVersus {
    fn update(
        &mut self,
        ctx: &mut Context,
        &mut AppContext {
            ref mut pads,
            ref control_schemes,
            ref audio,
            ..
        }: &mut AppContext,
    ) -> GameResult<crate::app_state::Transition> {
        let mut events = Vec::new();
        while let Some(event) = pads.next_event() {
            events.push(event);
        }
        let events = events;

        // only iterates over the first player
        for (player, current_frame, _) in self.local_players.iter_mut() {
            let control_scheme = &control_schemes[player];
            for event in events.iter() {
                let Event { id, event, .. } = event;
                if *id == control_scheme.gamepad {
                    match event {
                        EventType::ButtonPressed(button) => {
                            control_scheme.handle_press(*button, current_frame);
                        }
                        EventType::ButtonReleased(button) => {
                            control_scheme.handle_release(*button, current_frame);
                        }
                    }
                }
            }
        }

        self.socket.manual_poll(Instant::now());
        while let Some(event) = self.socket.recv() {
            match event {
                SocketEvent::Packet(packet) => {
                    match bincode::deserialize(packet.payload()).unwrap() {
                        NetworkData::Client(client_packet) => {
                            if let Some(response) = self.client.handle_packet(client_packet) {
                                let _ = self.socket.send(SocketPacket::unreliable(
                                    packet.addr(),
                                    bincode::serialize(&NetworkData::Client(response)).unwrap(),
                                ));
                            }
                        }
                        NetworkData::Ping(ping_time) => {
                            let _ = self.socket.send(SocketPacket::unreliable_sequenced(
                                packet.addr(),
                                bincode::serialize(&NetworkData::Pong(ping_time)).unwrap(),
                                Some(2),
                            ));
                        }
                        NetworkData::Pong(pong_time) => {
                            let ping_time =
                                (Instant::now() - self.start_time).as_millis() - pong_time;
                            for (_, handle, ref mut ping) in self
                                .network_players
                                .iter_mut()
                                .filter(|(addr, _, _)| *addr == packet.addr())
                            {
                                *ping = *ping * 0.5 + (ping_time as f32 / 2.0) * 0.5;
                                self.client
                                    .set_network_delay(((*ping) / 16.0).ceil() as usize, *handle);
                            }
                        }
                    }
                }
                SocketEvent::Timeout(timed_out_addr) => {
                    if self
                        .network_players
                        .iter()
                        .any(|(addr, _, _)| *addr == timed_out_addr)
                    {
                        self.next = Some(NextState::Back);
                    }
                }
                SocketEvent::Connect(_) => {}
            }
        }

        while ggez::timer::check_update_time(ctx, 60) {
            for (player, ref mut current_frame, handle) in self.local_players.iter_mut() {
                let output = self
                    .client
                    .handle_local_input(current_frame.clone(), *handle);
                if let Some(output) = output {
                    let output = NetworkData::Client(output);
                    for (addr, _, _) in self.network_players.iter() {
                        let _ = self.socket.send(SocketPacket::unreliable(
                            *addr,
                            bincode::serialize(&output).unwrap(),
                        ));
                    }
                }
                let control_scheme = &control_schemes[player];
                control_scheme.update_frame(current_frame);
            }

            let time = (Instant::now() - self.start_time).as_millis();
            let ping_packet = NetworkData::Ping(time);
            for (addr, _, _) in self.network_players.iter() {
                let _ = self.socket.send(SocketPacket::unreliable_sequenced(
                    *addr,
                    bincode::serialize(&ping_packet).unwrap(),
                    Some(1),
                ));
            }

            self.client.update(&mut self.game_state);
            if self.game_state.game_over().is_some() {
                self.next = Some(NextState::Back);
            }
            self.game_state.render_sounds(60, audio)?;
        }

        match std::mem::replace(&mut self.next, None) {
            Some(state) => match state {
                NextState::Back => Ok(Transition::Pop),
            },
            None => Ok(Transition::None),
        }
    }
    fn on_enter(
        &mut self,
        _: &mut Context,
        &mut AppContext {
            ref mut control_schemes,
            ..
        }: &mut AppContext,
    ) -> GameResult<()> {
        for (player, _, _) in self.local_players.iter() {
            control_schemes
                .entry(*player)
                .or_insert(PadControlScheme::new(*player));
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context, AppContext { .. }: &mut AppContext) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        self.game_state.draw(ctx)?;

        graphics::present(ctx)?;

        Ok(())
    }
}
