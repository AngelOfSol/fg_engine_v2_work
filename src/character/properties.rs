

use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Properties {
    pub health: u32,
    pub name: String,
    #[serde(skip)]
    _secret: (),
}

impl Properties {
    pub fn new() -> Self {
        Self {
            health: 1,
            name: "new_chara".to_owned(),
            _secret: (),
        }
    }
}