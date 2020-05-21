use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct MatchSettings {
    replay_version: usize,
    pub first_to: usize,
}

pub struct MatchSettingsBuilder {
    first_to: usize,
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
    pub fn new() -> MatchSettingsBuilder {
        MatchSettingsBuilder { first_to: 2 }
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
            first_to: self.first_to,
        }
    }
}
