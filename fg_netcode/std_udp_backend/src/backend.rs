mod lobby;

use std::net::SocketAddr;

use crate::{
    connection::{handle_incoming, ConnectionType},
    lobby_state,
    request::{Disconnected, JoinRequest, JoinResponse},
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
    pub messages: async_channel::Sender<NetworkingMessage>,
    pub actions: mpsc::Receiver<NetworkingAction>,
    pub handle: tokio::runtime::Handle,
}

impl NetworkBackend {
    async fn in_lobby(&mut self) {
        select! {
            Some(action) = self.actions.recv() => {
                match action {
                    NetworkingAction::Host(_) => self
                        .messages
                        .send(NetworkingMessage::Host(Err(HostLobbyError::InLobby)))
                        .await
                        .unwrap(),
                    NetworkingAction::ConnectTo(..) => self
                        .messages
                        .send(NetworkingMessage::Join(Err(JoinLobbyError::InLobby)))
                        .await
                        .unwrap(),
                }
            },
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
                let (task, lsi) = lobby_state::host(info);
                let (lobby_interface, backend) = LobbyBackend::new(task, lsi, vec![]);

                self.messages
                    .send(NetworkingMessage::Host(Ok(lobby_interface)))
                    .await
                    .unwrap();

                Some(backend)
            }
            NetworkingAction::ConnectTo(info, addr) => {
                if let Ok((interface, backend)) = self.try_connect(addr, info, quinn).await {
                    self.messages
                        .send(NetworkingMessage::Join(Ok(interface)))
                        .await
                        .unwrap();
                    Some(backend)
                } else {
                    self.messages
                        .send(NetworkingMessage::Join(Err(JoinLobbyError::Denied)))
                        .await
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
        let mut conn = quinn
            .endpoint
            .connect(&addr, "lobby_server")
            .unwrap()
            .await
            .unwrap();
        let remote_addr = conn.connection.remote_address();

        let (mut join_request, join_response) = conn.connection.open_bi().await?;
        join_request
            .write_all(
                &bincode::serialize(&JoinRequest {
                    target: remote_addr,
                    info,
                })
                .unwrap(),
            )
            .await?;
        join_request.finish().await?;

        let response = join_response.read_to_end(1000).await?;
        let _ = bincode::deserialize::<JoinResponse>(&response).map_err(|_| Disconnected)?;

        let lobby_stream = conn.uni_streams.next().await.ok_or(Disconnected)??;
        let lobby_data = lobby_stream.read_to_end(1000).await?;
        let lobby_data =
            bincode::deserialize::<LobbyState>(&lobby_data).map_err(|_| Disconnected)?;
        let peer_id = lobby_data.host_id();

        let (lsi_task, lsi) = lobby_state::join(lobby_data);

        let conn_task = handle_incoming(conn, peer_id, lsi.clone(), ConnectionType::PeerToHost);

        Ok(LobbyBackend::new(lsi_task, lsi, vec![conn_task]))
    }
}
