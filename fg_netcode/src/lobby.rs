use std::{ops::Index, thread, time::Duration};

use tokio::sync::watch;

#[derive(Clone, Debug)]
pub struct PlayerInfo {
    pub name: String,
}

pub type Player = usize;

#[derive(Clone, Debug)]
pub struct GameInfo {
    player_list: Vec<Player>,
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

#[derive(Clone, Debug)]
pub struct LobbyState {
    pub(crate) player_list: Vec<PlayerInfo>,
    pub(crate) games: Vec<GameInfo>,
    pub(crate) user: Player,
}

impl Index<Player> for LobbyState {
    type Output = PlayerInfo;
    fn index(&self, index: Player) -> &Self::Output {
        &self.player_list[index]
    }
}

impl LobbyState {
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
        &self[self.user]
    }
    pub fn games(&self) -> &[GameInfo] {
        &self.games
    }
}

pub struct Lobby {
    state: watch::Receiver<LobbyState>,
    tx: watch::Sender<LobbyState>,
}

impl Lobby {
    pub(crate) fn new(state: LobbyState) -> Self {
        let (tx, rx) = watch::channel(state);

        Self { state: rx, tx }
    }

    pub fn state(&self) -> watch::Ref<'_, LobbyState> {
        self.state.borrow()
    }

    pub fn create_game(&mut self) {
        let mut temp = (*self.state.borrow()).clone();
        temp.games.push(GameInfo {
            player_list: vec![temp.user, temp.user, temp.user],
        });

        self.tx.send(temp).unwrap();
    }
}
