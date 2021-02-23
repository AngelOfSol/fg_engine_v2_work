pub mod lobby_state;

use crate::{game::Game, player_info::PlayerInfo, player_list::Player};
use fg_datastructures::player_data::PlayerData;
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, watch};

use self::lobby_state::LobbyState;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct GameInfo {
    pub player_list: Vec<Player>,
    pub ready: PlayerData<bool>,
}

impl GameInfo {
    pub fn spectators(&self) -> &[Player] {
        if self.player_list.len() > 2 {
            &self.player_list[2..]
        } else {
            &self.player_list[0..0]
        }
    }
    pub fn players(&self) -> &[Player] {
        &self.player_list[0..2.min(self.player_list.len())]
    }
}

#[derive(Debug)]
pub struct Lobby {
    state: watch::Receiver<LobbyState>,
    message: async_channel::Receiver<LobbyMessage>,
    action: mpsc::Sender<LobbyAction>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidGame;

pub enum LobbyAction {
    CreateGame,
    JoinGame(Game),
    UpdatePlayerInfo(Box<dyn FnOnce(&mut PlayerInfo) + Send + Sync>),
}

pub enum LobbyMessage {
    Empty,
}

impl Lobby {
    pub fn new(
        state: watch::Receiver<LobbyState>,
        message: async_channel::Receiver<LobbyMessage>,
        action: mpsc::Sender<LobbyAction>,
    ) -> Self {
        Self {
            state,
            message,
            action,
        }
    }

    pub fn state(&self) -> watch::Ref<'_, LobbyState> {
        self.state.borrow()
    }

    pub fn create_game(&mut self) {
        // let mut temp = (*self.state.borrow()).clone();
        // temp.games.push(GameInfo {
        //     player_list: vec![temp.user, temp.user, temp.user],
        //     ready: [false, false].into(),
        // });

        // self.tx.send(temp).unwrap();
    }
    pub fn join_game(&mut self, _idx: usize) -> Result<(), InvalidGame> {
        Err(InvalidGame)
    }

    pub fn update_player_data<F: FnOnce(&mut PlayerInfo)>(&mut self, _update: F) {
        // let mut temp = (*self.state.borrow()).clone();
        // update(&mut temp.player_list[temp.user]);
        // self.tx.send(temp).unwrap();
    }

    pub fn poll(&mut self) -> Option<LobbyAction> {
        None
    }
}
