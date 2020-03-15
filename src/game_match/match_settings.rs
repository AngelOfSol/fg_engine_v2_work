use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct MatchSettings {
    replay_version: usize,
}

pub struct MatchSettingsBuilder {}

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
        MatchSettingsBuilder {}
    }

    pub fn validate(&self) -> Result<(), MatchSettingsError> {
        if self.replay_version != crate::typedefs::REPLAY_VERSION {
            return Err(MatchSettingsError::ReplayVersionMismatch);
        }

        Ok(())
    }
}

impl MatchSettingsBuilder {
    pub fn build(self) -> MatchSettings {
        MatchSettings {
            replay_version: crate::typedefs::REPLAY_VERSION,
        }
    }
}
