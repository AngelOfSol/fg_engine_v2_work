mod lobby;

use std::{collections::HashMap, net::SocketAddr};

use crate::{
    connection::{handle_incoming, ConnectionType},
    lobby_state,
    request::{Disconnected, JoinRequest, JoinResponse},
    util::{self},
    NetworkingAction, QuinnHandle,
};
use fg_netcode::{
    error::{HostLobbyError, JoinLobbyError},
    lobby::{lobby_state::LobbyState, Lobby},
    player_info::PlayerInfo,
    NetworkingMessage,
};
use futures_util::StreamExt;
use tokio::{select, sync::mpsc};

use self::lobby::LobbyBackend;

pub enum State {
    Disconnected(NetworkBackend),
    Lobby(NetworkBackend, LobbyBackend),
}

impl State {
    pub(crate) async fn main_loop(self, quinn: &mut QuinnHandle) -> Self {
        match self {
            State::Disconnected(mut backend) => {
                if let Some(lobby) = backend.main_loop(quinn).await {
                    Self::Lobby(backend, lobby)
                } else {
                    Self::Disconnected(backend)
                }
            }
            State::Lobby(mut backend, mut lobby) => {
                select! {
                    _ = backend.in_lobby() => (),
                    _ = lobby.main_loop(quinn) => (),
                }

                Self::Lobby(backend, lobby)
            }
        }
    }
}

pub struct NetworkBackend {
    pub messages: crossbeam_channel::Sender<NetworkingMessage>,
    pub actions: mpsc::Receiver<NetworkingAction>,
}

impl NetworkBackend {
    async fn in_lobby(&mut self) {
        select! {
            Some(action) = self.actions.recv() => {
                match action {
                    NetworkingAction::Host(_) => self
                        .messages
                        .send(NetworkingMessage::Host(Err(HostLobbyError::InLobby)))

                        .unwrap(),
                    NetworkingAction::ConnectTo(..) => self
                        .messages
                        .send(NetworkingMessage::Join(Err(JoinLobbyError::InLobby)))

                        .unwrap(),
                }
            },
            else => (),
        }
    }
    async fn main_loop(&mut self, quinn: &mut QuinnHandle) -> Option<LobbyBackend> {
        select! {
            Some(action) = self.actions.recv() => self.handle_action(action, quinn).await,
            _ = quinn.incoming.next() => None,
            else => None,
        }
    }

    async fn handle_action(
        &mut self,
        action: NetworkingAction,
        quinn: &mut QuinnHandle,
    ) -> Option<LobbyBackend> {
        match action {
            NetworkingAction::Host(info) => {
                let result = lobby_state::host(info);
                let (lobby_interface, backend) = LobbyBackend::new(result);

                self.messages
                    .send(NetworkingMessage::Host(Ok(lobby_interface)))
                    .unwrap();

                Some(backend)
            }
            NetworkingAction::ConnectTo(info, addr) => {
                if let Ok((interface, backend)) = self.try_connect(addr, info, quinn).await {
                    self.messages
                        .send(NetworkingMessage::Join(Ok(interface)))
                        .unwrap();
                    Some(backend)
                } else {
                    self.messages
                        .send(NetworkingMessage::Join(Err(JoinLobbyError::Denied)))
                        .unwrap();
                    None
                }
            }
        }
    }

    async fn try_connect(
        &mut self,
        addr: SocketAddr,
        info: PlayerInfo,
        quinn: &mut QuinnHandle,
    ) -> Result<(Lobby, LobbyBackend), Disconnected> {
        println!("pre-conn client for {}", info.name);
        let conn = quinn.endpoint.connect(&addr, "lobby_server")?.await?;
        println!("post-conn client for {}", info.name);

        let remote_addr = conn.connection.remote_address();

        let (send, recv) = conn.connection.open_bi().await?;

        util::write_to(
            &JoinRequest {
                target: remote_addr,
                info,
            },
            send,
        )
        .await?;

        let (this_addr, mut lobby_state) =
            util::read_from::<(SocketAddr, LobbyState)>(1000, recv).await?;

        let peer_id = lobby_state.host_id();

        lobby_state.user = lobby_state
            .player_list
            .pairs()
            .find(|(_, info)| info.addr == this_addr)
            .map(|(id, _)| *id)
            .ok_or(Disconnected)?;

        let result = lobby_state::join(lobby_state);

        let (lobby, mut lobby_backend) = LobbyBackend::new(result);

        lobby_backend.attach_peer(conn, peer_id, ConnectionType::PeerToHost);

        // TODO, attempt to connect p2p to all the other connections

        Ok((lobby, lobby_backend))
    }
}
