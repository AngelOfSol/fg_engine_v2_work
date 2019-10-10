use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BulletId {
    Butterfly,
}

impl Default for BulletId {
    fn default() -> Self {
        BulletId::Butterfly
    }
}
