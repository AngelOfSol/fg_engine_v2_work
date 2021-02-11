pub mod error;

use async_trait::async_trait;
use error::{
    CreateGameError, HostLobbyError, JoinGameError, JoinLobbyError, NetworkError,
    SpectateGameError, UpdateMetaError,
};

/// You can join/host a lobby via an ID, and once you're a part of a lobby,
/// you get a "handle" that represents the lobby and allows you to access its state.
/// Additionally you can ask to access the list of available lobbies, but not
/// every implementation will return any information here.
#[async_trait]
pub trait NetworkingSubsytem {
    type LobbyId;
    type Lobby: Lobby;

    async fn request_host(&mut self) -> Result<Self::Lobby, HostLobbyError>;
    async fn request_join(&mut self, lobby: Self::LobbyId) -> Result<Self::Lobby, JoinLobbyError>;
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
    type Meta;
    type PlayerInfo;

    fn game_list(&self) -> &[GameInfo<'_, Self::PlayerInfo, Self::Meta>];
    fn idle_players(&self) -> &[Self::PlayerInfo];

    async fn create_game(&self) -> Result<Self::Playing, CreateGameError>;
    async fn join_game(&self, id: usize) -> Result<Self::Playing, JoinGameError>;
    async fn spectate_game(&self, id: usize) -> Result<Self::Spectating, SpectateGameError>;
    async fn leave(self) -> Result<(), NetworkError>;
}
pub struct GameInfo<'a, PlayerInfo, Meta> {
    pub players: &'a [PlayerInfo],
    pub spectators: &'a [PlayerInfo],
    pub meta: &'a Meta,
}

/// A handle to a game where the user is spectating.  They're given read-only access
/// to the game's settings, and are not allowed to send packets to to other players
/// in the same game.
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

    /// Should indicate that the player is ready to start.
    async fn ready(&mut self) -> Result<usize, NetworkError>;
    async fn unready(&mut self) -> Result<usize, NetworkError>;
    /// Provide a function that makes changes to the game's settings
    /// This should be callable multiple times, so that if the update fails
    /// it can be retried.
    async fn update_meta<F: Fn(&mut Self::Meta)>(
        &mut self,
        update: F,
    ) -> Result<(), UpdateMetaError>;
    fn meta(&self) -> &Self::Meta;
}
