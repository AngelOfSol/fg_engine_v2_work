use crate::{
    character::{components::AttackInfo, state::components::StateType},
    input::{DirectedAxis, InputState},
    roster::{
        character::{data::Data, player_state::PlayerState, typedefs::Character},
        hit_info::{
            block, counter_hit, graze, guard_crush, hit, wrong_block, HitEffect, HitResult, Source,
        },
    },
};

impl<C: Character> PlayerState<C> {
    pub fn would_be_hit(
        &self,
        data: &Data<C>,
        input: &[InputState],
        attack_info: &AttackInfo,
        source: &Source,
        old_effect: Option<HitEffect>,
    ) -> HitResult {
        let state_data = data.get(self);
        let axis = DirectedAxis::from_facing(input.last().unwrap().axis, self.facing);
        match old_effect {
            Some(effect) => match effect {
                HitEffect::Hit(effect) => {
                    if effect.combo.available_limit > 0 {
                        effect.append_hit(attack_info, source).into()
                    } else {
                        effect.into()
                    }
                }
                HitEffect::GuardCrush(effect) => {
                    if effect.combo.available_limit > 0 {
                        effect.append_hit(attack_info).into()
                    } else {
                        effect.into()
                    }
                }
                HitEffect::CounterHit(effect) => {
                    if effect.combo.available_limit > 0 {
                        effect.append_hit(attack_info).into()
                    } else {
                        effect.into()
                    }
                }
                HitEffect::Graze(effect) => {
                    if attack_info.magic && state_data.flags.bullet.is_invuln()
                        || attack_info.melee && state_data.flags.melee.is_invuln()
                        || attack_info.air && state_data.flags.air.is_invuln()
                        || attack_info.foot && state_data.flags.foot.is_invuln()
                    {
                        effect.into()
                    } else if attack_info.grazeable {
                        effect.append_graze(attack_info).into()
                    } else if (state_data.flags.can_block
                        && (axis.is_blocking(false)
                            || axis.is_blocking(self.facing == source.facing)))
                        && !(attack_info.air_unblockable && state_data.flags.airborne)
                    {
                        if state_data.flags.airborne || axis.is_guarding(attack_info.guard) {
                            if block::Effect::would_crush(
                                Some(effect.defender.take_spirit_gauge),
                                attack_info,
                                self.spirit_gauge,
                            ) {
                                effect
                                    .append_guard_crush(
                                        attack_info,
                                        source,
                                        state_data.flags.airborne,
                                    )
                                    .into()
                            } else {
                                effect
                                    .append_block(attack_info, source, state_data.flags.airborne)
                                    .into()
                            }
                        } else if wrong_block::Effect::would_crush(
                            Some(effect.defender.take_spirit_gauge),
                            attack_info,
                            self.spirit_gauge,
                        ) {
                            effect
                                .append_guard_crush(attack_info, source, state_data.flags.airborne)
                                .into()
                        } else {
                            effect.append_wrongblock(attack_info, source).into()
                        }
                    } else if state_data.flags.can_be_counter_hit && attack_info.can_counter_hit {
                        effect
                            .append_counterhit(attack_info, source, state_data.flags.airborne)
                            .into()
                    } else {
                        effect
                            .append_hit(attack_info, source, state_data.flags.airborne)
                            .into()
                    }
                }
                HitEffect::Block(effect) => {
                    if !(attack_info.air_unblockable && state_data.flags.airborne) {
                        if state_data.flags.airborne || axis.is_guarding(attack_info.guard) {
                            if block::Effect::would_crush(
                                Some(effect.defender.take_spirit_gauge),
                                attack_info,
                                self.spirit_gauge,
                            ) {
                                effect
                                    .append_guard_crush(
                                        attack_info,
                                        source,
                                        state_data.flags.airborne,
                                    )
                                    .into()
                            } else {
                                effect
                                    .append_block(attack_info, source, state_data.flags.airborne)
                                    .into()
                            }
                        } else if wrong_block::Effect::would_crush(
                            Some(effect.defender.take_spirit_gauge),
                            attack_info,
                            self.spirit_gauge,
                        ) {
                            effect
                                .append_guard_crush(attack_info, source, state_data.flags.airborne)
                                .into()
                        } else {
                            effect.append_wrongblock(attack_info, source).into()
                        }
                    } else {
                        effect
                            .append_hit(attack_info, source, state_data.flags.airborne)
                            .into()
                    }
                }
                HitEffect::WrongBlock(effect) => {
                    if axis.is_guarding(attack_info.guard) {
                        if block::Effect::would_crush(
                            Some(effect.defender.take_spirit_gauge),
                            attack_info,
                            self.spirit_gauge,
                        ) {
                            effect
                                .append_guard_crush(attack_info, source, state_data.flags.airborne)
                                .into()
                        } else {
                            effect.append_block(attack_info).into()
                        }
                    } else if wrong_block::Effect::would_crush(
                        Some(effect.defender.take_spirit_gauge),
                        attack_info,
                        self.spirit_gauge,
                    ) {
                        effect
                            .append_guard_crush(attack_info, source, state_data.flags.airborne)
                            .into()
                    } else {
                        effect.append_wrongblock(attack_info, source).into()
                    }
                }
            },
            None => {
                if attack_info.magic && state_data.flags.bullet.is_invuln()
                    || attack_info.melee && state_data.flags.melee.is_invuln()
                    || attack_info.air && state_data.flags.air.is_invuln()
                    || attack_info.foot && state_data.flags.foot.is_invuln()
                    || self
                        .current_combo
                        .as_ref()
                        .map(|item| item.available_limit <= 0)
                        .unwrap_or(false)
                {
                    HitResult::None
                } else if attack_info.grazeable && state_data.flags.grazing {
                    graze::Effect::build(attack_info).into()
                } else if (matches!(state_data.state_type, StateType::Blockstun)
                    || (state_data.flags.can_block
                    // this is crossup protection
                    // if the attack is facing the same direction you're facing
                    // then the attack should be able to be blocked by holding both back
                    // and forward.
                    && (axis.is_blocking(false) || axis.is_blocking(self.facing == source.facing))))
                    && !(attack_info.air_unblockable && state_data.flags.airborne)
                {
                    if state_data.flags.airborne || axis.is_guarding(attack_info.guard) {
                        if block::Effect::would_crush(None, attack_info, self.spirit_gauge) {
                            guard_crush::Effect::build(
                                attack_info,
                                source,
                                state_data.flags.airborne,
                            )
                            .into()
                        } else {
                            block::Effect::build(attack_info, source, state_data.flags.airborne)
                                .into()
                        }
                    } else if wrong_block::Effect::would_crush(None, attack_info, self.spirit_gauge)
                    {
                        guard_crush::Effect::build(attack_info, source, state_data.flags.airborne)
                            .into()
                    } else {
                        wrong_block::Effect::build(attack_info, source).into()
                    }
                } else if state_data.flags.can_be_counter_hit && attack_info.can_counter_hit {
                    counter_hit::Effect::build(attack_info, source, state_data.flags.airborne)
                        .into()
                } else {
                    self.current_combo
                        .as_ref()
                        .map(|combo| {
                            if combo.available_limit > 0 {
                                hit::Effect::build(
                                    attack_info,
                                    source,
                                    state_data.flags.airborne,
                                    combo.clone(),
                                )
                                .into()
                            } else {
                                HitResult::None
                            }
                        })
                        .unwrap_or_else(|| {
                            hit::Effect::build_starter(
                                attack_info,
                                source,
                                state_data.flags.airborne,
                            )
                            .into()
                        })
                }
            }
        }
    }
}
