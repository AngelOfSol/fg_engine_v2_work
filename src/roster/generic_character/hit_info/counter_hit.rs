use crate::{
    character::components::AttackInfo,
    roster::hit_info::{Force, HitSource},
    typedefs::collision,
};

use super::{ComboEffect, HitType, Source};
pub struct Effect {
    pub defender: DefenderEffect,
    pub combo: ComboEffect,
}
pub struct DefenderEffect {
    pub is_lethal: bool,
    pub take_damage: i32,
    pub take_spirit_gauge: i32,
    pub modify_meter: i32,
    pub add_spirit_delay: i32,
    pub reset_spirit_delay: bool,
    pub set_stun: i32,
    pub set_force: Force,
    pub set_stop: i32,
    pub set_should_pushback: bool,
}

impl Effect {
    pub fn build(attack_info: &AttackInfo, source: &Source, airborne: bool) -> (Effect, HitType) {
        let counter_hit_info = &attack_info.on_counter_hit;
        (
            Effect {
                combo: ComboEffect {
                    available_limit: counter_hit_info.starter_limit,
                    hits: 1,
                    proration: counter_hit_info.proration,
                    total_damage: counter_hit_info.damage,
                    ground_action: counter_hit_info.ground_action,
                },
                defender: DefenderEffect {
                    add_spirit_delay: counter_hit_info.spirit_delay,
                    is_lethal: counter_hit_info.lethal,
                    modify_meter: counter_hit_info.defender_meter,
                    reset_spirit_delay: counter_hit_info.reset_spirit_delay,
                    set_stop: counter_hit_info.defender_stop,
                    take_spirit_gauge: counter_hit_info.spirit_cost,
                    set_stun: if airborne || counter_hit_info.launcher {
                        counter_hit_info.air_stun
                    } else {
                        counter_hit_info.stun
                    },
                    take_damage: counter_hit_info.damage,
                    set_should_pushback: source.source_type == HitSource::Character,
                    set_force: if airborne || counter_hit_info.launcher {
                        Force::Airborne(source.facing.fix_collision(counter_hit_info.air_force))
                    } else {
                        Force::Grounded(source.facing.fix_collision(collision::Vec2::new(
                            counter_hit_info.ground_pushback,
                            0_00,
                        )))
                    },
                },
            },
            HitType::CounterHit,
        )
    }

    pub fn append_hit(mut self, attack_info: &AttackInfo) -> (Self, HitType) {
        let attack_info = &attack_info.on_hit;
        let damage = attack_info.damage * self.combo.proration / 100;

        self.combo.available_limit -= attack_info.limit_cost;
        self.combo.hits += 1;
        self.combo.proration *= attack_info.proration;
        self.combo.proration /= 100;
        self.combo.total_damage += damage;

        self.defender.add_spirit_delay += attack_info.spirit_delay;
        self.defender.is_lethal |= attack_info.lethal;
        self.defender.modify_meter += attack_info.defender_meter;
        self.defender.reset_spirit_delay |= attack_info.reset_spirit_delay;
        self.defender.take_spirit_gauge += attack_info.spirit_cost;
        self.defender.take_damage += damage;

        (self, HitType::Hit)
    }
}
