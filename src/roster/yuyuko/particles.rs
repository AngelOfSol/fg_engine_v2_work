use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Particle {
    SuperJumpParticle,
    HitEffect,
}

impl Default for Particle {
    fn default() -> Self {
        Particle::SuperJumpParticle
    }
}
