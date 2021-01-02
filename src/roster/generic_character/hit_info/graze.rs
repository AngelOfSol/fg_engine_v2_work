use super::{block, counter_hit, guard_crush, hit, wrong_block, HitType, Source};
use crate::character::components::AttackInfo;

pub struct Effect {
    pub defender: DefenderEffect,
}
pub struct DefenderEffect {
    pub take_damage: i32,
    pub take_spirit_gauge: i32,
    pub modify_meter: i32,
    pub add_spirit_delay: i32,
    pub reset_spirit_delay: bool,
    pub set_stop: i32,
}

impl Effect {
    pub fn build(attack_info: &AttackInfo) -> (Effect, HitType) {
        let graze_info = &attack_info.on_graze;
        (
            Effect {
                defender: DefenderEffect {
                    add_spirit_delay: graze_info.spirit_delay,
                    modify_meter: graze_info.defender_meter,
                    reset_spirit_delay: graze_info.reset_spirit_delay,
                    set_stop: graze_info.defender_stop,
                    take_spirit_gauge: graze_info.spirit_cost,
                    take_damage: graze_info.damage,
                },
            },
            HitType::Graze,
        )
    }
    pub fn append_block(
        self,
        attack_info: &AttackInfo,
        source: &Source,
        airborne: bool,
    ) -> (block::Effect, HitType) {
        let mut effect = block::Effect::build(attack_info, source, airborne);
        {
            let effect = &mut effect.0;

            effect.defender.modify_meter += self.defender.modify_meter;
            effect.defender.take_spirit_gauge += self.defender.take_spirit_gauge;
            effect.defender.take_damage += self.defender.take_damage;
        }

        effect
    }
    pub fn append_wrongblock(
        self,
        attack_info: &AttackInfo,
        source: &Source,
    ) -> (wrong_block::Effect, HitType) {
        let mut effect = wrong_block::Effect::build(attack_info, source);
        {
            let effect = &mut effect.0;

            effect.defender.modify_meter += self.defender.modify_meter;
            effect.defender.take_spirit_gauge += self.defender.take_spirit_gauge;
            effect.defender.take_damage += self.defender.take_damage;
        }

        effect
    }
    pub fn append_hit(
        self,
        attack_info: &AttackInfo,
        source: &Source,
        airborne: bool,
    ) -> (hit::Effect, HitType) {
        let mut effect = hit::Effect::build_starter(attack_info, source, airborne);
        {
            let effect = &mut effect.0;

            effect.defender.modify_meter += self.defender.modify_meter;
            effect.defender.take_spirit_gauge += self.defender.take_spirit_gauge;
            effect.defender.take_damage += self.defender.take_damage;
        }

        effect
    }
    pub fn append_graze(mut self, attack_info: &AttackInfo) -> (Self, HitType) {
        let graze_info = &attack_info.on_graze;

        self.defender.add_spirit_delay += graze_info.spirit_delay;
        self.defender.modify_meter += graze_info.defender_meter;
        self.defender.reset_spirit_delay |= graze_info.reset_spirit_delay;
        self.defender.set_stop += graze_info.defender_stop;
        self.defender.take_spirit_gauge += graze_info.spirit_cost;
        self.defender.take_damage += graze_info.damage;

        (self, HitType::Graze)
    }

    pub fn append_counterhit(
        self,
        attack_info: &AttackInfo,
        source: &Source,
        airborne: bool,
    ) -> (counter_hit::Effect, HitType) {
        let mut effect = counter_hit::Effect::build(attack_info, source, airborne);
        {
            let effect = &mut effect.0;

            effect.defender.modify_meter += self.defender.modify_meter;
            effect.defender.take_spirit_gauge += self.defender.take_spirit_gauge;
            effect.defender.take_damage += self.defender.take_damage;
        }

        effect
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
