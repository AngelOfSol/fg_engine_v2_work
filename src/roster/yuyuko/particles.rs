use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Display)]
#[serde(rename_all = "snake_case")]
pub enum ParticleId {
    SuperJumpParticle,
    HitEffect,
    ButterflyFlare,
}

impl Default for ParticleId {
    fn default() -> Self {
        ParticleId::SuperJumpParticle
    }
}

impl ParticleId {
    pub fn file_name(self) -> String {
        serde_json::to_string(&self)
            .unwrap()
            .trim_matches('\"')
            .to_owned()
    }
}
