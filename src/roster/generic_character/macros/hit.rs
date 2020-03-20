macro_rules! impl_would_be_hit {
    () => {
        fn would_be_hit(
            &self,
            input: &[InputState],
            info: HitAction,
            effect: Option<HitEffect>,
        ) -> (Option<HitEffect>, Option<HitResult>) {
            let attack_info = &info.attack_info;
            let flags = self.current_flags();
            let state_type = self.data.states[&self.state.current_state.1].state_type;
            let axis = DirectedAxis::from_facing(input.last().unwrap().axis, self.state.facing);
            let hit_type = effect
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
                        if !attack_info.melee && flags.bullet.is_invuln()
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
            match hit_type {
                None => (effect, None),
                Some(HitEffectType::Graze) => match effect {
                    effect @ None
                    | effect
                    @
                    Some(HitEffect {
                        hit_type: HitEffectType::Graze,
                        ..
                    }) => (
                        effect,
                        Some(HitResult {
                            hit_type: HitEffectType::Graze,
                            action: info,
                        }),
                    ),
                    _ => unreachable!(),
                },
                Some(HitEffectType::CounterHit) => match effect {
                    None => {
                        let effect =
                            EffectData::counter_hit(&info, self.state.current_combo, flags.airborne)
                                .build();
                        (
                            Some(HitEffect {
                                hit_type: HitEffectType::CounterHit,
                                effect,
                            }),
                            Some(HitResult {
                                hit_type: HitEffectType::CounterHit,
                                action: info,
                            }),
                        )
                    }
                    Some(
                        effect
                        @
                        HitEffect {
                            hit_type: HitEffectType::Graze,
                            ..
                        },
                    ) => {
                        let effect =
                            EffectData::counter_hit(&info, self.state.current_combo, flags.airborne)
                                .take_spirit_gauge(effect.effect.take_spirit_gauge)
                                .take_damage(effect.effect.take_damage)
                                .build();
                        (
                            Some(HitEffect {
                                hit_type: HitEffectType::CounterHit,
                                effect,
                            }),
                            Some(HitResult {
                                hit_type: HitEffectType::CounterHit,
                                action: info,
                            }),
                        )
                    }
                    _ => unreachable!(),
                },
                Some(HitEffectType::Block) => match effect {
                    None => {
                        if self.state.spirit_gauge - attack_info.spirit_cost <= 0 {
                            let effect = EffectData::guard_crush(&info, flags.airborne).build();
                            (
                                Some(HitEffect {
                                    hit_type: HitEffectType::GuardCrush,
                                    effect,
                                }),
                                Some(HitResult {
                                    hit_type: HitEffectType::GuardCrush,
                                    action: info,
                                }),
                            )
                        } else {
                            let effect = EffectData::block(&info, flags.airborne).build();
                            (
                                Some(HitEffect {
                                    hit_type: HitEffectType::Block,
                                    effect,
                                }),
                                Some(HitResult {
                                    hit_type: HitEffectType::Block,
                                    action: info,
                                }),
                            )
                        }
                    }
                    Some(
                        old_effect
                        @
                        HitEffect {
                            hit_type:
                                HitEffectType::Block | HitEffectType::WrongBlock | HitEffectType::Graze,
                            ..
                        },
                    ) => {
                        let effect = old_effect
                            .effect
                            .into_builder()
                            .take_spirit_gauge(info.attack_info.spirit_cost)
                            .take_damage(info.attack_info.chip_damage)
                            .build();

                        if self.state.spirit_gauge - effect.take_spirit_gauge <= 0 {
                            let effect = EffectData::guard_crush(&info, flags.airborne)
                                .take_damage(old_effect.effect.take_damage)
                                .build();
                            (
                                Some(HitEffect {
                                    hit_type: HitEffectType::GuardCrush,
                                    effect,
                                }),
                                Some(HitResult {
                                    hit_type: HitEffectType::GuardCrush,
                                    action: info,
                                }),
                            )
                        } else {
                            let hit_type = old_effect.hit_type;
                            (
                                Some(HitEffect { hit_type, effect }),
                                Some(HitResult {
                                    hit_type: HitEffectType::Block,
                                    action: info,
                                }),
                            )
                        }
                    }
                    Some(HitEffect {
                        hit_type:
                            HitEffectType::Hit
                            | HitEffectType::CounterHit
                            | HitEffectType::GuardCrush
                            | HitEffectType::GrazeCrush,
                        ..
                    }) => unreachable!(),
                },
                Some(HitEffectType::WrongBlock) => match effect {
                    None => {
                        // TODO write getters for attaack_info for block_cost and wrongblock_cost
                        if self.state.spirit_gauge - attack_info.level.wrongblock_cost() <= 0 {
                            let effect = EffectData::guard_crush(&info, flags.airborne).build();
                            (
                                Some(HitEffect {
                                    hit_type: HitEffectType::GuardCrush,
                                    effect,
                                }),
                                Some(HitResult {
                                    hit_type: HitEffectType::GuardCrush,
                                    action: info,
                                }),
                            )
                        } else {
                            let effect = EffectData::wrong_block(&info, flags.airborne).build();
                            (
                                Some(HitEffect {
                                    hit_type: HitEffectType::WrongBlock,
                                    effect,
                                }),
                                Some(HitResult {
                                    hit_type: HitEffectType::WrongBlock,
                                    action: info,
                                }),
                            )
                        }
                    }
                    Some(
                        old_effect
                        @
                        HitEffect {
                            hit_type: HitEffectType::Block | HitEffectType::Graze,
                            ..
                        },
                    ) => {
                        let effect = EffectData::wrong_block(&info, flags.airborne)
                            .take_spirit_gauge(old_effect.effect.take_spirit_gauge)
                            .take_damage(old_effect.effect.take_damage)
                            .build();

                        if self.state.spirit_gauge - effect.take_spirit_gauge <= 0 {
                            let effect = EffectData::guard_crush(&info, flags.airborne)
                                .take_damage(old_effect.effect.take_damage)
                                .build();
                            (
                                Some(HitEffect {
                                    hit_type: HitEffectType::GuardCrush,
                                    effect,
                                }),
                                Some(HitResult {
                                    hit_type: HitEffectType::GuardCrush,
                                    action: info,
                                }),
                            )
                        } else {
                            (
                                Some(HitEffect {
                                    hit_type: HitEffectType::WrongBlock,
                                    effect,
                                }),
                                Some(HitResult {
                                    hit_type: HitEffectType::WrongBlock,
                                    action: info,
                                }),
                            )
                        }
                    }
                    Some(
                        old_effect
                        @
                        HitEffect {
                            hit_type: HitEffectType::WrongBlock,
                            ..
                        },
                    ) => {
                        let effect = old_effect
                            .effect
                            .into_builder()
                            .take_spirit_gauge(info.attack_info.spirit_cost)
                            .take_damage(info.attack_info.chip_damage)
                            .build();

                        if self.state.spirit_gauge - effect.take_spirit_gauge <= 0 {
                            let effect = EffectData::guard_crush(&info, flags.airborne)
                                .take_damage(old_effect.effect.take_damage)
                                .build();
                            (
                                Some(HitEffect {
                                    hit_type: HitEffectType::GuardCrush,
                                    effect,
                                }),
                                Some(HitResult {
                                    hit_type: HitEffectType::GuardCrush,
                                    action: info,
                                }),
                            )
                        } else {
                            (
                                Some(HitEffect {
                                    hit_type: HitEffectType::WrongBlock,
                                    effect,
                                }),
                                Some(HitResult {
                                    hit_type: HitEffectType::WrongBlock,
                                    action: info,
                                }),
                            )
                        }
                    }
                    Some(HitEffect {
                        hit_type:
                            HitEffectType::Hit
                            | HitEffectType::CounterHit
                            | HitEffectType::GuardCrush
                            | HitEffectType::GrazeCrush,
                        ..
                    }) => unreachable!(),
                },
                Some(HitEffectType::Hit) => match effect {
                    None => {
                        let effect =
                            EffectData::hit(&info, self.state.current_combo, flags.airborne).build();
                        (
                            Some(HitEffect {
                                hit_type: HitEffectType::Hit,
                                effect,
                            }),
                            Some(HitResult {
                                hit_type: HitEffectType::Hit,
                                action: info,
                            }),
                        )
                    }
                    Some(
                        effect
                        @
                        HitEffect {
                            hit_type:
                                HitEffectType::GrazeCrush
                                | HitEffectType::GuardCrush
                                | HitEffectType::CounterHit
                                | HitEffectType::Hit,
                            ..
                        },
                    ) => {
                        assert!(effect.effect.set_combo.unwrap().available_limit > 0);
                        let hit_type = effect.hit_type;
                        let effect = effect.effect.into_builder().apply_hit(&info).build();
                        (
                            Some(HitEffect { hit_type, effect }),
                            Some(HitResult {
                                hit_type: HitEffectType::Hit,
                                action: info,
                            }),
                        )
                    }
                    Some(
                        effect
                        @
                        HitEffect {
                            hit_type:
                                HitEffectType::Block | HitEffectType::WrongBlock | HitEffectType::Graze,
                            ..
                        },
                    ) => {
                        let effect = EffectData::hit(&info, None, flags.airborne)
                            .inherit_non_hit_data(&effect.effect)
                            .build();

                        (
                            Some(HitEffect {
                                hit_type: HitEffectType::Hit,
                                effect,
                            }),
                            Some(HitResult {
                                hit_type: HitEffectType::Hit,
                                action: info,
                            }),
                        )
                    }
                },
                Some(HitEffectType::GuardCrush) => unreachable!(),
                Some(HitEffectType::GrazeCrush) => unreachable!(),
            }
        }

    };
}

