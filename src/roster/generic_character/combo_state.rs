use crate::character::components::{AttackInfo, GroundAction};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct ComboState {
    pub hits: u32,
    pub total_damage: i32,
    pub last_hit_damage: i32,
    pub proration: i32,
    pub ground_action: GroundAction,
    pub available_limit: i32,
}

#[derive(Eq, PartialEq, Debug, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum HitModifier {
    GuardCrush,
    CounterHit,
    None,
}

impl ComboState {
    pub fn update(previous: Option<ComboState>, info: &AttackInfo, modifier: HitModifier) -> Self {
        match previous {
            Some(state) => {
                let proration = i32::max(info.proration * state.proration / 100, 20);
                let last_hit_damage = info.hit_damage * state.proration / 100;
                ComboState {
                    hits: state.hits + 1,
                    total_damage: state.total_damage + last_hit_damage,
                    last_hit_damage,
                    proration,
                    ground_action: info.ground_action,
                    available_limit: state.available_limit - info.limit_cost,
                }
            }
            None => {
                let initial_hit_damage = if modifier == HitModifier::GuardCrush {
                    0
                } else {
                    info.hit_damage
                };
                ComboState {
                    hits: 1,
                    total_damage: initial_hit_damage,
                    last_hit_damage: initial_hit_damage,
                    proration: info.proration,
                    ground_action: info.ground_action,
                    available_limit: if modifier == HitModifier::CounterHit {
                        info.counter_hit_limit
                    } else {
                        info.starter_limit
                    },
                }
            }
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AllowedCancel {
    Always,
    Hit,
    Block,
}
