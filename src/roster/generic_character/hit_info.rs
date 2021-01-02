#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub enum HitSource {
    Character,
    #[allow(dead_code)]
    Object,
}

// action -> would_hit
// effect -> deal_hit/take_hit
// result -> repond_hit

use crate::typedefs::collision::Vec2;
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Force {
    Grounded(Vec2),
    Airborne(Vec2),
}

pub mod new {
    use crate::{character::components::GroundAction, input::Facing};

    use super::HitSource;

    #[derive(Debug, Clone)]
    pub struct ComboEffect {
        pub hits: u32,
        pub total_damage: i32,
        pub proration: i32,
        pub available_limit: i32,
        pub ground_action: GroundAction,
    }

    pub struct AttackerEffect {
        pub modify_meter: i32,
        pub set_stop: i32,
    }
    pub struct Source {
        pub source_type: HitSource,
        pub facing: Facing,
    }

    pub enum OnHitType {
        Hit,
        GuardCrush,
        Graze,
        CounterHit,
        Block,
        WrongBlock,
    }

    pub enum HitResultNew {
        None,
        Pass(OnHitEffect),
        HitBy(OnHitType, OnHitEffect),
    }

    impl<F> From<(F, OnHitType)> for HitResultNew
    where
        OnHitEffect: From<F>,
    {
        fn from((effect, hit_type): (F, OnHitType)) -> Self {
            Self::HitBy(hit_type, OnHitEffect::from(effect))
        }
    }

    impl<F> From<F> for HitResultNew
    where
        OnHitEffect: From<F>,
    {
        fn from(effect: F) -> Self {
            Self::Pass(OnHitEffect::from(effect))
        }
    }
    impl From<Option<OnHitEffect>> for HitResultNew {
        fn from(value: Option<OnHitEffect>) -> Self {
            match value {
                Some(effect) => Self::Pass(effect),
                None => Self::None,
            }
        }
    }

    impl HitResultNew {
        pub fn split(self) -> (Option<OnHitEffect>, Option<OnHitType>) {
            match self {
                Self::None => (None, None),
                Self::Pass(effect) => (Some(effect), None),
                Self::HitBy(hit_type, effect) => (Some(effect), Some(hit_type)),
            }
        }
    }

    pub enum OnHitEffect {
        Hit(hit::Effect),
        GuardCrush(guard_crush::Effect),
        CounterHit(counter_hit::Effect),
        Graze(graze::Effect),
        Block(block::Effect),
        WrongBlock(wrong_block::Effect),
    }

    impl From<hit::Effect> for OnHitEffect {
        fn from(value: hit::Effect) -> Self {
            Self::Hit(value)
        }
    }
    impl From<guard_crush::Effect> for OnHitEffect {
        fn from(value: guard_crush::Effect) -> Self {
            Self::GuardCrush(value)
        }
    }
    impl From<counter_hit::Effect> for OnHitEffect {
        fn from(value: counter_hit::Effect) -> Self {
            Self::CounterHit(value)
        }
    }
    impl From<graze::Effect> for OnHitEffect {
        fn from(value: graze::Effect) -> Self {
            Self::Graze(value)
        }
    }
    impl From<block::Effect> for OnHitEffect {
        fn from(value: block::Effect) -> Self {
            Self::Block(value)
        }
    }
    impl From<wrong_block::Effect> for OnHitEffect {
        fn from(value: wrong_block::Effect) -> Self {
            Self::WrongBlock(value)
        }
    }

    pub mod hit {
        use crate::{
            character::components::AttackInfo,
            roster::hit_info::{Force, HitSource},
            typedefs::collision,
        };

