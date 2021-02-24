pub mod lobby_state;

use crate::{game::Game, player_info::PlayerInfo, player_list::Player};
use crossbeam_channel::TryRecvError;
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
    message: crossbeam_channel::Receiver<LobbyMessage>,
    action: mpsc::Sender<LobbyAction>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidGame;

#[derive(Debug, PartialEq, Eq)]
pub enum LobbyAction {
    CreateGame,
    JoinGame(usize),
    UpdatePlayerInfo(PlayerInfo),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JoinGameError {
    InGame,
    InvalidGame,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InGame;

#[derive(Debug, PartialEq, Eq)]
pub enum LobbyMessage {
    CreateGame(Result<usize, InGame>),
    JoinGame(Result<usize, JoinGameError>),
}

impl Lobby {
    pub fn new(
        state: watch::Receiver<LobbyState>,
        message: crossbeam_channel::Receiver<LobbyMessage>,
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

    pub fn create_game(&self) {
        self.action.blocking_send(LobbyAction::CreateGame).unwrap();
    }
    pub fn join_game(&mut self, _idx: usize) -> Result<(), InvalidGame> {
        Err(InvalidGame)
    }

    pub fn update_player_data<F: FnOnce(&mut PlayerInfo)>(&self, update: F) {
        let mut temp = self.state.borrow().user().clone();
        update(&mut temp);
        self.action
            .blocking_send(LobbyAction::UpdatePlayerInfo(temp))
            .unwrap();
    }

    pub fn poll(&self) -> Option<LobbyMessage> {
        match self.message.try_recv() {
            Ok(value) => Some(value),
            Err(TryRecvError::Disconnected) => panic!("Backing network was disconnected."),
            Err(TryRecvError::Empty) => None,
        }
    }
}
