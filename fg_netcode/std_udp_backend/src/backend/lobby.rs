use std::{borrow::Borrow, collections::HashMap, ops::Deref};

use crate::{
    connection::{handle_incoming, ConnectionType, Peer},
    lobby_state::{LobbyStateAction, LobbyTaskResult},
    request::{Disconnected, JoinRequest, JoinResponse},
    util, QuinnHandle,
};
use fg_netcode::{
    lobby::{lobby_state::LobbyState, Lobby, LobbyAction, LobbyMessage},
    player_info::PlayerInfo,
    player_list::Player,
};
use futures_util::StreamExt;
use quinn::{Connecting, NewConnection};
use tokio::{
    select,
    sync::{broadcast, mpsc, oneshot, watch},
};

pub struct LobbyBackend {
    pub from_frontend: mpsc::Receiver<LobbyAction>,
    pub from_network: mpsc::Receiver<LobbyStateAction>,

    pub to_self: mpsc::Sender<LobbyStateAction>,

    pub to_network: broadcast::Sender<LobbyStateAction>,
    pub to_local: mpsc::Sender<LobbyStateAction>,

    pub lobby_state: watch::Receiver<LobbyState>,

    pub connection_list: HashMap<Player, Peer>,
}

impl LobbyBackend {
    pub fn new(interface: LobbyTaskResult) -> (Lobby, Self) {
        let (to_backend, from_frontend) = mpsc::channel(4);

        let lobby_interface = Lobby::new(interface.state.clone(), interface.recv, to_backend);

        let (to_network, _) = broadcast::channel(4);
        let (to_self, from_network) = mpsc::channel(4);

        (
            lobby_interface,
            LobbyBackend {
                connection_list: HashMap::new(),
                from_frontend,
                from_network,
                to_self,
                to_network,
                to_local: interface.actions,
                lobby_state: interface.state,
            },
        )
    }

    pub fn attach_peer(
        &mut self,
        conn: NewConnection,
        peer_id: Player,
        connection_type: ConnectionType,
    ) {
        self.connection_list.insert(
            peer_id,
            handle_incoming(
                conn,
                peer_id,
                self.lobby_state.clone(),
                connection_type,
                self.to_self.clone(),
                self.to_network.subscribe(),
            ),
        );
    }

    pub fn host_peer(&self) -> &Peer {
        let host = { self.lobby_state.borrow().host_id() };
        &self.connection_list[&host]
    }

    async fn in_game(&mut self, quinn: &mut QuinnHandle) {
        // select! {
        //     Some(action) = self.local_actions.recv() => self.handle_action_in_game(action).await,
        //     Some(incoming) = quinn.incoming.next() => self.handle_incoming(incoming).await.unwrap(),
        //     _ = self.lobby_state.changed() => (),
        // }
    }

    async fn handle_incoming(&mut self, incoming: Connecting) -> Result<(), Disconnected> {
        println!("pre incoming: {}", incoming.remote_address());
        let mut conn = incoming.await?;
        let remote_addr = conn.connection.remote_address();
        println!("post incoming: {}", remote_addr);
        let lobby_state = { self.lobby_state.borrow().clone() };

        if lobby_state.is_user_host() {
            let (join_response, join_request) =
                conn.bi_streams.next().await.ok_or(Disconnected)??;

            let request = util::read_from::<JoinRequest>(1000, join_request).await?;

            self.to_local
                .send(LobbyStateAction::UpdateAddr(
                    lobby_state.user,
                    request.target,
                ))
                .await
                .unwrap();

            self.lobby_state.changed().await.unwrap();

            let _ = self.to_network.send(LobbyStateAction::UpdateAddr(
                lobby_state.user,
                request.target,
            ));

            let info = PlayerInfo {
                addr: remote_addr,
                ..request.info
            };

            self.to_local
                .send(LobbyStateAction::NewPlayer(info.clone()))
                .await
                .unwrap();
            self.lobby_state.changed().await.unwrap();
            let _ = self.to_network.send(LobbyStateAction::NewPlayer(info));

            let lobby_state = { self.lobby_state.borrow().clone() };

            util::write_to(&(remote_addr, lobby_state), join_response).await?;
        }

        let lobby_state = { self.lobby_state.borrow().clone() };
        let peer_id = lobby_state
            .player_list
            .pairs()
            .find(|(_, info)| info.addr == conn.connection.remote_address())
            .map(|(id, _)| *id)
            .ok_or(Disconnected)?;

        self.attach_peer(
            conn,
            peer_id,
            ConnectionType::from_peer(peer_id, &lobby_state),
        );

        Ok(())
    }

    async fn handle_incoming_packet(
        &mut self,
        incoming: LobbyStateAction,
    ) -> Result<(), Disconnected> {
        let is_host = { self.lobby_state.borrow().is_user_host() };

        println!("is_host: {}, handling: {:?}", is_host, incoming);

        self.to_local.send(incoming.clone()).await.unwrap();
        if is_host {
            let _ = self.to_network.send(incoming);
        }

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
        dbg!("main");
        select! {
            Some(incoming) = quinn.incoming.next() => self.handle_incoming(incoming).await.ok(),
            // Some(action) = self.from_frontend.recv() => self.handle_action(action, None).await,
            Some(incoming) = self.from_network.recv() => self.handle_incoming_packet(incoming).await.ok(),
            else => None,
        };
    }

    async fn handle_action(&mut self, action: LobbyAction, user: Option<Player>) -> Option<()> {
        // let is_host = self.lobby_state.borrow().is_user_host();
        // let user = user.unwrap_or_else(|| self.lobby_state.borrow().user);

        match action {
            LobbyAction::CreateGame => {
                // let (send, recv) = oneshot::channel();

                // self.lobby_state
                //     .actions
                //     .send(LobbyStateAction::CreateGame(user, send))
                //     .await
                //     .ok()?;

                // if is_host {
                //     self.messages
                //         .send(LobbyMessage::CreateGame(recv.await.ok()?))
                //         .ok()?;
                // } else {
                //     send to
                // }
            }
            LobbyAction::JoinGame(game) => {
                // let (send, recv) = oneshot::channel();

                // self.lobby_state
                //     .actions
                //     .send(LobbyStateAction::JoinGame(game, user, send))
                //     .await
                //     .ok()?;

                // if is_host {
                //     self.messages
                //         .send(LobbyMessage::JoinGame(recv.await.ok()?))
                //         .ok()?;
                // }
            }
            LobbyAction::UpdatePlayerInfo(info) => {
                // self.lobby_state
                //     .actions
                //     .send(LobbyStateAction::UpdatePlayer(user, info.clone()))
                //     .await
                //     .ok()?;

                // if !is_host {
                //     self.host_peer()
                //         .lobby_action
                //         .send(LobbyAction::UpdatePlayerInfo(info))
                //         .await
                //         .unwrap();
                // }
            }
        }

        None
    }
}
