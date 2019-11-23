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

    pub fn guarentee_unique_key<S: Into<String>>(&self, key: S) -> String {
        let base = key.into();
        let mut new_key = base.clone();
        let mut counter = 1;
        loop {
            if self.particles.contains_key(&new_key) {
                new_key = format!("{} ({})", base, counter);
                counter += 1;
            } else {
                break;
            }
        }
        new_key
    }
}

impl Default for Particles {
    fn default() -> Self {
        Self::new()
    }
}
