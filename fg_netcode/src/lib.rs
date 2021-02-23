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
    pub fn request_host(&mut self, _player: PlayerInfo) {
        // let tx = self.tx.clone();
        // thread::spawn(move || {
        //     thread::sleep(Duration::from_secs(2));
        //     tx.send(NetworkingMessage::Host(Ok(Lobby::new(LobbyState {
        //         player_list: vec![player],
        //         user: 0,
        //         games: vec![],
        //     }))))
        //     .unwrap();
        // });
    }
    pub fn request_join(&mut self, _id: SocketAddr, _player: PlayerInfo) {
        // let tx = self.tx.clone();
        // thread::spawn(move || {
        //     thread::sleep(Duration::from_secs(2));
        //     tx.send(NetworkingMessage::Host(Ok(Lobby::new(LobbyState {
        //         player_list: vec![PlayerInfo::default(), player],
        //         user: 1,
        //         games: vec![GameInfo {
        //             player_list: vec![0],
        //             ready: [false, false].into(),
        //         }],
        //     }))))
        //     .unwrap();
        // });
    }

    pub fn poll(&mut self) -> Option<NetworkingMessage> {
        match self.rx.try_recv() {
            Ok(value) => Some(value),
            Err(TryRecvError::Disconnected) => panic!("Backing network was disconnected."),
            Err(TryRecvError::Empty) => None,
        }
    }
}
