macro_rules! impl_would_be_hit {
    () => {
        fn would_be_hit(
            &self,
            input: &[InputState],
            info: HitAction,
            old_effect: Option<HitEffect>,
        ) -> (Option<HitEffect>, Option<HitResult>) {
            let attack_info = &info.attack_info;
            let flags = self.current_flags();
            let state_type = self.data.states[&self.state.current_state.1].state_type;
            let axis = DirectedAxis::from_facing(input.last().unwrap().axis, self.state.facing);
            let new_hit_type = old_effect
                .as_ref()
                .and_then(|item| match item.hit_type {
                    HitEffectType::Hit
                    | HitEffectType::CounterHit
                    | HitEffectType::GuardCrush
                    | HitEffectType::GrazeCrush => {
                        Some(if item.effect.set_combo.unwrap().available_limit > 0 {
                            Some(HitEffectType::Hit)
                        } else {
                            None
                        })
                    }
                    HitEffectType::Block | HitEffectType::WrongBlock => {
                        Some(if attack_info.air_unblockable && flags.airborne {
                            Some(HitEffectType::Hit)
                        } else if flags.airborne || axis.is_guarding(attack_info.guard) {
                            Some(HitEffectType::Block)
                        } else {
                            Some(HitEffectType::WrongBlock)
                        })
                    }
                    HitEffectType::Graze => None,
                })
                .or_else(|| {
                    Some(
                        if attack_info.magic && flags.bullet.is_invuln()
                            || attack_info.melee && flags.melee.is_invuln()
                            || self
                                .state
                                .current_combo
                                .map(|item| item.available_limit <= 0)
                                .unwrap_or(false)
                        {
                            None
                        } else if attack_info.grazeable && flags.grazing {
                            Some(HitEffectType::Graze)
                        } else if (state_type.is_blockstun()
                            || (flags.can_block
                                // this is crossup protection
                                // if the attack is facing the same direction you're facing
                                // then the attack should be able to be blocked by holding both back
                                // and forward.
                                && (axis.is_blocking(false) || axis.is_blocking(self.state.facing == info.facing))))
                            && !(attack_info.air_unblockable && flags.airborne)
                        {
                            if flags.airborne || axis.is_guarding(attack_info.guard) {
                                Some(HitEffectType::Block)
                            } else {
                                Some(HitEffectType::WrongBlock)
                            }
                        } else if flags.can_be_counter_hit && attack_info.can_counter_hit {
                            Some(HitEffectType::CounterHit)
                        } else {
                            Some(HitEffectType::Hit)
                        },
                    )
                })
                .flatten();

            if new_hit_type.is_none() {
                return (old_effect, None);
            }

            let new_hit_type = new_hit_type.unwrap();
            let new_effect = match new_hit_type {
                HitEffectType::Graze => EffectData::graze(&info, flags.airborne).build(),
                HitEffectType::CounterHit => {
                    EffectData::counter_hit(&info, self.state.current_combo, flags.airborne).build()
                }
                HitEffectType::Block => EffectData::block(&info, flags.airborne).build(),
                HitEffectType::WrongBlock => EffectData::wrong_block(&info).build(),
                HitEffectType::Hit => {
                    EffectData::hit(&info, self.state.current_combo, flags.airborne).build()
                }
                HitEffectType::GuardCrush => unreachable!(),
                HitEffectType::GrazeCrush => unreachable!(),
            };

            let (new_effect, new_hit_type) = match old_effect {
                None => (new_effect, new_hit_type),
                Some(old_effect) => {
                    let old_hit_type = old_effect.hit_type;
                    let old_effect = old_effect.effect;
                    match old_hit_type {
                        HitEffectType::Graze => match new_hit_type {
                            HitEffectType::Graze => (
                                old_effect
                                    .into_builder()
                                    .inherit_non_hit_data(&new_effect)
                                    .set_stop(new_effect.set_stop.max(old_effect.set_stop))
                                    .build(),
                                old_hit_type,
                            ),
                            HitEffectType::CounterHit | HitEffectType::Hit => (
                                new_effect
                                    .into_builder()
                                    .inherit_non_hit_data(&old_effect)
                                    .inherit_spirit_delay(&old_effect)
                                    .build(),
                                new_hit_type,
                            ),
                            HitEffectType::Block | HitEffectType::WrongBlock => (
                                new_effect
                                    .into_builder()
                                    .inherit_non_hit_data(&old_effect)
                                    .build(),
                                new_hit_type,
                            ),
                            HitEffectType::GuardCrush | HitEffectType::GrazeCrush => unreachable!(),
                        },

                        HitEffectType::Hit
                        | HitEffectType::CounterHit
                        | HitEffectType::GuardCrush
                        | HitEffectType::GrazeCrush => match new_hit_type {
                            HitEffectType::Hit => {
                                assert!(old_effect.set_combo.unwrap().available_limit > 0);
                                (
                                    old_effect.into_builder().apply_hit(&info).build(),
                                    old_hit_type,
                                )
                            }
                            HitEffectType::GuardCrush
                            | HitEffectType::GrazeCrush
                            | HitEffectType::Block
                            | HitEffectType::WrongBlock
                            | HitEffectType::Graze
                            | HitEffectType::CounterHit => unreachable!(),
                        },
                        HitEffectType::Block => match new_hit_type {
                            HitEffectType::Hit => (
                                new_effect
                                    .into_builder()
                                    .inherit_non_hit_data(&old_effect)
                                    .inherit_spirit_delay(&old_effect)
                                    .build(),
                                new_hit_type,
                            ),
                            HitEffectType::Block => (
                                old_effect
                                    .into_builder()
                                    .inherit_non_hit_data(&new_effect)
                                    .build(),
                                old_hit_type,
                            ),
                            HitEffectType::WrongBlock => (
                                new_effect
                                    .into_builder()
                                    .inherit_non_hit_data(&old_effect)
                                    .build(),
                                new_hit_type,
                            ),
                            HitEffectType::GuardCrush
                            | HitEffectType::GrazeCrush
                            | HitEffectType::Graze
                            | HitEffectType::CounterHit => unreachable!(),
                        },
                        HitEffectType::WrongBlock => match new_hit_type {
                            HitEffectType::Hit => (
                                new_effect
                                    .into_builder()
                                    .inherit_non_hit_data(&old_effect)
                                    .inherit_spirit_delay(&old_effect)
                                    .build(),
                                new_hit_type,
                            ),
                            HitEffectType::Block | HitEffectType::WrongBlock => (
                                old_effect
                                    .into_builder()
                                    .inherit_non_hit_data(&new_effect)
                                    .build(),
                                old_hit_type,
                            ),
                            HitEffectType::GuardCrush
                            | HitEffectType::GrazeCrush
                            | HitEffectType::Graze
                            | HitEffectType::CounterHit => unreachable!(),
                        },
                    }
                }
            };

            let (effect, hit_type) = match new_hit_type {
                HitEffectType::Block | HitEffectType::WrongBlock
                    if self.state.spirit_gauge - new_effect.take_spirit_gauge <= 0 =>
                {
                    (
                        EffectData::guard_crush(&info, flags.airborne).build(),
                        HitEffectType::GuardCrush,
                    )
                }
                _ => (new_effect, new_hit_type),
            };

            (
                Some(HitEffect { hit_type, effect }),
                Some(HitResult {
                    hit_type,
                    action: info,
                }),
            )
        }
    };
}

