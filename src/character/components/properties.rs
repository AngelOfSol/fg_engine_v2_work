use fg_datastructures::roster::RosterCharacter;
use inspect_design::Inspect;
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Debug, Serialize, Deserialize, Clone, Inspect, Default)]
pub struct Properties {
    pub health: i32,
    pub name: String,

    #[serde(default = "default_max_air_actions")]
    pub max_air_actions: usize,
    #[serde(default = "default_max_spirit_gauge")]
    pub max_spirit_gauge: i32,

    #[serde(default)]
    #[skip]
    pub character: RosterCharacter,
}

fn default_max_air_actions() -> usize {
    2
}
fn default_max_spirit_gauge() -> i32 {
    500
}
