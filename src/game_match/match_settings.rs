use crate::player_list::PlayerList;
use crate::roster::Character;
use crate::typedefs::player::PlayerData;
use ggez::{Context, GameResult};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct MatchSettings {
    replay_version: usize,
    pub first_to: usize,
    pub characters: PlayerData<Character>,
}

pub struct MatchSettingsBuilder {
    first_to: usize,
    characters: PlayerData<Character>,
}

pub enum MatchSettingsError {
    ReplayVersionMismatch,
    DeserializeError(bincode::Error),
}

impl From<bincode::Error> for MatchSettingsError {
    fn from(value: bincode::Error) -> MatchSettingsError {
        MatchSettingsError::DeserializeError(value)
    }
}

impl MatchSettings {
    pub fn new(characters: PlayerData<Character>) -> MatchSettingsBuilder {
        MatchSettingsBuilder {
            first_to: 2,
            characters,
        }
    }

    pub fn validate(&self) -> Result<(), MatchSettingsError> {
        if self.replay_version != crate::typedefs::REPLAY_VERSION {
            return Err(MatchSettingsError::ReplayVersionMismatch);
        }

        Ok(())
    }
}

impl MatchSettingsBuilder {
    pub fn first_to(mut self, wins: usize) -> Self {
        self.first_to = wins;
        self
    }
    pub fn build(self) -> MatchSettings {
        MatchSettings {
            replay_version: crate::typedefs::REPLAY_VERSION,
            characters: self.characters,
            first_to: self.first_to,
        }
    }
}
pub trait FromMatchSettings {
    fn from_settings(
        ctx: &mut Context,
        player_list: PlayerList,
        settings: MatchSettings,
    ) -> GameResult<Box<Self>>;
}
