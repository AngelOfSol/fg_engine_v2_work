use crate::{
    game_match::PlayArea,
    roster::{
        character::{
            data::Data,
            player_state::PlayerState,
            typedefs::{state::StateConsts, Character, Timed},
        },
        hit_info::{block, counter_hit, graze, guard_crush, hit, wrong_block, Force, HitEffect},
    },
};

impl<C: Character> PlayerState<C> {
    pub fn take_hit(&mut self, data: &Data<C>, info: &HitEffect, play_area: &PlayArea) {
        let airborne = match info {
            HitEffect::Hit(hit::Effect {
                defender:
                    hit::DefenderEffect {
                        take_damage,
                        is_lethal,
                        set_force,
                        modify_meter,
                        set_stun,
                        set_stop,
                        set_should_pushback,
                        ..
                    },
                ..
            })
            | HitEffect::CounterHit(counter_hit::Effect {
                defender:
                    counter_hit::DefenderEffect {
                        take_damage,
                        is_lethal,
                        set_force,
                        modify_meter,
                        set_stun,
                        set_stop,
                        set_should_pushback,
                        ..
                    },
                ..
            })
            | HitEffect::GuardCrush(guard_crush::Effect {
                defender:
                    guard_crush::DefenderEffect {
                        take_damage,
                        is_lethal,
                        set_force,
                        modify_meter,
                        set_stun,
                        set_stop,
                        set_should_pushback,
                        ..
                    },
                ..
            })
            | HitEffect::Block(block::Effect {
                defender:
                    block::DefenderEffect {
                        take_damage,
                        is_lethal,
                        set_force,
                        modify_meter,
                        set_stun,
                        set_stop,
                        set_should_pushback,
                        ..
                    },
                ..
            })
            | HitEffect::WrongBlock(wrong_block::Effect {
                defender:
                    wrong_block::DefenderEffect {
                        take_damage,
                        is_lethal,
                        set_force,
                        modify_meter,
                        set_stun,
                        set_stop,
                        set_should_pushback,
                        ..
                    },
                ..
            }) => {
                self.health -= take_damage;

                if self.health <= 0 && *is_lethal {
                    self.dead = true;
                }

                let airborne = match set_force {
                    Force::Airborne(_) => true,
                    Force::Grounded(_) => false,
                } || self.dead;

                self.should_pushback = *set_should_pushback;
                self.meter += modify_meter;

                self.hitstop = *set_stop;

                self.stun = Some(*set_stun);
                self.velocity = match set_force {
                    Force::Airborne(value) | Force::Grounded(value) => *value,
                };

                airborne
            }

            HitEffect::Graze(effect) => {
                let effect = &effect.defender;

                self.health -= effect.take_damage;

                self.hitstop = effect.set_stop;
                data.get(self).flags.airborne
            }
        };

        match info {
            HitEffect::Hit(hit::Effect { combo, .. })
            | HitEffect::GuardCrush(guard_crush::Effect { combo, .. })
            | HitEffect::CounterHit(counter_hit::Effect { combo, .. }) => {
                self.current_combo = Some(combo.clone());
            }
            _ => {}
        }

        match info {
            HitEffect::Hit(hit::Effect {
                defender:
                    hit::DefenderEffect {
                        take_spirit_gauge,
                        reset_spirit_delay,
                        add_spirit_delay,
                        ..
                    },
                ..
            })
            | HitEffect::CounterHit(counter_hit::Effect {
                defender:
                    counter_hit::DefenderEffect {
                        take_spirit_gauge,
                        reset_spirit_delay,
                        add_spirit_delay,
                        ..
                    },
                ..
            })
            | HitEffect::Graze(graze::Effect {
                defender:
                    graze::DefenderEffect {
                        take_spirit_gauge,
                        reset_spirit_delay,
                        add_spirit_delay,
                        ..
                    },
                ..
            })
            | HitEffect::Block(block::Effect {
                defender:
                    block::DefenderEffect {
                        take_spirit_gauge,
                        reset_spirit_delay,
                        add_spirit_delay,
                        ..
                    },
                ..
            })
            | HitEffect::WrongBlock(wrong_block::Effect {
                defender:
                    wrong_block::DefenderEffect {
                        take_spirit_gauge,
                        reset_spirit_delay,
                        add_spirit_delay,
                        ..
                    },
                ..
            }) => {
                self.spirit_gauge -= *take_spirit_gauge;
                self.spirit_delay = if *reset_spirit_delay {
                    0
                } else {
                    self.spirit_delay
                } + *add_spirit_delay;
            }
            HitEffect::GuardCrush(_) => {
                self.spirit_gauge = data.properties.max_spirit_gauge;
            }
        }

        match info {
            HitEffect::GuardCrush(_) => {
                self.current_state = if airborne {
                    Timed {
                        time: 0,
                        id: C::State::AIR_HITSTUN,
                    }
                } else {
                    Timed {
                        time: 0,
                        id: C::State::GUARD_CRUSH,
                    }
                }
            }
            HitEffect::Hit(_) | HitEffect::CounterHit(_) => {
                self.current_state = if airborne {
                    Timed {
                        time: 0,
                        id: C::State::AIR_HITSTUN,
                    }
                } else {
                    Timed {
                        time: 0,
                        id: C::State::GROUND_HITSTUN,
                    }
                }
            }
            HitEffect::Block(_) => {
                self.current_state = if airborne {
                    Timed {
                        time: 0,
                        id: C::State::AIR_BLOCKSTUN,
                    }
                } else if data.get(self).flags.crouching {
                    Timed {
                        time: 0,
                        id: C::State::CROUCH_BLOCKSTUN,
                    }
                } else {
                    Timed {
                        time: 0,
                        id: C::State::STAND_BLOCKSTUN,
                    }
                }
            }
            HitEffect::WrongBlock(_) => {
                self.current_state = if data.get(self).flags.crouching {
                    Timed {
                        time: 0,
                        id: C::State::CROUCH_WRONG_BLOCKSTUN,
                    }
                } else {
                    Timed {
                        time: 0,
                        id: C::State::STAND_WRONG_BLOCKSTUN,
                    }
                }
            }
            HitEffect::Graze(_) => {}
        }

        self.validate_position(data, play_area);
    }
}