macro_rules! impl_take_hit {
    (hitstun_air: $hitstun_air:expr, hitstun_ground: $hitstun_ground:expr,
        blockstun_air: $blockstun_air:expr, blockstun_stand: $blockstun_stand:expr, blockstun_crouch: $blockstun_crouch:expr,
        wrongblock_stand: $wrongblock_stand:expr, wrongblock_crouch: $wrongblock_crouch:expr, guard_crush_ground: $guard_crush_ground:expr, guard_crush_air: $guard_crush_air:expr) => {
        fn take_hit(&mut self, info: HitEffect, play_area: &PlayArea) {
            let flags = self.current_flags();

            let hit_type = info.hit_type;
            let effect = info.effect;

            let crouching = flags.crouching;

            self.state.health -= effect.take_damage;

            if self.state.health <= 0 && effect.is_lethal {
                self.state.dead = true;
            }

            let airborne = match effect.set_force {
                Force::Airborne(_) => true,
                Force::Grounded(_) => false,
            } || self.state.dead;

            self.state.should_pushback = effect.set_should_pushback;
            self.state.spirit_gauge -= effect.take_spirit_gauge;
            self.state.meter += effect.modify_meter;

            self.state.spirit_delay = if effect.reset_spirit_delay {
                0
            } else {
                self.state.spirit_delay
            } + effect.add_spirit_delay;

            self.state.hitstop = effect.set_stop;

            match hit_type {
                HitEffectType::Graze => {}
                _ => {
                    self.state.extra_data = ExtraData::Stun(effect.set_stun);
                    self.state.velocity = match effect.set_force {
                        Force::Airborne(value) | Force::Grounded(value) => value,
                    };
                }
            }

            self.state.current_combo = effect.set_combo;

            match hit_type {
                HitEffectType::Graze => (),
                HitEffectType::WrongBlock => {
                    if crouching {
                        self.state.current_state = (0, $wrongblock_crouch);
                    } else {
                        self.state.current_state = (0, $wrongblock_stand);
                    }
                }
                HitEffectType::Block => {
                    if airborne {
                        self.state.current_state = (0, $blockstun_air);
                    } else if crouching {
                        self.state.current_state = (0, $blockstun_crouch);
                    } else {
                        self.state.current_state = (0, $blockstun_stand);
                    }
                }
                HitEffectType::Hit | HitEffectType::CounterHit | HitEffectType::GrazeCrush => {
                    if airborne {
                        self.state.current_state = (0, $hitstun_air);
                    } else {
                        self.state.current_state = (0, $hitstun_ground);
                    }
                }
                HitEffectType::GuardCrush => {
                    if airborne {
                        self.state.current_state = (0, $guard_crush_air);
                    } else {
                        self.state.current_state = (0, $guard_crush_ground);
                    }
                }
            }

            self.validate_position(play_area);

            match hit_type {
                HitEffectType::GuardCrush => {
                    self.state.spirit_gauge = self.data.properties.max_spirit_gauge
                }
                _ => (),
            }
        }
    };
}

