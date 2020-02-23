use crate::typedefs::collision::{Int, Vec2};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct BulletSpawn {
    pub bullet_id: String,
    pub frame: usize,
    pub offset: Vec2,
    #[serde(flatten)]
    pub properties: HashMap<String, Int>,
}

impl Default for BulletSpawn {
    fn default() -> Self {
        Self {
            bullet_id: "".to_owned(),
            frame: 0,
            offset: Vec2::new(0_00, 0_00),
            properties: HashMap::new(),
        }
    }
}

impl BulletSpawn {
    pub fn new(bullet_id: String, properties: &HashSet<String>) -> Self {
        Self {
            bullet_id,
            frame: 0,
            offset: Vec2::new(0_00, 0_00),
            properties: properties.iter().map(|key| (key.clone(), 0)).collect(),
        }
    }
}
