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
    pub fn build_starter(
        attack_info: &AttackInfo,
        source: &Source,
        airborne: bool,
    ) -> (Effect, HitType) {
        let attack_info = &attack_info.on_hit;
        (
            Effect {
                combo: ComboEffect {
                    available_limit: attack_info.starter_limit,
                    hits: 1,
                    proration: attack_info.proration,
                    total_damage: attack_info.damage,
                    ground_action: attack_info.ground_action,
                },
                defender: DefenderEffect {
                    add_spirit_delay: attack_info.spirit_delay,
                    is_lethal: attack_info.lethal,
                    modify_meter: attack_info.defender_meter,
                    reset_spirit_delay: attack_info.reset_spirit_delay,
                    set_stop: attack_info.defender_stop,
                    take_spirit_gauge: attack_info.spirit_cost,
                    set_stun: if airborne || attack_info.launcher {
                        attack_info.air_stun
                    } else {
                        attack_info.stun
                    },
                    take_damage: attack_info.damage,
                    set_should_pushback: source.source_type == HitSource::Character,
                    set_force: if airborne || attack_info.launcher {
                        Force::Airborne(source.facing.fix_collision(attack_info.air_force))
                    } else {
                        Force::Grounded(
                            source.facing.fix_collision(collision::Vec2::new(
                                attack_info.ground_pushback,
                                0_00,
                            )),
                        )
                    },
                },
            },
            HitType::Hit,
        )
    }

    pub fn build(
        attack_info: &AttackInfo,
        source: &Source,
        airborne: bool,
        current_combo: ComboEffect,
    ) -> (Effect, HitType) {
        let attack_info = &attack_info.on_hit;
        let damage = attack_info.damage * current_combo.proration / 100;
        (
            Effect {
                combo: ComboEffect {
                    available_limit: (current_combo.available_limit - attack_info.limit_cost)
                        .max(0),
                    hits: current_combo.hits + 1,
                    proration: current_combo.proration * attack_info.proration / 100,
                    total_damage: current_combo.total_damage + damage,
                    ground_action: attack_info.ground_action,
                },
                defender: DefenderEffect {
                    add_spirit_delay: attack_info.spirit_delay,
                    is_lethal: attack_info.lethal,
                    modify_meter: attack_info.defender_meter,
                    reset_spirit_delay: attack_info.reset_spirit_delay,
                    set_stop: attack_info.defender_stop,
                    take_spirit_gauge: attack_info.spirit_cost,
                    set_stun: if airborne || attack_info.launcher {
                        attack_info.air_stun
                    } else {
                        attack_info.stun
                    },
                    take_damage: attack_info.damage,
                    set_should_pushback: source.source_type == HitSource::Character,
                    set_force: if airborne || attack_info.launcher {
                        Force::Airborne(source.facing.fix_collision(attack_info.air_force))
                    } else {
                        Force::Grounded(
                            source.facing.fix_collision(collision::Vec2::new(
                                attack_info.ground_pushback,
                                0_00,
                            )),
                        )
                    },
                },
            },
            HitType::Hit,
        )
    }

    pub fn append_hit(mut self, attack_info: &AttackInfo, source: &Source) -> (Self, HitType) {
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

        if matches!(self.defender.set_force, Force::Grounded(..)) && attack_info.launcher {
            self.combo.ground_action = attack_info.ground_action;

            self.defender.set_force =
                Force::Airborne(source.facing.fix_collision(attack_info.air_force));
            self.defender.set_stun = attack_info.air_stun;
            self.defender.set_stop = attack_info.defender_stop;
            self.defender.set_should_pushback = source.source_type == HitSource::Character;
        } else if matches!(self.defender.set_force, Force::Grounded(..))
            || self.defender.set_stun < attack_info.stun
        {
            self.combo.ground_action = attack_info.ground_action;

            self.defender.set_force = Force::Grounded(
                source
                    .facing
                    .fix_collision(collision::Vec2::new(attack_info.ground_pushback, 0_00)),
            );
            self.defender.set_stun = attack_info.stun;
            self.defender.set_stop = attack_info.defender_stop;
            self.defender.set_should_pushback = source.source_type == HitSource::Character;
        } else if matches!(self.defender.set_force, Force::Airborne(..))
            || self.defender.set_stun < attack_info.air_stun
        {
            self.combo.ground_action = attack_info.ground_action;

            self.defender.set_force =
                Force::Airborne(source.facing.fix_collision(attack_info.air_force));
            self.defender.set_stun = attack_info.air_stun;
            self.defender.set_stop = attack_info.defender_stop;
            self.defender.set_should_pushback = source.source_type == HitSource::Character;
        }

        (self, HitType::Hit)
    }
}
