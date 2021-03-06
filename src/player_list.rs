use fg_datastructures::player_data::PlayerData;
use sdl_controller_backend::ControllerId;
use std::net::SocketAddr;
use strum::Display;

#[derive(Debug, Copy, Clone, PartialEq, Display)]
pub enum PlayerType {
    LocalGamepad(ControllerId),
    Networked(SocketAddr),
    Dummy,
}

impl PlayerType {
    pub fn is_networked(&self) -> bool {
        matches!(self, Self::Networked(_))
    }

    pub fn is_local(&self) -> bool {
        !self.is_networked()
    }

    #[allow(dead_code)]
    pub fn is_gamepad(&self) -> bool {
        matches!(self, Self::LocalGamepad(_))
    }

    pub fn is_dummy(&self) -> bool {
        matches!(self, Self::Dummy)
    }

    pub fn gamepad_id(&self) -> Option<ControllerId> {
        match self {
            Self::LocalGamepad(id) => Some(*id),
            Self::Dummy => None,
            Self::Networked(_) => None,
        }
    }
    pub fn addr(&self) -> Option<SocketAddr> {
        match self {
            Self::Networked(id) => Some(*id),
            Self::Dummy => None,
            Self::LocalGamepad(_) => None,
        }
    }
}

impl From<ControllerId> for PlayerType {
    fn from(value: ControllerId) -> Self {
        Self::LocalGamepad(value)
    }
}

impl From<SocketAddr> for PlayerType {
    fn from(value: SocketAddr) -> Self {
        Self::Networked(value)
    }
}

#[derive(Debug, Clone)]
pub struct PlayerList {
    pub current_players: PlayerData<PlayerType>,
    pub spectators: Vec<PlayerType>,
}

impl PlayerList {
    pub fn new(current_players: PlayerData<PlayerType>) -> Self {
        Self {
            current_players,
            spectators: Vec::new(),
        }
    }

    pub fn gamepads(&self) -> impl Iterator<Item = ControllerId> + '_ {
        self.current_players
            .iter()
            .filter_map(PlayerType::gamepad_id)
    }

    pub fn network_addrs(&self) -> impl Iterator<Item = SocketAddr> + '_ {
        self.current_players
            .iter()
            .chain(self.spectators.iter())
            .filter_map(PlayerType::addr)
    }
}
