use crate::graphics::Animation;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Particles {
    #[serde(flatten)]
    pub particles: HashMap<String, Animation>,
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