macro_rules! impl_take_hit {
    (hitstun_air: $hitstun_air:expr, hitstun_ground: $hitstun_ground:expr,
        blockstun_air: $blockstun_air:expr, blockstun_stand: $blockstun_stand:expr, blockstun_crouch: $blockstun_crouch:expr,
        wrongblock_stand: $wrongblock_stand:expr, wrongblock_crouch: $wrongblock_crouch:expr, guard_crush_ground: $guard_crush_ground:expr, guard_crush_air: $guard_crush_air:expr) => {
        fn take_hit(&mut self, info: HitEffect) {
            let flags = self.current_flags();

            let hit_type = info.hit_type;
            let effect = info.effect;
            let airborne = match effect.set_force {
                Force::Airborne(_) => true,
                Force::Grounded(_) => false,
            };
            let crouching = flags.crouching;

            self.state.health -= effect.take_damage;
            self.state.should_pushback = effect.set_should_pushback;
            self.state.spirit_gauge -= effect.take_spirit_gauge;
            self.state.spirit_delay = if effect.reset_spirit_delay {
                0
            } else {
                self.state.spirit_delay
            } + effect.add_spirit_delay;
            match hit_type {
                HitEffectType::Graze => {}
                _ => {
                    self.state.hitstop = effect.set_stop;
                    self.state.extra_data = ExtraData::Stun(effect.set_stun);
                    self.state.velocity = match effect.set_force {
                        Force::Airborne(value) | Force::Grounded(value) => value,
                    };
                }
            }
            match hit_type {
                HitEffectType::Graze => {}
                _ => {
                    self.state.hitstop = effect.set_stop;
                    self.state.extra_data = ExtraData::Stun(effect.set_stun);
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

            match hit_type {
                HitEffectType::GuardCrush => {
                    self.crush_orb();
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
                HitEffectType::Hit
                | HitEffectType::CounterHit
                | HitEffectType::GuardCrush
                | HitEffectType::GrazeCrush => {
                    if info.action.source == HitSource::Character {
                        self.state.last_hit_using = Some(info.action.hash);
                        self.state.allowed_cancels = AllowedCancel::Hit;
                        self.state.hitstop = info.action.attack_info.on_hit.attacker_stop;
                    }
                }
                HitEffectType::Block | HitEffectType::WrongBlock => {
                    if info.action.source == HitSource::Character {
                        self.state.last_hit_using = Some(info.action.hash);
                        self.state.allowed_cancels = AllowedCancel::Block;
                        self.state.hitstop = info.action.attack_info.on_block.attacker_stop;
                    }
                }
                HitEffectType::Graze => {
                    if info.action.source == HitSource::Character {
                        self.state.last_hit_using = Some(info.action.hash);
                    }
                }
            }
        }
    };
}
