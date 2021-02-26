use std::net::SocketAddr;

use fg_netcode::{
    lobby::{lobby_state::LobbyState, GameInfo, InGame, JoinGameError, LobbyMessage},
    player_info::PlayerInfo,
    player_list::Player,
};
use serde::{Deserialize, Serialize};
use tokio::{
    select,
    sync::{broadcast, mpsc, oneshot, watch},
    task::JoinHandle,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LobbyStateAction {
    NewPlayer(PlayerInfo),
    UpdatePlayer(Player, PlayerInfo),
    Disconnect(Player),
    CreateGame(Player),
    JoinGame(Player, usize),
    UpdateAddr(Player, SocketAddr),
    #[serde(skip)]
    Kill,
}

pub struct LobbyTaskResult {
    pub task: JoinHandle<()>,
    pub state: watch::Receiver<LobbyState>,
    pub actions: mpsc::Sender<LobbyStateAction>,
    pub recv: crossbeam_channel::Receiver<LobbyMessage>,
}

pub fn host(info: PlayerInfo) -> LobbyTaskResult {
    join(LobbyState::new(info))
}

pub fn join(lobby_state: LobbyState) -> LobbyTaskResult {
    let user = lobby_state.user;

    let (watch_tx, watch_rx) = watch::channel(lobby_state);

    let (actions_tx, actions_rx) = mpsc::channel(4);

    let (messages_tx, messages_rx) = crossbeam_channel::bounded(4);

    let lobby_actor = LobbyActor {
        state: watch_tx,
        incoming: actions_rx,
        messages: messages_tx,
    };

    let task = tokio::spawn(lobby_state_loop(user, lobby_actor));

    LobbyTaskResult {
        task,
        recv: messages_rx,
        state: watch_rx,
        actions: actions_tx,
    }
}

struct LobbyActor {
    state: watch::Sender<LobbyState>,
    incoming: mpsc::Receiver<LobbyStateAction>,
    messages: crossbeam_channel::Sender<LobbyMessage>,
}

async fn lobby_state_loop(user: Player, mut actor: LobbyActor) {
    let mut lobby_state = actor.state.borrow().clone();

    while let Some(action) = actor.incoming.recv().await {
        if lobby_state.user != lobby_state.host_id() {
            dbg!("handling incoming action for user:");
            dbg!(user);
            dbg!(&action);
        }
        match action {
            LobbyStateAction::Kill => break,
            LobbyStateAction::UpdateAddr(player, addr) => {
                lobby_state.player_list.get_mut(player).unwrap().addr = addr;
            }
            LobbyStateAction::NewPlayer(info) => {
                lobby_state.player_list.insert(info);
            }
            LobbyStateAction::Disconnect(id) => {
                if id == user {
                    break;
                }
                lobby_state.remove(&id);
            }
            LobbyStateAction::CreateGame(player) => {
                if lobby_state
                    .games
                    .iter()
                    .any(|game| game.player_list.iter().any(|item| item == &player))
                {
                } else {
                    lobby_state.games.push(GameInfo {
                        player_list: vec![player],
                        ready: [false, false].into(),
                    });
                }
            }
            LobbyStateAction::JoinGame(player, game) => {
                if lobby_state
                    .games
                    .iter()
                    .any(|game| game.player_list.iter().any(|item| item == &player))
                {
                } else if game >= lobby_state.games.len() {
                    todo!()
                } else {
                    lobby_state.games[game].player_list.push(player);
                }
            }
            LobbyStateAction::UpdatePlayer(player, info) => {
                if let Some(player) = lobby_state.player_list.get_mut(player) {
                    *player = info;
                }
            }
        }
        match actor.state.send(lobby_state.clone()) {
            Ok(_) => {
                if lobby_state.user != lobby_state.host_id() {
                    dbg!("action handled");
                }
            }
            Err(inside) => panic!(inside),
        }
    }
}
