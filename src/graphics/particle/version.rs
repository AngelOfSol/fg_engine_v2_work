use super::{Particle, ParticleV1};
use crate::graphics::animation::{self, Animation};
use crate::graphics::keyframe::Modifiers;

use serde::Deserialize;

pub mod hash_map {
    use super::{Particle, ParticleVersioned};
    use serde::{Deserialize, Deserializer};
    use std::collections::HashMap;

    pub fn deserialize<'de, D, K>(deserializer: D) -> Result<HashMap<K, Particle>, D::Error>
    where
        D: Deserializer<'de>,
        K: std::cmp::Eq + std::hash::Hash + Deserialize<'de>,
        HashMap<K, Particle>: Deserialize<'de>,
    {
        Ok(HashMap::<K, ParticleVersioned>::deserialize(deserializer)?
            .into_iter()
            .map(|(key, value)| (key, value.into_modern()))
            .collect())
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum ParticleVersioned {
    V1(ParticleV1),
    #[serde(deserialize_with = "animation::version::single::deserialize")]
    Legacy(Animation),
}

impl ParticleVersioned {
    fn into_modern(self) -> Particle {
        match self {
            Self::V1(value) => value.into_modern(),
            Self::Legacy(value) => ParticleV1 {
                animations: vec![value],
                modifiers: Modifiers::new(),
            }
            .into_modern(),
        }
    }
}
