use crate::graphics::particle::version::hash_map;
use crate::graphics::particle::Particle;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Particles {
    #[serde(flatten, deserialize_with = "hash_map::deserialize")]
    pub particles: HashMap<String, Particle>,
}
impl Particles {
    pub fn new() -> Self {
        Self {
            particles: HashMap::new(),
        }
    }
}

impl Default for Particles {
    fn default() -> Self {
        Self::new()
    }
}
