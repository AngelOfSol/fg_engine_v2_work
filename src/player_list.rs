use crate::input::pads_context::GamepadId;
use crate::typedefs::player::PlayerData;
use std::net::SocketAddr;
use strum_macros::Display;

#[derive(Debug, Copy, Clone, PartialEq, Display)]
pub enum PlayerType {
    LocalGamepad(GamepadId),
    Networked(SocketAddr),
    Dummy,
}

impl PlayerType {
    pub fn is_networked(&self) -> bool {
        match self {
            Self::Networked(_) => true,
            _ => false,
        }
    }

    pub fn is_local(&self) -> bool {
        !self.is_networked()
    }

    #[allow(dead_code)]
    pub fn is_gamepad(&self) -> bool {
        match self {
            Self::LocalGamepad(_) => true,
            _ => false,
        }
    }

    pub fn is_dummy(&self) -> bool {
        match self {
            Self::Dummy => true,
            _ => false,
        }
    }

    pub fn gamepad_id(&self) -> Option<GamepadId> {
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

impl From<GamepadId> for PlayerType {
    fn from(value: GamepadId) -> Self {
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

    pub fn gamepads<'a>(&'a self) -> impl Iterator<Item = GamepadId> + 'a {
        self.current_players
            .iter()
            .filter_map(PlayerType::gamepad_id)
    }

    pub fn network_addrs<'a>(&'a self) -> impl Iterator<Item = SocketAddr> + 'a {
        self.current_players
            .iter()
            .chain(self.spectators.iter())
            .filter_map(PlayerType::addr)
    }
}
