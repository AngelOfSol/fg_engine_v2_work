#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum JoinLobbyError {
    AlreadyConnected,
    Denied,
    NetworkError,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HostLobbyError {
    InLobby,
    NetworkError,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum JoinGameError {
    NoSuchGame,
    GameFull,
    AlreadyInGame,
    GameAlreadyStarted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SpectateGameError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CreateGameError {
    AlreadyInGame,
    OutOfGames,
}

pub enum UpdateMetaError {
    InvalidPermission,
    InvalidUpdate,
    OutOfDate,
    NetworkError,
}

pub enum NetworkError {
    PeerDisconnected,
    HostDisconneted,
}
