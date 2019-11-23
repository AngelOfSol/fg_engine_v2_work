mod bullet_info;

pub use bullet_info::BulletInfo;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bullets {
    #[serde(flatten)]
    pub bullets: HashMap<String, BulletInfo>,
}

impl Bullets {
    pub fn new() -> Self {
        Self {
            bullets: HashMap::new(),
        }
    }

    pub fn guarentee_unique_key<S: Into<String>>(&self, key: S) -> String {
        let base = key.into();
        let mut new_key = base.clone();
        let mut counter = 1;
        loop {
            if self.bullets.contains_key(&new_key) {
                new_key = format!("{} ({})", base, counter);
                counter += 1;
            } else {
                break;
            }
        }
        new_key
    }
}

impl Default for Bullets {
    fn default() -> Self {
        Self::new()
    }
}
