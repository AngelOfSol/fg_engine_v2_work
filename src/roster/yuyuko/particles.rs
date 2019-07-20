use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum YuyukoParticle {
    SuperJumpParticle,
}

impl Default for YuyukoParticle {
    fn default() -> Self {
        YuyukoParticle::SuperJumpParticle
    }
}