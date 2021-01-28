use super::{guard_crush, HitType, Source};
use crate::{
    character::components::AttackInfo,
    roster::hit_info::{Force, HitSource},
};
use fg_datastructures::math::collision;

pub struct Effect {
    pub defender: DefenderEffect,
}
pub struct DefenderEffect {
    pub take_damage: i32,
    pub take_spirit_gauge: i32,
    pub modify_meter: i32,
    pub add_spirit_delay: i32,
    pub reset_spirit_delay: bool,
    pub set_stun: i32,
    pub set_force: Force,
    pub set_stop: i32,
    pub is_lethal: bool,
    pub set_should_pushback: bool,
}

impl Effect {
    pub fn would_crush(
        previous: Option<i32>,
        attack_info: &AttackInfo,
        remaining_spirit: i32,
    ) -> bool {
        previous.unwrap_or(0) + attack_info.on_wrongblock.spirit_cost >= remaining_spirit
    }
    pub fn build(attack_info: &AttackInfo, source: &Source) -> (Effect, HitType) {
        let block_info = &attack_info.on_wrongblock;
        (
            Effect {
                defender: DefenderEffect {
                    is_lethal: false,
                    add_spirit_delay: block_info.spirit_delay,
                    modify_meter: block_info.defender_meter,
                    reset_spirit_delay: block_info.reset_spirit_delay,
                    set_stop: block_info.defender_stop,
                    take_spirit_gauge: block_info.spirit_cost,
                    set_stun: block_info.stun,
                    take_damage: block_info.damage,
                    set_should_pushback: source.source_type == HitSource::Character,
                    set_force: Force::Grounded(
                        source
                            .facing
                            .fix(collision::Vec2::new(block_info.ground_pushback, 0_00)),
                    ),
                },
            },
            HitType::WrongBlock,
        )
    }

    pub fn append_block(mut self, attack_info: &AttackInfo) -> (Self, HitType) {
        let block_info = &attack_info.on_block;

        self.defender.add_spirit_delay += block_info.spirit_delay;
        self.defender.modify_meter += block_info.defender_meter;
        self.defender.reset_spirit_delay |= block_info.reset_spirit_delay;
        self.defender.take_spirit_gauge += block_info.spirit_cost;
        self.defender.take_damage += block_info.damage;

        (self, HitType::Block)
    }

    pub fn append_wrongblock(
        mut self,
        attack_info: &AttackInfo,
        source: &Source,
    ) -> (Self, HitType) {
        let block_info = &attack_info.on_wrongblock;

        self.defender.add_spirit_delay += block_info.spirit_delay;
        self.defender.modify_meter += block_info.defender_meter;
        self.defender.reset_spirit_delay |= block_info.reset_spirit_delay;
        self.defender.take_spirit_gauge += block_info.spirit_cost;
        self.defender.take_damage += block_info.damage;

        if self.defender.set_stun < block_info.stun {
            self.defender.set_force = Force::Grounded(
                source
                    .facing
                    .fix(collision::Vec2::new(block_info.ground_pushback, 0_00)),
            );
            self.defender.set_stun = block_info.stun;
            self.defender.set_stop = block_info.defender_stop;
            self.defender.set_should_pushback = source.source_type == HitSource::Character;
        }

        (self, HitType::WrongBlock)
    }
    pub fn append_guard_crush(
        self,
        attack_info: &AttackInfo,
        source: &Source,
        airborne: bool,
    ) -> (guard_crush::Effect, HitType) {
        let mut effect = guard_crush::Effect::build(attack_info, source, airborne);
        {
            let effect = &mut effect.0;

            effect.defender.modify_meter += self.defender.modify_meter;
            effect.defender.take_damage += self.defender.take_damage;
        }
        effect
    }
}
