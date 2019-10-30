use crate::roster::generic_character::particle_id::GenericParticleId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Particle {
    SuperJumpParticle,
    HitEffect,
}

impl GenericParticleId for Particle {
    const ON_HIT: Self = Self::HitEffect;
}

impl Default for Particle {
    fn default() -> Self {
        Particle::SuperJumpParticle
    }
}
