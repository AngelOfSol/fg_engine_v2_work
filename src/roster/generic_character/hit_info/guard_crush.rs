use crate::{
    character::components::AttackInfo,
    roster::hit_info::{Force, HitSource},
    typedefs::collision,
};

use super::{ComboEffect, OnHitType, Source};
pub struct Effect {
    pub defender: DefenderEffect,
    pub combo: ComboEffect,
}
pub struct DefenderEffect {
    pub is_lethal: bool,
    pub take_damage: i32,
    pub modify_meter: i32,
    pub set_stun: i32,
    pub set_force: Force,
    pub set_stop: i32,
    pub set_should_pushback: bool,
}

impl Effect {
    pub fn build(attack_info: &AttackInfo, source: &Source, airborne: bool) -> (Effect, OnHitType) {
        let guard_crush_info = &attack_info.on_guard_crush;
        (
            Effect {
                combo: ComboEffect {
                    available_limit: guard_crush_info.starter_limit,
                    hits: 1,
                    proration: guard_crush_info.proration,
                    total_damage: guard_crush_info.damage,
                    ground_action: guard_crush_info.ground_action,
                },
                defender: DefenderEffect {
                    is_lethal: guard_crush_info.lethal,
                    modify_meter: guard_crush_info.defender_meter,
                    set_stop: guard_crush_info.defender_stop,
                    set_stun: if airborne || guard_crush_info.launcher {
                        guard_crush_info.air_stun
                    } else {
                        guard_crush_info.stun
                    },
                    take_damage: guard_crush_info.damage,
                    set_should_pushback: source.source_type == HitSource::Character,
                    set_force: if airborne || guard_crush_info.launcher {
                        Force::Airborne(source.facing.fix_collision(guard_crush_info.air_force))
                    } else {
                        Force::Grounded(source.facing.fix_collision(collision::Vec2::new(
                            guard_crush_info.ground_pushback,
                            0_00,
                        )))
                    },
                },
            },
            OnHitType::GuardCrush,
        )
    }
    pub fn append_hit(mut self, attack_info: &AttackInfo) -> (Self, OnHitType) {
        let attack_info = &attack_info.on_hit;
        let damage = attack_info.damage * self.combo.proration / 100;

        self.combo.available_limit -= attack_info.limit_cost;
        self.combo.hits += 1;
        self.combo.proration *= attack_info.proration;
        self.combo.proration /= 100;
        self.combo.total_damage += damage;

        self.defender.is_lethal |= attack_info.lethal;
        self.defender.modify_meter += attack_info.defender_meter;
        self.defender.take_damage += damage;

        (self, OnHitType::Hit)
    }
}
