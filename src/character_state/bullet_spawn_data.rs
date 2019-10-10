use crate::typedefs::collision::Int;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct BulletSpawn<Id> {
    pub bullet_id: Id,
    pub frame: usize,
    pub properties: HashMap<String, Int>,
}
