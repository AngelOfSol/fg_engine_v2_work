use crate::typedefs::collision::Int;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Default)]
pub struct BulletSpawn {
    pub bullet_id: String,
    pub frame: usize,
    pub properties: HashMap<String, Int>,
}
