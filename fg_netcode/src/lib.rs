use async_trait::async_trait;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum JoinError {
    AlreadyConnected,
    Denied,
}
pub enum HostError {
    AlreadyHosting,
}

#[async_trait]
pub trait NetworkingSubsytem {
    type LobbyId;
    type GameId;
    type PlayerId;

    async fn request_host(&mut self) -> Result<Self::LobbyId, HostError>;
    async fn request_join(&mut self, lobby: Self::LobbyId) -> Result<Self::LobbyId, JoinError>;
    // fn request_lobby_list(&self) -> Vec<Lobby<Self::LobbyId>>;
}

// every method just on the networking subsystem trait
// return none if the data isn't cached
// provide BASIC smart pointer types

// use ASYNC
