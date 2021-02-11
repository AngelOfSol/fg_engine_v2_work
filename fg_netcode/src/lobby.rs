use tokio::sync::watch;

pub struct PlayerInfo {
    pub name: String,
}

pub struct LobbyState {
    pub host: PlayerInfo,
    pub player_list: Vec<PlayerInfo>,
}

pub struct Lobby {
    state: watch::Receiver<LobbyState>,
}

impl Lobby {
    pub(crate) fn new(state: LobbyState) -> Self {
        let (tx, rx) = watch::channel(state);

        Box::leak(Box::new(tx));

        Self { state: rx }
    }

    pub fn state(&self) -> watch::Ref<'_, LobbyState> {
        self.state.borrow()
    }
}
