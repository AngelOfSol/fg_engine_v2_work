use crate::{
    game::Game,
    player_info::PlayerInfo,
    player_list::{Player, PlayerList},
};
use fg_datastructures::player_data::PlayerData;
use serde::{Deserialize, Serialize};
use std::ops::Index;
use tokio::sync::watch;

use super::GameInfo;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct LobbyState {
    pub player_list: PlayerList,
    pub games: Vec<GameInfo>,
    pub user: Player,
}

impl LobbyState {
    pub fn remove(&mut self, removed: &Player) -> Option<PlayerInfo> {
        let res = self.player_list.remove(removed);
        for game in self.games.iter_mut() {
            game.player_list.retain(|item| item != removed);
        }
        res
    }

    pub fn is_user_host(&self) -> bool {
        self.player_list.is_host(self.user)
    }
    pub fn is_host(&self, id: Player) -> bool {
        self.player_list.is_host(id)
    }

    pub fn host_id(&self) -> Player {
        self.player_list.host_id()
    }

    pub fn host(&self) -> &PlayerInfo {
        &self.player_list[0]
    }
    pub fn clients(&self) -> &[PlayerInfo] {
        &self.player_list[1..]
    }
    pub fn players(&self) -> &[PlayerInfo] {
        &self.player_list
    }
    pub fn user(&self) -> &PlayerInfo {
        self.player_list.get(self.user).unwrap()
    }
    pub fn games(&self) -> &[GameInfo] {
        &self.games
    }
}
