use crate::character::components::GroundAction;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComboState {
    pub hits: u32,
    pub total_damage: i32,
    pub last_hit_damage: i32,
    pub proration: i32,
    pub ground_action: GroundAction,
    pub available_limit: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AllowedCancel {
    Always,
    Hit,
    Block,
}
