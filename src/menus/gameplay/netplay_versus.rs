use crate::app_state::{AppContext, AppState, Transition};
use crate::game_match::{FromMatchSettings, Match, MatchSettings};
use crate::input::control_scheme::PadControlScheme;
use crate::input::pads_context::{Event, EventType};
use crate::input::InputState;
use crate::netcode::{NetcodeClient as Client, Packet as NetcodeClientPacket, PlayerHandle};
use ggez::{graphics, Context, GameResult};
use laminar::{Packet as SocketPacket, SocketEvent};
use serde::{Deserialize, Serialize};
use std::time::Instant;

use super::retry_screen::RetryScreen;
use crate::player_list::{PlayerList, PlayerType};
use std::collections::HashMap;

type NetplayMatch = Match<crate::replay::ReplayWriterFile>;

type NetcodeClient =
    Client<InputState, <NetplayMatch as crate::netcode::RollbackableGameState>::SavedState>;

enum NextState {
    Back,
}

pub struct NetplayVersus {
    next: Option<NextState>,
    player_list: PlayerList,

    pings: HashMap<PlayerHandle, f32>,
    local_input: HashMap<PlayerHandle, InputState>,

    start_time: Instant,

    game_state: NetplayMatch,
    client: NetcodeClient,
}
impl FromMatchSettings for NetplayVersus {
    fn from_settings(
        ctx: &mut Context,
        player_list: PlayerList,
        settings: MatchSettings,
    ) -> GameResult<Box<Self>> {
        let mut client = NetcodeClient::new(60);

        for (idx, player) in player_list.current_players.iter().enumerate() {
            if player.is_local() {
                client.add_local_player(idx);
            } else if player.is_networked() {
                client.add_network_player(idx);
            }
        }

        Ok(Box::new(NetplayVersus::new(
            ctx,
            player_list,
            settings,
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
        player_list: PlayerList,
        settings: MatchSettings,
        mut client: NetcodeClient,
    ) -> GameResult<Self> {
        client.set_input_delay(3);
        client.set_allowed_rollback(10);
        client.set_packet_buffer_size(13);

        Ok(Self {
            next: None,

            pings: player_list
                .current_players
                .iter()
                .enumerate()
                .filter_map(|(idx, player)| {
                    if player.is_networked() {
                        Some((idx, 0.0))
                    } else {
                        None
                    }
                })
                .collect(),

            local_input: player_list
                .current_players
                .iter()
                .enumerate()
                .filter_map(|(idx, player)| {
                    if player.is_local() {
                        Some((idx, InputState::default()))
                    } else {
                        None
                    }
                })
                .collect(),

            game_state: NetplayMatch::new(
                ctx,
                settings,
                crate::replay::create_new_replay_file("netplay")?,
            )?,
            client,
            player_list,
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
            ref mut socket,
            ..
        }: &mut AppContext,
    ) -> GameResult<crate::app_state::Transition> {
        let mut events = Vec::new();
        while let Some(event) = pads.next_event() {
            events.push(event);
        }
        let events = events;

        for (player, current_frame) in self.local_input.iter_mut() {
            if let Some(gamepad) = self.player_list.current_players[*player].gamepad_id() {
                let control_scheme = &control_schemes[&gamepad];
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
        }

        if let Some(ref mut socket) = socket {
            socket.manual_poll(Instant::now());
            while let Some(event) = socket.recv() {
                match event {
                    SocketEvent::Packet(packet) => {
                        match bincode::deserialize(packet.payload()).unwrap() {
                            NetworkData::Client(client_packet) => {
                                if let Some(response) = self.client.handle_packet(client_packet) {
                                    let _ = socket.send(SocketPacket::unreliable(
                                        packet.addr(),
                                        bincode::serialize(&NetworkData::Client(response)).unwrap(),
                                    ));
                                }
                            }
                            NetworkData::Ping(ping_time) => {
                                let _ = socket.send(SocketPacket::unreliable_sequenced(
                                    packet.addr(),
                                    bincode::serialize(&NetworkData::Pong(ping_time)).unwrap(),
                                    Some(2),
                                ));
                            }
                            NetworkData::Pong(pong_time) => {
                                let ping_time =
                                    (Instant::now() - self.start_time).as_millis() - pong_time;

                                let player = self
                                    .player_list
                                    .current_players
                                    .iter()
                                    .position(|item| item == &packet.addr().into());

                                if let Some(player) = player {
                                    *self.pings.get_mut(&player).unwrap() =
                                        self.pings[&player] * 0.5 + (ping_time as f32 / 2.0) * 0.5;
                                    self.client.set_network_delay(
                                        ((self.pings[&player]) / 16.0).ceil() as usize,
                                        player,
                                    );
                                }
                            }
                        }
                    }
                    SocketEvent::Timeout(timed_out_addr) => {
                        if self
                            .player_list
                            .current_players
                            .iter()
                            .any(|item| item == &timed_out_addr.into())
                        {
                            self.next = Some(NextState::Back);
                        }
                    }
                    SocketEvent::Connect(_) => {}
                }
            }
        }

        while ggez::timer::check_update_time(ctx, 60) {
            for (handle, current_frame) in self.local_input.iter_mut() {
                let output = self
                    .client
                    .handle_local_input(current_frame.clone(), *handle);
                if let Some(ref mut socket) = socket {
                    if let Some(output) = output {
                        let output = NetworkData::Client(output);
                        for addr in self.player_list.network_addrs() {
                            let _ = socket.send(SocketPacket::unreliable(
                                addr,
                                bincode::serialize(&output).unwrap(),
                            ));
                        }
                    }
                }

                let control_scheme = &control_schemes[&self.player_list.current_players[*handle]
                    .gamepad_id()
                    .unwrap()];
                control_scheme.update_frame(current_frame);
            }

            if let Some(ref mut socket) = socket {
                let time = (Instant::now() - self.start_time).as_millis();
                let ping_packet = NetworkData::Ping(time);
                for addr in self
                    .player_list
                    .current_players
                    .iter()
                    .filter_map(PlayerType::addr)
                {
                    let _ = socket.send(SocketPacket::unreliable_sequenced(
                        addr,
                        bincode::serialize(&ping_packet).unwrap(),
                        Some(1),
                    ));
                }
            }

            self.client.update(&mut self.game_state);
            if self.game_state.game_over().is_some() {
                self.next = Some(NextState::Back);
            }
            self.game_state.render_sounds(60, audio)?;
        }

        match std::mem::replace(&mut self.next, None) {
            Some(state) => match state {
                NextState::Back => Ok(Transition::Replace(Box::new(
                    RetryScreen::<NetplayVersus>::new(
                        self.player_list.clone(),
                        self.game_state.settings.clone(),
                    ),
                ))),
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
        for player in self
            .player_list
            .current_players
            .iter()
            .filter_map(PlayerType::gamepad_id)
        {
            control_schemes
                .entry(player)
                .or_insert(PadControlScheme::new(player));
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
