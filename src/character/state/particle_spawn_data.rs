use crate::typedefs::collision::Vec2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct ParticleSpawn<Id> {
    pub particle_id: Id,
    pub frame: usize,
    pub offset: Vec2,
}

impl ParticleSpawn<String> {
    pub fn new(particle_id: String) -> Self {
        Self {
            particle_id,
            frame: 0,
            offset: Vec2::zeros(),
        }
    }
}
