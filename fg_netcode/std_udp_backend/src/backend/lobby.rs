use crate::{
    connection::{handle_incoming, ConnectionType},
    lobby_state::{LobbyStateAction, LobbyStateInterface},
    request::{Disconnected, JoinRequest, JoinResponse},
    util, QuinnHandle,
};
use fg_netcode::{
    lobby::{lobby_state::LobbyState, Lobby, LobbyAction, LobbyMessage},
    player_info::PlayerInfo,
};
use futures_util::StreamExt;
use quinn::Connecting;
use tokio::{
    select,
    sync::{mpsc, oneshot},
};

pub struct LobbyBackend {
    pub messages: crossbeam_channel::Sender<LobbyMessage>,
    pub connection_message: (mpsc::Sender<()>, mpsc::Receiver<()>),
    pub actions: mpsc::Receiver<LobbyAction>,
    pub lobby_state: LobbyStateInterface,
}

impl LobbyBackend {
    pub fn new(interface: LobbyStateInterface) -> (Lobby, Self) {
        let (message_tx, message_rx) = crossbeam_channel::bounded(4);
        let (action_tx, action_rx) = mpsc::channel(4);

        let lobby_interface = Lobby::new(interface.state.clone(), message_rx, action_tx);

        (
            lobby_interface,
            LobbyBackend {
                messages: message_tx,
                actions: action_rx,
                connection_message: mpsc::channel(4),
                lobby_state: interface,
            },
        )
    }

    async fn in_game(&mut self, quinn: &mut QuinnHandle) {
        select! {
            Some(action) = self.actions.recv() => self.handle_action_in_game(action).await,
            Some(incoming) = quinn.incoming.next() => self.handle_incoming(incoming).await.unwrap(),
            _ = self.lobby_state.state.changed() => (),
        }
    }

    async fn handle_incoming(&mut self, incoming: Connecting) -> Result<(), Disconnected> {
        let mut conn = incoming.await?;
        let remote_addr = conn.connection.remote_address();
        let is_host = self.lobby_state.state.borrow().is_user_host();
        let actions = self.lobby_state.actions.clone();

        if is_host {
            let (join_response, join_request) =
                conn.bi_streams.next().await.ok_or(Disconnected)??;

            let request = util::read_from::<JoinRequest>(1000, join_request).await?;

            actions
                .send(LobbyStateAction::UpdateAddr(request.target))
                .await
                .unwrap();
            self.lobby_state.state.changed().await.unwrap();

            util::write_to(
                &JoinResponse {
                    target: remote_addr,
                },
                join_response,
            )
            .await?;

            let info = PlayerInfo {
                addr: remote_addr,
                ..request.info
            };

            actions
                .send(LobbyStateAction::NewPlayer(info))
                .await
                .unwrap();
            self.lobby_state.state.changed().await.unwrap();
        }

        let lobby_state = self.lobby_state.state.borrow().clone();
        let peer_id = lobby_state
            .player_list
            .pairs()
            .find(|(_, info)| info.addr == conn.connection.remote_address())
            .map(|(id, _)| *id)
            .ok_or(Disconnected)?;
        let connection_type = ConnectionType::from_peer(peer_id, &lobby_state);

        util::write_to(
            &LobbyState {
                user: peer_id,
                ..lobby_state
            },
            conn.connection.open_uni().await?,
        )
        .await?;

        handle_incoming(conn, peer_id, self.lobby_state.clone(), connection_type);

        Ok(())
    }

    async fn handle_action_in_game(&mut self, action: LobbyAction) {
        match action {
            LobbyAction::CreateGame => {}
            LobbyAction::JoinGame(_) => {}
            LobbyAction::UpdatePlayerInfo(_) => {}
        }
    }

    pub(crate) async fn main_loop(&mut self, quinn: &mut QuinnHandle) {
        select! {
            Some(incoming) = quinn.incoming.next() => self.handle_incoming(incoming).await.ok(),
            Some(action) = self.actions.recv() => self.handle_action(action).await,
            else => None,
        };
    }

    async fn handle_action(&mut self, action: LobbyAction) -> Option<()> {
        let is_host = self.lobby_state.state.borrow().is_user_host();
        let user = self.lobby_state.state.borrow().user;

        if is_host {
            match action {
                LobbyAction::CreateGame => {
                    let (send, recv) = oneshot::channel();

                    self.lobby_state
                        .actions
                        .send(LobbyStateAction::CreateGame(user, send))
                        .await
                        .ok()?;

                    self.messages
                        .send(LobbyMessage::CreateGame(recv.await.ok()?))
                        .ok()?;
                }
                LobbyAction::JoinGame(game) => {
                    let (send, recv) = oneshot::channel();

                    self.lobby_state
                        .actions
                        .send(LobbyStateAction::JoinGame(game, user, send))
                        .await
                        .ok()?;

                    self.messages
                        .send(LobbyMessage::JoinGame(recv.await.ok()?))
                        .ok()?;
                }
                LobbyAction::UpdatePlayerInfo(info) => {
                    self.lobby_state
                        .actions
                        .send(LobbyStateAction::UpdatePlayer(user, info))
                        .await
                        .ok()?;
                }
            }
        }

        None
    }
}
