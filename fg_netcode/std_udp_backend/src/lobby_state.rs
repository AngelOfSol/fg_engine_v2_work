use std::{borrow::BorrowMut, net::SocketAddr};

use fg_netcode::{
    lobby::lobby_state::LobbyState,
    player_info::PlayerInfo,
    player_list::{self, Player, PlayerList},
};
use futures_util::FutureExt;
use tokio::{
    sync::{mpsc, watch},
    task::JoinHandle,
};

#[derive(Debug)]
pub enum LobbyStateAction {
    NewPlayer(PlayerInfo),
    Disconnect(Player),
    RawUpdate(LobbyState),
    UpdateAddr(SocketAddr),
    Kill,
}

#[derive(Clone)]
pub struct LobbyStateInterface {
    pub state: watch::Receiver<LobbyState>,
    pub actions: mpsc::Sender<LobbyStateAction>,
}

pub fn host(info: PlayerInfo) -> (JoinHandle<()>, LobbyStateInterface) {
    let (user, player_list) = {
        let mut player_list = PlayerList::default();
        let user = player_list.insert(info);
        (user, player_list)
    };

    join(LobbyState {
        games: vec![],
        player_list,
        user,
    })
}

pub fn join(lobby_state: LobbyState) -> (JoinHandle<()>, LobbyStateInterface) {
    let user = lobby_state.user;

    let (watch_tx, watch_rx) = watch::channel(lobby_state);

    let (actions_tx, actions_rx) = mpsc::channel(4);

    let task = tokio::spawn(lobby_state_loop(user, watch_tx, actions_rx));

    (
        task,
        LobbyStateInterface {
            state: watch_rx,
            actions: actions_tx,
        },
    )
}

async fn lobby_state_loop(
    user: Player,
    tx: watch::Sender<LobbyState>,
    mut actions: mpsc::Receiver<LobbyStateAction>,
) {
    let mut lobby_state = tx.borrow().clone();
    while let Some(action) = actions.recv().await {
        match action {
            LobbyStateAction::NewPlayer(info) => {
                let _id = lobby_state.player_list.insert(info);
            }
            LobbyStateAction::Disconnect(id) => {
                if id == user {
                    break;
                }
                let _old_player = lobby_state.remove(&id);
            }
            LobbyStateAction::RawUpdate(new_lobby_state) => {
                assert_eq!(lobby_state.user, user);
                lobby_state = new_lobby_state;
            }
            LobbyStateAction::Kill => break,
            LobbyStateAction::UpdateAddr(addr) => {
                lobby_state.player_list.get_mut(user).unwrap().addr = addr;
            }
        }
        match tx.send(lobby_state.clone()) {
            Ok(_) => {}
            Err(_) => break,
        }
    }
}