macro_rules! impl_deal_hit {
    (on_hit_particle: $on_hit_particle:expr) => {
        fn deal_hit(&mut self, info: &HitResult) {
            match info.hit_type {
                HitEffectType::Hit => {
                    self.state
                        .sound_state
                        .play_sound(ChannelName::Hit, GlobalSound::Hit.into());
                }
                HitEffectType::CounterHit => {
                    self.state
                        .sound_state
                        .play_sound(ChannelName::Hit, GlobalSound::CounterHit.into());
                }
                HitEffectType::GuardCrush => {
                    self.state
                        .sound_state
                        .play_sound(ChannelName::Hit, GlobalSound::GuardCrush.into());
                }
                HitEffectType::Block => {
                    self.state
                        .sound_state
                        .play_sound(ChannelName::Hit, GlobalSound::Block.into());
                }
                HitEffectType::WrongBlock => {
                    self.state
                        .sound_state
                        .play_sound(ChannelName::Hit, GlobalSound::WrongBlock.into());
                }
                _ => (),
            }
            match info.hit_type {
                HitEffectType::Hit => {
                    self.state.meter += info.action.attack_info.on_hit.attacker_meter
                }
                HitEffectType::CounterHit => {
                    self.state.meter += info.action.attack_info.on_counter_hit.attacker_meter
                }
                HitEffectType::GuardCrush => {
                    self.state.meter += info.action.attack_info.on_guard_crush.attacker_meter
                }
                HitEffectType::Graze => {
                    self.state.meter += info.action.attack_info.on_graze.attacker_meter
                }
                HitEffectType::Block => {
                    self.state.meter += info.action.attack_info.on_block.attacker_meter
                }
                HitEffectType::WrongBlock => {
                    self.state.meter += info.action.attack_info.on_wrongblock.attacker_meter
                }
                HitEffectType::GrazeCrush => {
                    //self.state.meter += info.action.attack_info.on_graze_crush.attacker_meter
                }
            }

            if info.action.source == HitSource::Character {
                self.state.last_hit_using = Some(info.action.hash);
            }

            match info.hit_type {
                HitEffectType::Hit
                | HitEffectType::CounterHit
                | HitEffectType::GuardCrush
                | HitEffectType::GrazeCrush => {
                    if info.action.source == HitSource::Character {
                        self.state.allowed_cancels = AllowedCancel::Hit;
                        self.state.hitstop = info.action.attack_info.on_hit.attacker_stop;
                    }
                }
                HitEffectType::Block | HitEffectType::WrongBlock => {
                    if info.action.source == HitSource::Character {
                        self.state.allowed_cancels = AllowedCancel::Block;
                        self.state.hitstop = info.action.attack_info.on_block.attacker_stop;
                    }
                }
                HitEffectType::Graze => {}
            }
        }
    };
}
