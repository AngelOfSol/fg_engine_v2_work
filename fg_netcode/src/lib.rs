pub mod error;
pub mod game;
pub mod lobby;
pub mod player_info;
pub mod player_list;

use std::net::SocketAddr;

use crossbeam_channel::{bounded, Receiver, Sender, TryRecvError};
use error::{HostLobbyError, JoinLobbyError};
use lobby::Lobby;
use player_info::PlayerInfo;

pub struct Networking {
    rx: Receiver<NetworkingMessage>,
    _tx: Sender<NetworkingMessage>,
}

#[derive(Debug)]
pub enum NetworkingMessage {
    Host(Result<Lobby, HostLobbyError>),
    Join(Result<Lobby, JoinLobbyError>),
}

impl Default for Networking {
    fn default() -> Self {
        let (tx, rx) = bounded(4);
        Self { _tx: tx, rx }
    }
}
impl Networking {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn request_host(&mut self, _player: PlayerInfo) {}
    pub fn request_join(&mut self, _id: SocketAddr, _player: PlayerInfo) {}

    pub fn poll(&mut self) -> Option<NetworkingMessage> {
        match self.rx.try_recv() {
            Ok(value) => Some(value),
            Err(TryRecvError::Disconnected) => panic!("Backing network was disconnected."),
            Err(TryRecvError::Empty) => None,
        }
    }
}
