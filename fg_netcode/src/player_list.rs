use crate::player_info::PlayerInfo;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, Index, IndexMut};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Player(usize);

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlayerList {
    next_key: usize,
    values: Vec<PlayerInfo>,
    keys: Vec<Player>,
}

impl Default for PlayerList {
    fn default() -> Self {
        Self {
            next_key: 0,
            values: Vec::new(),
            keys: Vec::new(),
        }
    }
}

impl Deref for PlayerList {
    type Target = Vec<PlayerInfo>;
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl From<Vec<PlayerInfo>> for PlayerList {
    fn from(value: Vec<PlayerInfo>) -> Self {
        Self {
            next_key: value.len(),
            keys: (0..value.len()).into_iter().map(Player).collect(),
            values: value,
        }
    }
}

impl PlayerList {
    pub fn get(&self, index: Player) -> Option<&PlayerInfo> {
        Some(&self.values[self.get_idx(&index)?])
    }
    pub fn get_mut(&mut self, index: Player) -> Option<&mut PlayerInfo> {
        let idx = self.get_idx(&index)?;
        Some(&mut self.values[idx])
    }

    pub fn is_host(&self, index: Player) -> bool {
        self.keys[0] == index
    }

    pub fn host_id(&self) -> Player {
        self.keys[0]
    }

    pub fn find_key<F: Fn(&PlayerInfo) -> bool>(&self, f: F) -> Option<Player> {
        self.values
            .iter()
            .zip(self.keys.iter())
            .find(|(value, _)| f(*value))
            .map(|(_, key)| *key)
    }

    fn get_idx(&self, key: &Player) -> Option<usize> {
        self.keys.iter().position(|item| item == key)
    }

    pub fn insert(&mut self, value: PlayerInfo) -> Player {
        let key = Player(self.next_key);
        self.next_key = self.next_key.checked_add(1).unwrap();
        self.values.push(value);
        self.keys.push(key);
        key
    }
    pub fn remove(&mut self, key: &Player) -> Option<PlayerInfo> {
        let idx = self.get_idx(key)?;
        self.keys.remove(idx);
        Some(self.values.remove(idx))
    }
    pub fn pairs(&self) -> impl Iterator<Item = (&Player, &PlayerInfo)> {
        self.keys.iter().zip(self.values.iter())
    }

    pub fn keys(&self) -> impl Iterator<Item = &Player> {
        self.keys.iter()
    }
}
