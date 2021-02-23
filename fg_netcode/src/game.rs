use bytes::Bytes;

use crate::player_list::Player;

pub struct Game {}

pub enum GameMessage {
    PlayersReady,
}

impl Game {
    pub fn pass(&mut self) {}
    pub fn ready(&mut self) {}
    pub fn poll(&mut self) -> Option<GameMessage> {
        None
    }
}

pub struct Match {}
pub enum MatchMessage {
    Packet(WhoIs, Bytes),
    Disconnected(WhoIs),
}

pub enum WhoIs {
    P1,
    P2,
    Spectator,
}

impl Match {
    pub fn poll(&mut self) -> Option<MatchMessage> {
        None
    }
    pub fn send(&mut self) {}
    pub fn who_is(&self, player: Player) -> WhoIs {
        WhoIs::P1
    }
}
