use crate::typedefs::collision::Vec2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct ParticleSpawn {
    pub particle_id: String,
    pub frame: usize,
    pub offset: Vec2,
}

impl ParticleSpawn {
    pub fn new() -> Self {
        Self {
            particle_id: "CHANGE ID".to_owned(),
            frame: 0,
            offset: Vec2::zeros(),
        }
    }
}
