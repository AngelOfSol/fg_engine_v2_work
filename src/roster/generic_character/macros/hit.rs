macro_rules! impl_would_be_hit {
    () => {};
}

macro_rules! impl_take_hit {
    (hitstun_air: $hitstun_air:expr, hitstun_ground: $hitstun_ground:expr,
        blockstun_air: $blockstun_air:expr, blockstun_stand: $blockstun_stand:expr, blockstun_crouch: $blockstun_crouch:expr,
        wrongblock_stand: $wrongblock_stand:expr, wrongblock_crouch: $wrongblock_crouch:expr) => {
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
                HitEffectType::Hit
                | HitEffectType::CounterHit
                | HitEffectType::GuardCrush
                | HitEffectType::GrazeCrush => {
                    if airborne {
                        self.state.current_state = (0, $hitstun_air);
                    } else {
                        self.state.current_state = (0, $hitstun_ground);
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