        use super::{AttackerEffect, ComboEffect, OnHitType, Source};
        pub struct Effect {
            pub attacker: AttackerEffect,
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
            ) -> (Effect, OnHitType) {
                let attack_info = &attack_info.on_hit;
                (
                    Effect {
                        attacker: AttackerEffect {
                            modify_meter: attack_info.attacker_meter,
                            set_stop: if source.source_type == HitSource::Character {
                                attack_info.attacker_stop
                            } else {
                                0
                            },
                        },

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
                                Force::Grounded(source.facing.fix_collision(collision::Vec2::new(
                                    attack_info.ground_pushback,
                                    0_00,
                                )))
                            },
                        },
                    },
                    OnHitType::Hit,
                )
            }

            pub fn build(
                attack_info: &AttackInfo,
                source: &Source,
                airborne: bool,
                current_combo: ComboEffect,
            ) -> (Effect, OnHitType) {
                let attack_info = &attack_info.on_hit;
                let damage = attack_info.damage * current_combo.proration / 100;
                (
                    Effect {
                        attacker: AttackerEffect {
                            modify_meter: attack_info.attacker_meter,
                            set_stop: if source.source_type == HitSource::Character {
                                attack_info.attacker_stop
                            } else {
                                0
                            },
                        },

                        combo: ComboEffect {
                            available_limit: (current_combo.available_limit
                                - attack_info.limit_cost)
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
                                Force::Grounded(source.facing.fix_collision(collision::Vec2::new(
                                    attack_info.ground_pushback,
                                    0_00,
                                )))
                            },
                        },
                    },
                    OnHitType::Hit,
                )
            }

            pub fn append_hit(
                mut self,
                attack_info: &AttackInfo,
                source: &Source,
            ) -> (Self, OnHitType) {
                let attack_info = &attack_info.on_hit;
                let damage = attack_info.damage * self.combo.proration / 100;

                self.attacker.modify_meter += attack_info.attacker_meter;

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

                (self, OnHitType::Hit)
            }
        }
    }

    pub mod guard_crush {
        use crate::{
            character::components::AttackInfo,
            roster::hit_info::{Force, HitSource},
            typedefs::collision,
        };

        use super::{AttackerEffect, ComboEffect, OnHitType, Source};
        pub struct Effect {
            pub attacker: AttackerEffect,
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
            pub fn build(
                attack_info: &AttackInfo,
                source: &Source,
                airborne: bool,
            ) -> (Effect, OnHitType) {
                let guard_crush_info = &attack_info.on_guard_crush;
                (
                    Effect {
                        attacker: AttackerEffect {
                            modify_meter: guard_crush_info.attacker_meter,
                            set_stop: if source.source_type == HitSource::Character {
                                guard_crush_info.attacker_stop
                            } else {
                                0
                            },
                        },

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
                                Force::Airborne(
                                    source.facing.fix_collision(guard_crush_info.air_force),
                                )
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

                self.attacker.modify_meter += attack_info.attacker_meter;

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
    }

    pub mod counter_hit {
        use crate::{
            character::components::AttackInfo,
            roster::hit_info::{Force, HitSource},
            typedefs::collision,
        };

        use super::{AttackerEffect, ComboEffect, OnHitType, Source};
        pub struct Effect {
            pub attacker: AttackerEffect,
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
            pub fn build(
                attack_info: &AttackInfo,
                source: &Source,
                airborne: bool,
            ) -> (Effect, OnHitType) {
                let counter_hit_info = &attack_info.on_counter_hit;
                (
                    Effect {
                        attacker: AttackerEffect {
                            modify_meter: counter_hit_info.attacker_meter,
                            set_stop: if source.source_type == HitSource::Character {
                                counter_hit_info.attacker_stop
                            } else {
                                0
                            },
                        },

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
                                Force::Airborne(
                                    source.facing.fix_collision(counter_hit_info.air_force),
                                )
                            } else {
                                Force::Grounded(source.facing.fix_collision(collision::Vec2::new(
                                    counter_hit_info.ground_pushback,
                                    0_00,
                                )))
                            },
                        },
                    },
                    OnHitType::CounterHit,
                )
            }

            pub fn append_hit(mut self, attack_info: &AttackInfo) -> (Self, OnHitType) {
                let attack_info = &attack_info.on_hit;
                let damage = attack_info.damage * self.combo.proration / 100;

                self.attacker.modify_meter += attack_info.attacker_meter;

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

                (self, OnHitType::Hit)
            }
        }
    }

    pub mod graze {
        use super::{
            block, counter_hit, guard_crush, hit, wrong_block, AttackerEffect, OnHitType, Source,
        };
        use crate::character::components::AttackInfo;

        pub struct Effect {
            pub attacker: AttackerEffect,
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
            pub fn build(attack_info: &AttackInfo) -> (Effect, OnHitType) {
                let graze_info = &attack_info.on_graze;
                (
                    Effect {
                        attacker: AttackerEffect {
                            modify_meter: graze_info.attacker_meter,
                            set_stop: 0,
                        },

                        defender: DefenderEffect {
                            add_spirit_delay: graze_info.spirit_delay,
                            modify_meter: graze_info.defender_meter,
                            reset_spirit_delay: graze_info.reset_spirit_delay,
                            set_stop: graze_info.defender_stop,
                            take_spirit_gauge: graze_info.spirit_cost,
                            take_damage: graze_info.damage,
                        },
                    },
                    OnHitType::Graze,
                )
            }
            pub fn append_block(
                self,
                attack_info: &AttackInfo,
                source: &Source,
                airborne: bool,
            ) -> (block::Effect, OnHitType) {
                let mut effect = block::Effect::build(attack_info, source, airborne);
                {
                    let effect = &mut effect.0;
                    effect.attacker.modify_meter += self.attacker.modify_meter;

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
            ) -> (wrong_block::Effect, OnHitType) {
                let mut effect = wrong_block::Effect::build(attack_info, source);
                {
                    let effect = &mut effect.0;
                    effect.attacker.modify_meter += self.attacker.modify_meter;

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
            ) -> (hit::Effect, OnHitType) {
                let mut effect = hit::Effect::build_starter(attack_info, source, airborne);
                {
                    let effect = &mut effect.0;
                    effect.attacker.modify_meter += self.attacker.modify_meter;

                    effect.defender.modify_meter += self.defender.modify_meter;
                    effect.defender.take_spirit_gauge += self.defender.take_spirit_gauge;
                    effect.defender.take_damage += self.defender.take_damage;
                }

                effect
            }
            pub fn append_graze(mut self, attack_info: &AttackInfo) -> (Self, OnHitType) {
                let graze_info = &attack_info.on_graze;

                self.attacker.modify_meter += graze_info.attacker_meter;

                self.defender.add_spirit_delay += graze_info.spirit_delay;
                self.defender.modify_meter += graze_info.defender_meter;
                self.defender.reset_spirit_delay |= graze_info.reset_spirit_delay;
                self.defender.set_stop += graze_info.defender_stop;
                self.defender.take_spirit_gauge += graze_info.spirit_cost;
                self.defender.take_damage += graze_info.damage;

                (self, OnHitType::Graze)
            }

            pub fn append_counterhit(
                self,
                attack_info: &AttackInfo,
                source: &Source,
                airborne: bool,
            ) -> (counter_hit::Effect, OnHitType) {
                let mut effect = counter_hit::Effect::build(attack_info, source, airborne);
                {
                    let effect = &mut effect.0;
                    effect.attacker.modify_meter += self.attacker.modify_meter;

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
            ) -> (guard_crush::Effect, OnHitType) {
                let mut effect = guard_crush::Effect::build(attack_info, source, airborne);
                {
                    let effect = &mut effect.0;
                    effect.attacker.modify_meter += self.attacker.modify_meter;

                    effect.defender.modify_meter += self.defender.modify_meter;
                    effect.defender.take_damage += self.defender.take_damage;
                }
                effect
            }
        }
    }

    pub mod block {
        use crate::{
            character::components::AttackInfo,
            roster::hit_info::{Force, HitSource},
            typedefs::collision,
        };

        use super::{guard_crush, hit, wrong_block, AttackerEffect, OnHitType, Source};
        pub struct Effect {
            pub attacker: AttackerEffect,
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
            pub set_should_pushback: bool,
            pub is_lethal: bool,
        }

        impl Effect {
            pub fn would_crush(
                previous: Option<i32>,
                attack_info: &AttackInfo,
                remaining_spirit: i32,
            ) -> bool {
                previous.unwrap_or(0) + attack_info.on_block.spirit_cost >= remaining_spirit
            }

            pub fn build(
                attack_info: &AttackInfo,
                source: &Source,
                airborne: bool,
            ) -> (Effect, OnHitType) {
                let block_info = &attack_info.on_block;
                (
                    Effect {
                        attacker: AttackerEffect {
                            modify_meter: block_info.attacker_meter,
                            set_stop: if source.source_type == HitSource::Character {
                                block_info.attacker_stop
                            } else {
                                0
                            },
                        },

                        defender: DefenderEffect {
                            is_lethal: false,
                            add_spirit_delay: block_info.spirit_delay,
                            modify_meter: block_info.defender_meter,
                            reset_spirit_delay: block_info.reset_spirit_delay,
                            set_stop: block_info.defender_stop,
                            take_spirit_gauge: block_info.spirit_cost,
                            set_stun: if airborne {
                                block_info.air_stun
                            } else {
                                block_info.stun
                            },
                            take_damage: block_info.damage,
                            set_should_pushback: source.source_type == HitSource::Character,
                            set_force: if airborne {
                                Force::Airborne(source.facing.fix_collision(block_info.air_force))
                            } else {
                                Force::Grounded(source.facing.fix_collision(collision::Vec2::new(
                                    block_info.ground_pushback,
                                    0_00,
                                )))
                            },
                        },
                    },
                    OnHitType::Block,
                )
            }

            pub fn append_block(
                mut self,
                attack_info: &AttackInfo,
                source: &Source,
                airborne: bool,
            ) -> (Self, OnHitType) {
                let block_info = &attack_info.on_block;

                self.attacker.modify_meter += block_info.attacker_meter;

                self.defender.add_spirit_delay += block_info.spirit_delay;
                self.defender.modify_meter += block_info.defender_meter;
                self.defender.reset_spirit_delay |= block_info.reset_spirit_delay;
                self.defender.take_spirit_gauge += block_info.spirit_cost;
                self.defender.take_damage += block_info.damage;

                if self.defender.set_stun < block_info.stun {
                    self.defender.set_force =
                        if airborne {
                            Force::Airborne(source.facing.fix_collision(block_info.air_force))
                        } else {
                            Force::Grounded(source.facing.fix_collision(collision::Vec2::new(
                                block_info.ground_pushback,
                                0_00,
                            )))
                        };
                    self.defender.set_stun = if airborne {
                        block_info.air_stun
                    } else {
                        block_info.stun
                    };
                    self.defender.set_stop = block_info.defender_stop;
                    self.defender.set_should_pushback = source.source_type == HitSource::Character;
                }

                (self, OnHitType::Block)
            }
            pub fn append_wrongblock(
                self,
                attack_info: &AttackInfo,
                source: &Source,
            ) -> (wrong_block::Effect, OnHitType) {
                let mut effect = wrong_block::Effect::build(attack_info, source);
                {
                    let effect = &mut effect.0;
                    effect.attacker.modify_meter += self.attacker.modify_meter;

                    effect.defender.add_spirit_delay += self.defender.add_spirit_delay;
                    effect.defender.modify_meter += self.defender.modify_meter;
                    effect.defender.reset_spirit_delay |= self.defender.reset_spirit_delay;
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
            ) -> (hit::Effect, OnHitType) {
                let mut effect = hit::Effect::build_starter(attack_info, source, airborne);
                {
                    let effect = &mut effect.0;
                    effect.attacker.modify_meter += self.attacker.modify_meter;

                    effect.defender.add_spirit_delay += self.defender.add_spirit_delay;
                    effect.defender.modify_meter += self.defender.modify_meter;
                    effect.defender.reset_spirit_delay |= self.defender.reset_spirit_delay;
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
            ) -> (guard_crush::Effect, OnHitType) {
                let mut effect = guard_crush::Effect::build(attack_info, source, airborne);
                {
                    let effect = &mut effect.0;
                    effect.attacker.modify_meter += self.attacker.modify_meter;

                    effect.defender.modify_meter += self.defender.modify_meter;
                    effect.defender.take_damage += self.defender.take_damage;
                }

                effect
            }
        }
    }

    pub mod wrong_block {
        use crate::{
            character::components::AttackInfo,
            roster::hit_info::{Force, HitSource},
            typedefs::collision,
        };

        use super::{guard_crush, AttackerEffect, OnHitType, Source};
        pub struct Effect {
            pub attacker: AttackerEffect,
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
            pub fn build(attack_info: &AttackInfo, source: &Source) -> (Effect, OnHitType) {
                let block_info = &attack_info.on_wrongblock;
                (
                    Effect {
                        attacker: AttackerEffect {
                            modify_meter: block_info.attacker_meter,
                            set_stop: if source.source_type == HitSource::Character {
                                block_info.attacker_stop
                            } else {
                                0
                            },
                        },

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
                            set_force: Force::Grounded(source.facing.fix_collision(
                                collision::Vec2::new(block_info.ground_pushback, 0_00),
                            )),
                        },
                    },
                    OnHitType::WrongBlock,
                )
            }

            pub fn append_block(mut self, attack_info: &AttackInfo) -> (Self, OnHitType) {
                let block_info = &attack_info.on_block;

                self.attacker.modify_meter += block_info.attacker_meter;

                self.defender.add_spirit_delay += block_info.spirit_delay;
                self.defender.modify_meter += block_info.defender_meter;
                self.defender.reset_spirit_delay |= block_info.reset_spirit_delay;
                self.defender.take_spirit_gauge += block_info.spirit_cost;
                self.defender.take_damage += block_info.damage;

                (self, OnHitType::Block)
            }

            pub fn append_wrongblock(
                mut self,
                attack_info: &AttackInfo,
                source: &Source,
            ) -> (Self, OnHitType) {
                let block_info = &attack_info.on_wrongblock;

                self.attacker.modify_meter += block_info.attacker_meter;

                self.defender.add_spirit_delay += block_info.spirit_delay;
                self.defender.modify_meter += block_info.defender_meter;
                self.defender.reset_spirit_delay |= block_info.reset_spirit_delay;
                self.defender.take_spirit_gauge += block_info.spirit_cost;
                self.defender.take_damage += block_info.damage;

                if self.defender.set_stun < block_info.stun {
                    self.defender.set_force = Force::Grounded(
                        source
                            .facing
                            .fix_collision(collision::Vec2::new(block_info.ground_pushback, 0_00)),
                    );
                    self.defender.set_stun = block_info.stun;
                    self.defender.set_stop = block_info.defender_stop;
                    self.defender.set_should_pushback = source.source_type == HitSource::Character;
                }

                (self, OnHitType::WrongBlock)
            }
            pub fn append_guard_crush(
                self,
                attack_info: &AttackInfo,
                source: &Source,
                airborne: bool,
            ) -> (guard_crush::Effect, OnHitType) {
                let mut effect = guard_crush::Effect::build(attack_info, source, airborne);
                {
                    let effect = &mut effect.0;
                    effect.attacker.modify_meter += self.attacker.modify_meter;

                    effect.defender.modify_meter += self.defender.modify_meter;
                    effect.defender.take_damage += self.defender.take_damage;
                }
                effect
            }
        }
    }
}
