pub mod error;
pub mod lobby;
pub mod player_info;

use std::{net::SocketAddr, sync::mpsc::TryRecvError, thread, time::Duration};

use error::{HostLobbyError, JoinLobbyError};
use lobby::{GameInfo, Lobby, LobbyState};
use player_info::PlayerInfo;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};

pub struct Networking {
    rx: Receiver<NetworkingMessage>,
    tx: SyncSender<NetworkingMessage>,
}

pub enum NetworkingMessage {
    Host(Result<Lobby, HostLobbyError>),
    Join(Result<Lobby, JoinLobbyError>),
}

impl Default for Networking {
    fn default() -> Self {
        let (tx, rx) = sync_channel(4);
        Self { tx, rx }
    }
}
impl Networking {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn request_host(&mut self, player: PlayerInfo) {
        let tx = self.tx.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(2));
            tx.send(NetworkingMessage::Host(Ok(Lobby::new(LobbyState {
                player_list: vec![player],
                user: 0,
                games: vec![],
            }))))
            .unwrap();
        });
    }
    pub fn request_join(&mut self, _id: SocketAddr, player: PlayerInfo) {
        let tx = self.tx.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(2));
            tx.send(NetworkingMessage::Host(Ok(Lobby::new(LobbyState {
                player_list: vec![PlayerInfo::default(), player],
                user: 1,
                games: vec![GameInfo {
                    player_list: vec![0],
                }],
            }))))
            .unwrap();
        });
    }

    pub fn poll(&mut self) -> Option<NetworkingMessage> {
        match self.rx.try_recv() {
            Ok(value) => Some(value),
            Err(TryRecvError::Disconnected) => panic!("Backing network was disconnected."),
            Err(TryRecvError::Empty) => None,
        }
    }
}

#[cfg(feature = "never")]
mod traits {
    use crate::error::{
        CreateGameError, HostLobbyError, JoinGameError, JoinLobbyError, NetworkError,
        SpectateGameError, UpdateMetaError,
    };
    use async_trait::async_trait;

    /// You can join/host a lobby via an ID, and once you're a part of a lobby,
    /// you get a "handle" that represents the lobby and allows you to access its state.
    /// Additionally you can ask to access the list of available lobbies, but not
    /// every implementation will return any information here.
    #[async_trait]
    pub trait NetworkingSubsytem {
        type LobbyId;
        type Lobby: Lobby;

        async fn request_host(&mut self) -> Result<Self::Lobby, HostLobbyError>;
        async fn request_join(
            &mut self,
            lobby: Self::LobbyId,
        ) -> Result<Self::Lobby, JoinLobbyError>;
        async fn request_lobby_list(&self) -> Vec<Self::LobbyId>;
    }

    /// A lobby handle, that provides functionality for manipulating lobby state.
    /// Allows you to view available games, check the player list for player's who
    /// aren't a game, and enter or leave games.
    ///
    /// When all copies of a handle are dropped or consumed via leave, the lobby state should
    /// reflect that player leaving.
    #[async_trait]
    pub trait Lobby: Clone {
        type Playing: Playing<Meta = Self::Meta>;
        type Spectating: Spectating<Meta = Self::Meta>;
        type Meta: Meta;
        type PlayerInfo;

        fn game_list(&self) -> &[GameInfo<'_, Self::PlayerInfo, Self::Meta>];
        fn idle_players(&self) -> &[Self::PlayerInfo];

        async fn create_game(&self) -> Result<Self::Playing, CreateGameError>;
        async fn join_game(&self, id: usize) -> Result<Self::Playing, JoinGameError>;
        async fn spectate_game(&self, id: usize) -> Result<Self::Spectating, SpectateGameError>;
        async fn leave(self) -> Result<(), NetworkError>;
    }

    pub trait Meta {
        fn midgame(&self) -> bool;
    }

    pub struct GameInfo<'a, PlayerInfo, Meta> {
        pub players: &'a [PlayerInfo],
        pub spectators: &'a [PlayerInfo],
        pub meta: &'a Meta,
    }

    /// A handle to a game where the user is spectating.  They're given read-only access
    /// to the game's settings, and are not allowed to send packets to to other players
    /// in the same game, but will receive them unreliabley.
    ///
    /// When the handle is dropped or consumed via stop, the lobby state should
    /// reflect that player leaving the game.
    #[async_trait]
    pub trait Spectating: Clone {
        type Meta;

        /// Returns a usize representing which player sent the data.
        async fn recv_raw(&self, data: &mut [u8]) -> Result<usize, NetworkError>;
        async fn stop(self) -> Result<(), NetworkError>;
        fn meta(&self) -> &Self::Meta;
    }

    /// A handle to a game where the user is actively playing.  They're given read/write access
    /// to the game's settings, and are allowed to send packets to the other players and spectators
    /// in the same game.
    ///
    /// When the handle is dropped or consumed via stop, the lobby state should
    /// reflect that player leaving the game.
    #[async_trait]
    pub trait Playing: Clone {
        type Meta;
        /// Provides a way to send game updates unreliably.  Should return an error
        /// if the other player who is playing disconnects, and errors from spectators
        /// should be ignored.
        ///
        /// Assumes that only 2 players can be playing, so there is no ID
        /// specified to indicate who to send to.  Addtionally all data should
        /// be sent to all spectators in the game, so they can keep up to date.
        async fn send_raw(&self, data: &[u8]) -> Result<(), NetworkError>;
        /// Returns a usize representing which player sent the data.
        async fn recv_raw(&self, data: &mut [u8]) -> Result<usize, NetworkError>;
        async fn stop(self) -> Result<usize, NetworkError>;

        /// Provide a function that makes changes to the game's settings
        /// This should be callable multiple times, so that if the update fails
        /// it can be retried.
        async fn update_meta<F: Fn(&mut Self::Meta)>(
            &mut self,
            update: F,
        ) -> Result<(), UpdateMetaError>;
        fn meta(&self) -> &Self::Meta;
    }
}
