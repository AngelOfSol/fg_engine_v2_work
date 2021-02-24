use std::net::SocketAddr;

use fg_netcode::{
    lobby::{lobby_state::LobbyState, GameInfo, InGame, JoinGameError},
    player_info::PlayerInfo,
    player_list::Player,
};
use tokio::{
    sync::{mpsc, oneshot, watch},
    task::JoinHandle,
};

#[derive(Debug)]
pub enum LobbyStateAction {
    NewPlayer(PlayerInfo),
    UpdatePlayer(Player, PlayerInfo),
    Disconnect(Player),
    RawUpdate(LobbyState),
    UpdateAddr(SocketAddr),
    CreateGame(Player, oneshot::Sender<Result<usize, InGame>>),
    JoinGame(usize, Player, oneshot::Sender<Result<usize, JoinGameError>>),
    Kill,
}

#[derive(Clone)]
pub struct LobbyStateInterface {
    pub state: watch::Receiver<LobbyState>,
    pub actions: mpsc::Sender<LobbyStateAction>,
}

pub fn host(info: PlayerInfo) -> (JoinHandle<()>, LobbyStateInterface) {
    join(LobbyState::new(info))
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
            LobbyStateAction::CreateGame(player, result) => {
                if lobby_state
                    .games
                    .iter()
                    .any(|game| game.player_list.iter().any(|item| item == &player))
                {
                    let _ = result.send(Err(InGame));
                } else {
                    lobby_state.games.push(GameInfo {
                        player_list: vec![player],
                        ready: [false, false].into(),
                    });

                    let _ = result.send(Ok(lobby_state.games.len() - 1));
                }
            }
            LobbyStateAction::JoinGame(game, player, result) => {
                if lobby_state
                    .games
                    .iter()
                    .any(|game| game.player_list.iter().any(|item| item == &player))
                {
                    let _ = result.send(Err(JoinGameError::InGame));
                } else if game >= lobby_state.games.len() {
                    let _ = result.send(Err(JoinGameError::InvalidGame));
                } else {
                    lobby_state.games[game].player_list.push(player);
                    let _ = result.send(Ok(game));
                }
            }
            LobbyStateAction::UpdatePlayer(player, info) => {
                if let Some(player) = lobby_state.player_list.get_mut(player) {
                    *player = info;
                }
            }
        }
        match tx.send(lobby_state.clone()) {
            Ok(_) => {}
            Err(_) => break,
        }
    }
}
