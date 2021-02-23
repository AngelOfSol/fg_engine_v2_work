use std::net::SocketAddr;

use fg_datastructures::roster::RosterCharacter;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerInfo {
    pub name: String,
    pub character: RosterCharacter,
    pub addr: SocketAddr,
}

impl Default for PlayerInfo {
    fn default() -> Self {
        Self {
            name: "Fake Player".to_string(),
            character: RosterCharacter::default(),
            addr: ":::0".parse().unwrap(),
        }
    }
}
