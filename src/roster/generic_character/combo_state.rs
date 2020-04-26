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
                let info = info.on_hit;
                let proration = i32::max(info.proration * state.proration / 100, 20);
                let last_hit_damage = info.damage * state.proration / 100;
                ComboState {
                    hits: state.hits + 1,
                    total_damage: state.total_damage + last_hit_damage,
                    last_hit_damage,
                    proration,
                    ground_action: info.ground_action,
                    available_limit: state.available_limit - info.limit_cost,
                }
            }
            None => match modifier {
                HitModifier::None => {
                    let info = info.on_hit;
                    ComboState {
                        hits: 1,
                        total_damage: info.damage,
                        last_hit_damage: info.damage,
                        proration: info.proration,
                        ground_action: info.ground_action,
                        available_limit: info.starter_limit,
                    }
                }
                HitModifier::GuardCrush => {
                    let info = info.on_guard_crush;
                    ComboState {
                        hits: 1,
                        total_damage: info.damage,
                        last_hit_damage: info.damage,
                        proration: info.proration,
                        ground_action: info.ground_action,
                        available_limit: info.starter_limit,
                    }
                }
                HitModifier::CounterHit => {
                    let info = info.on_counter_hit;
                    ComboState {
                        hits: 1,
                        total_damage: info.damage,
                        last_hit_damage: info.damage,
                        proration: info.proration,
                        ground_action: info.ground_action,
                        available_limit: info.starter_limit,
                    }
                }
            },
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AllowedCancel {
    Always,
    Hit,
    Block,
}
