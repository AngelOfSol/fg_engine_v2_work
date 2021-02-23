use fg_netcode::{lobby::lobby_state::LobbyState, player_info::PlayerInfo};
use quinn::{ConnectError, ConnectionError, ReadError, ReadToEndError, WriteError};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Serialize, Deserialize)]
pub(crate) struct JoinRequest {
    pub(crate) target: SocketAddr,
    pub(crate) info: PlayerInfo,
}
#[derive(Serialize, Deserialize)]
pub(crate) struct JoinResponse {
    pub(crate) target: SocketAddr,
}

#[derive(Serialize, Deserialize)]
pub enum ClientPacket {
    CreateGame,
}

#[derive(Serialize, Deserialize)]
pub enum HostPacket {
    LobbyUpdate(LobbyState),
}

#[derive(Debug)]
pub struct Disconnected;

impl From<ConnectionError> for Disconnected {
    fn from(_: ConnectionError) -> Self {
        Self
    }
}

impl From<WriteError> for Disconnected {
    fn from(_: WriteError) -> Self {
        Self
    }
}
impl From<ReadError> for Disconnected {
    fn from(_: ReadError) -> Self {
        Self
    }
}
impl From<ReadToEndError> for Disconnected {
    fn from(_: ReadToEndError) -> Self {
        Self
    }
}
impl From<ConnectError> for Disconnected {
    fn from(_: ConnectError) -> Self {
        Self
    }
}
