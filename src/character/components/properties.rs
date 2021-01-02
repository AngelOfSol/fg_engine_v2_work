use crate::roster::Character;
use inspect_design::Inspect;
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Debug, Serialize, Deserialize, Clone, Inspect)]
pub struct Properties {
    pub health: i32,
    pub name: String,

    #[serde(default = "default_max_air_actions")]
    pub max_air_actions: usize,
    #[serde(default = "default_max_spirit_gauge")]
    pub max_spirit_gauge: i32,

    #[serde(default)]
    #[skip]
    pub character: Character,
}

fn default_max_air_actions() -> usize {
    2
}
fn default_max_spirit_gauge() -> i32 {
    500
}

impl Properties {
    pub fn new() -> Self {
        Self {
            health: 1,
            name: "new_chara".to_owned(),

            max_air_actions: default_max_air_actions(),
            max_spirit_gauge: default_max_spirit_gauge(),
            character: Default::default(),
        }
    }
}
