use crate::character::components::AttackInfo;
use crate::input::Facing;
use crate::roster::combo_state::ComboState;

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

#[derive(Debug, Copy, Clone)]
pub struct EffectData {
    pub take_damage: i32,
    pub set_should_pushback: bool,
    pub take_spirit_gauge: i32,
    pub add_spirit_delay: i32,
    pub reset_spirit_delay: bool,
    pub modify_meter: i32,
    pub set_stop: i32,
    pub set_stun: i32,
    pub set_force: Force,
    pub is_lethal: bool,
    pub set_combo: Option<ComboState>,
}

impl EffectData {
    pub fn builder() -> EffectDataBuilder<()> {
        EffectDataBuilder {
            take_damage: 0,
            set_should_pushback: false,
            take_spirit_gauge: 0,
            add_spirit_delay: 0,
            reset_spirit_delay: false,
            set_stop: 0,
            set_stun: 0,
            set_force: (),
            set_combo: None,
            modify_meter: 0,
            is_lethal: false,
        }
    }
    pub fn into_builder(self) -> EffectDataBuilder<Force> {
        EffectDataBuilder {
            take_damage: self.take_damage,
            set_should_pushback: self.set_should_pushback,
            take_spirit_gauge: self.take_spirit_gauge,
            add_spirit_delay: self.add_spirit_delay,
            reset_spirit_delay: self.reset_spirit_delay,
            set_stop: self.set_stop,
            set_stun: self.set_stun,
            set_force: self.set_force,
            set_combo: self.set_combo,
            modify_meter: self.modify_meter,
            is_lethal: self.is_lethal,
        }
    }
    pub fn graze(info: &HitAction, airborne: bool) -> EffectDataBuilder<Force> {
        let graze_info = &info.attack_info.on_graze;

        EffectData::builder()
            .set_force(if airborne {
                Force::Airborne(Vec2::zeros())
            } else {
                Force::Grounded(Vec2::zeros())
            })
            .reset_spirit_delay(graze_info.reset_spirit_delay)
            .add_spirit_delay(graze_info.spirit_delay)
            .take_spirit_gauge(graze_info.spirit_cost)
            .set_stop(graze_info.defender_stop)
            .take_damage(graze_info.damage)
            .modify_meter(graze_info.defender_meter)
    }
    pub fn guard_crush(info: &HitAction, airborne: bool) -> EffectDataBuilder<Force> {
        let current_combo = ComboState::update(None, &info.attack_info, HitModifier::GuardCrush);

        let guard_crush_info = &info.attack_info.on_guard_crush;
        let will_be_airborne = airborne || guard_crush_info.launcher;

        EffectData::builder()
            .set_force(if will_be_airborne {
                Force::Airborne(info.facing.fix_collision(guard_crush_info.air_force))
            } else {
                Force::Grounded(
                    info.facing.fix_collision(collision::Vec2::new(
                        guard_crush_info.ground_pushback,
                        0_00,
                    )),
                )
            })
            .set_stop(guard_crush_info.defender_stop)
            .set_stun(if will_be_airborne {
                guard_crush_info.air_stun
            } else {
                guard_crush_info.stun
            })
            .set_should_pushback(info.source == HitSource::Character)
            .take_damage(current_combo.last_hit_damage)
            .set_combo(current_combo)
            .modify_meter(guard_crush_info.defender_meter)
            .is_lethal(guard_crush_info.lethal)
    }
    pub fn block(info: &HitAction, airborne: bool) -> EffectDataBuilder<Force> {
        let block_info = &info.attack_info.on_block;

        EffectData::builder()
            .set_force(if airborne {
                Force::Airborne(info.facing.fix_collision(block_info.air_force))
            } else {
                Force::Grounded(
                    info.facing
                        .fix_collision(collision::Vec2::new(block_info.ground_pushback, 0_00)),
                )
            })
            .set_stop(block_info.defender_stop)
            .set_stun(if airborne {
                block_info.air_stun
            } else {
                block_info.stun
            })
            .reset_spirit_delay(block_info.reset_spirit_delay)
            .add_spirit_delay(block_info.spirit_delay)
            .take_spirit_gauge(block_info.spirit_cost)
            .set_should_pushback(info.source == HitSource::Character)
            .take_damage(block_info.damage)
            .modify_meter(block_info.defender_meter)
    }
    pub fn wrong_block(info: &HitAction) -> EffectDataBuilder<Force> {
        let wrongblock_info = &info.attack_info.on_wrongblock;

        EffectData::builder()
            .set_force(Force::Grounded(info.facing.fix_collision(
                collision::Vec2::new(wrongblock_info.ground_pushback, 0_00),
            )))
            .set_stop(wrongblock_info.defender_stop)
            .set_stun(wrongblock_info.stun)
            .reset_spirit_delay(wrongblock_info.reset_spirit_delay)
            .add_spirit_delay(wrongblock_info.spirit_delay)
            .take_spirit_gauge(wrongblock_info.spirit_cost)
            .set_should_pushback(info.source == HitSource::Character)
            .take_damage(wrongblock_info.damage)
            .modify_meter(wrongblock_info.defender_meter)
    }

    pub fn hit(
        info: &HitAction,
        current_combo: Option<ComboState>,
        airborne: bool,
    ) -> EffectDataBuilder<Force> {
        let hit_info = &info.attack_info.on_hit;
        let will_be_airborne = airborne || hit_info.launcher;

        let current_combo = ComboState::update(current_combo, &info.attack_info, HitModifier::None);
        EffectData::builder()
            .set_force(if will_be_airborne {
                Force::Airborne(info.facing.fix_collision(hit_info.air_force))
            } else {
                Force::Grounded(
                    info.facing
                        .fix_collision(collision::Vec2::new(hit_info.ground_pushback, 0_00)),
                )
            })
            .set_stop(hit_info.defender_stop)
            .set_stun(if will_be_airborne {
                hit_info.air_stun
            } else {
                hit_info.stun
            })
            .set_should_pushback(info.source == HitSource::Character)
            .take_damage(current_combo.last_hit_damage)
            .set_combo(current_combo)
            .modify_meter(hit_info.defender_meter)
            .is_lethal(hit_info.lethal)
    }
    pub fn counter_hit(
        info: &HitAction,
        current_combo: Option<ComboState>,
        airborne: bool,
    ) -> EffectDataBuilder<Force> {
        let counter_hit_info = &info.attack_info.on_counter_hit;
        let will_be_airborne = airborne || counter_hit_info.launcher;

        let current_combo =
            ComboState::update(current_combo, &info.attack_info, HitModifier::CounterHit);
        EffectData::builder()
            .set_force(if will_be_airborne {
                Force::Airborne(info.facing.fix_collision(counter_hit_info.air_force))
            } else {
                Force::Grounded(
                    info.facing.fix_collision(collision::Vec2::new(
                        counter_hit_info.ground_pushback,
                        0_00,
                    )),
                )
            })
            .set_stop(counter_hit_info.defender_stop)
            .set_stun(if will_be_airborne {
                counter_hit_info.air_stun
            } else {
                counter_hit_info.stun
            })
            .set_should_pushback(info.source == HitSource::Character)
            .take_damage(current_combo.last_hit_damage)
            .set_combo(current_combo)
            .modify_meter(counter_hit_info.defender_meter)
            .is_lethal(counter_hit_info.lethal)
    }
}
pub struct EffectDataBuilder<T> {
    take_damage: i32,
    set_should_pushback: bool,
    take_spirit_gauge: i32,
    add_spirit_delay: i32,
    reset_spirit_delay: bool,
    modify_meter: i32,
    set_stop: i32,
    set_stun: i32,
    set_force: T,
    set_combo: Option<ComboState>,
    is_lethal: bool,
}

impl<T> EffectDataBuilder<T> {
    pub fn set_force(self, value: Force) -> EffectDataBuilder<Force> {
        EffectDataBuilder {
            take_damage: self.take_damage,
            set_should_pushback: self.set_should_pushback,
            take_spirit_gauge: self.take_spirit_gauge,
            add_spirit_delay: self.add_spirit_delay,
            reset_spirit_delay: self.reset_spirit_delay,
            set_stop: self.set_stop,
            set_stun: self.set_stun,
            set_force: value,
            set_combo: self.set_combo,
            modify_meter: self.modify_meter,
            is_lethal: self.is_lethal,
        }
    }
    pub fn take_damage(mut self, value: i32) -> Self {
        self.take_damage += value;
        self
    }
    pub fn take_spirit_gauge(mut self, value: i32) -> Self {
        self.take_spirit_gauge += value;
        self
    }
    pub fn add_spirit_delay(mut self, value: i32) -> Self {
        self.add_spirit_delay = value;
        self
    }
    pub fn reset_spirit_delay(mut self, value: bool) -> Self {
        self.reset_spirit_delay = value;
        self
    }

    pub fn modify_meter(mut self, value: i32) -> Self {
        self.modify_meter += value;
        self
    }

    pub fn set_stop(mut self, value: i32) -> Self {
        self.set_stop = value;
        self
    }
    pub fn set_stun(mut self, value: i32) -> Self {
        self.set_stun = value;
        self
    }
    pub fn set_should_pushback(mut self, value: bool) -> Self {
        self.set_should_pushback = value;
        self
    }
    pub fn set_combo(mut self, value: ComboState) -> Self {
        self.set_combo = Some(value);
        self
    }
    #[allow(clippy::clippy::wrong_self_convention)]
    pub fn is_lethal(mut self, value: bool) -> Self {
        self.is_lethal = self.is_lethal || value;
        self
    }

    pub fn inherit_spirit_delay(self, old_effect: &EffectData) -> Self {
        self.reset_spirit_delay(old_effect.reset_spirit_delay)
            .add_spirit_delay(old_effect.add_spirit_delay)
    }

    pub fn inherit_non_hit_data(self, old_effect: &EffectData) -> Self {
        self.take_spirit_gauge(old_effect.take_spirit_gauge)
            .take_damage(old_effect.take_damage)
            .modify_meter(old_effect.modify_meter)
            .is_lethal(old_effect.is_lethal)
    }

    pub fn apply_hit(self, info: &HitAction) -> Self {
        let current_combo =
            ComboState::update(self.set_combo, &info.attack_info, HitModifier::None);
        self.take_damage(current_combo.last_hit_damage)
            .take_spirit_gauge(info.attack_info.on_hit.spirit_cost)
            .modify_meter(info.attack_info.on_hit.defender_meter)
            .is_lethal(info.attack_info.on_hit.lethal)
            .set_combo(current_combo)
    }
}
impl EffectDataBuilder<Force> {
    pub fn build(self) -> EffectData {
        EffectData {
            take_damage: self.take_damage,
            set_should_pushback: self.set_should_pushback,
            take_spirit_gauge: self.take_spirit_gauge,
            add_spirit_delay: self.add_spirit_delay,
            reset_spirit_delay: self.reset_spirit_delay,
            set_stop: self.set_stop,
            set_stun: self.set_stun,
            set_force: self.set_force,
            set_combo: self.set_combo,
            modify_meter: self.modify_meter,
            is_lethal: self.is_lethal,
        }
    }
}

use crate::roster::generic_character::combo_state::HitModifier;
use crate::typedefs::collision;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum HitEffectType {
    Graze,
    Block,
    WrongBlock,
    Hit,
    CounterHit,
    GuardCrush,
    GrazeCrush,
}

#[derive(Debug, Copy, Clone)]
pub struct HitEffect {
    pub hit_type: HitEffectType,
    pub effect: EffectData,
}

#[derive(Debug, Clone)]
pub struct HitAction {
    pub source: HitSource,
    pub hash: u64,
    pub attack_info: AttackInfo,
    pub facing: Facing,
    pub smp: bool,
}

#[derive(Debug, Clone)]
pub struct HitResult {
    pub action: HitAction,
    pub hit_type: HitEffectType,
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

        use super::{AttackerEffect, ComboEffect, Source};
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
            ) -> Effect {
                let attack_info = &attack_info.on_hit;
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
                        set_stun: attack_info.stun,
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
                }
            }

            pub fn build(
                attack_info: &AttackInfo,
                source: &Source,
                airborne: bool,
                current_combo: ComboEffect,
            ) -> Effect {
                let attack_info = &attack_info.on_hit;
                let damage = attack_info.damage * current_combo.proration / 100;
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
                        set_stun: attack_info.stun,
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
                }
            }

            pub fn append_hit(mut self, attack_info: &AttackInfo, source: &Source) -> Self {
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

                self
            }
        }
    }

    pub mod guard_crush {
        use crate::{
            character::components::AttackInfo,
            roster::hit_info::{Force, HitSource},
            typedefs::collision,
        };

        use super::{AttackerEffect, ComboEffect, Source};
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
            pub fn build(attack_info: &AttackInfo, source: &Source, airborne: bool) -> Effect {
                let guard_crush_info = &attack_info.on_guard_crush;
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
                        set_stun: guard_crush_info.stun,
                        take_damage: guard_crush_info.damage,
                        set_should_pushback: source.source_type == HitSource::Character,
                        set_force: if airborne || guard_crush_info.launcher {
                            Force::Airborne(source.facing.fix_collision(guard_crush_info.air_force))
                        } else {
                            Force::Grounded(source.facing.fix_collision(collision::Vec2::new(
                                guard_crush_info.ground_pushback,
                                0_00,
                            )))
                        },
                    },
                }
            }
            pub fn append_hit(mut self, attack_info: &AttackInfo) -> Self {
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

                self
            }
        }
    }

    pub mod counter_hit {
        use crate::{
            character::components::AttackInfo,
            roster::hit_info::{Force, HitSource},
            typedefs::collision,
        };

        use super::{AttackerEffect, ComboEffect, Source};
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
            pub fn build(attack_info: &AttackInfo, source: &Source, airborne: bool) -> Effect {
                let attack_info = &attack_info.on_counter_hit;
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
                        set_stun: attack_info.stun,
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
                }
            }

            pub fn append_hit(mut self, attack_info: &AttackInfo) -> Self {
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

                self
            }
        }
    }

    pub mod graze {
        use super::{block, counter_hit, guard_crush, hit, wrong_block, AttackerEffect, Source};
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
            pub fn build(attack_info: &AttackInfo) -> Effect {
                let graze_info = &attack_info.on_graze;
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
                }
            }
            pub fn append_block(
                self,
                attack_info: &AttackInfo,
                source: &Source,
                airborne: bool,
            ) -> block::Effect {
                let mut effect = block::Effect::build(attack_info, source, airborne);

                effect.attacker.modify_meter += self.attacker.modify_meter;

                effect.defender.modify_meter += self.defender.modify_meter;
                effect.defender.take_spirit_gauge += self.defender.take_spirit_gauge;
                effect.defender.take_damage += self.defender.take_damage;

                effect
            }
            pub fn append_wrongblock(
                self,
                attack_info: &AttackInfo,
                source: &Source,
            ) -> wrong_block::Effect {
                let mut effect = wrong_block::Effect::build(attack_info, source);

                effect.attacker.modify_meter += self.attacker.modify_meter;

                effect.defender.modify_meter += self.defender.modify_meter;
                effect.defender.take_spirit_gauge += self.defender.take_spirit_gauge;
                effect.defender.take_damage += self.defender.take_damage;

                effect
            }
            pub fn append_hit(
                self,
                attack_info: &AttackInfo,
                source: &Source,
                airborne: bool,
            ) -> hit::Effect {
                let mut effect = hit::Effect::build_starter(attack_info, source, airborne);

                effect.attacker.modify_meter += self.attacker.modify_meter;

                effect.defender.modify_meter += self.defender.modify_meter;
                effect.defender.take_spirit_gauge += self.defender.take_spirit_gauge;
                effect.defender.take_damage += self.defender.take_damage;

                effect
            }
            pub fn append_graze(mut self, attack_info: &AttackInfo) -> Self {
                let graze_info = &attack_info.on_graze;

                self.attacker.modify_meter += graze_info.attacker_meter;

                self.defender.add_spirit_delay += graze_info.spirit_delay;
                self.defender.modify_meter += graze_info.defender_meter;
                self.defender.reset_spirit_delay |= graze_info.reset_spirit_delay;
                self.defender.set_stop += graze_info.defender_stop;
                self.defender.take_spirit_gauge += graze_info.spirit_cost;
                self.defender.take_damage += graze_info.damage;

                self
            }

            pub fn append_counterhit(
                self,
                attack_info: &AttackInfo,
                source: &Source,
                airborne: bool,
            ) -> counter_hit::Effect {
                let mut effect = counter_hit::Effect::build(attack_info, source, airborne);

                effect.attacker.modify_meter += self.attacker.modify_meter;

                effect.defender.modify_meter += self.defender.modify_meter;
                effect.defender.take_spirit_gauge += self.defender.take_spirit_gauge;
                effect.defender.take_damage += self.defender.take_damage;

                effect
            }

            pub fn append_guard_crush(
                self,
                attack_info: &AttackInfo,
                source: &Source,
                airborne: bool,
            ) -> guard_crush::Effect {
                let mut effect = guard_crush::Effect::build(attack_info, source, airborne);

                effect.attacker.modify_meter += self.attacker.modify_meter;

                effect.defender.modify_meter += self.defender.modify_meter;
                effect.defender.take_damage += self.defender.take_damage;

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

        use super::{guard_crush, hit, wrong_block, AttackerEffect, Source};
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

            pub fn build(attack_info: &AttackInfo, source: &Source, airborne: bool) -> Effect {
                let block_info = &attack_info.on_block;
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
                        set_force: if airborne {
                            Force::Airborne(source.facing.fix_collision(block_info.air_force))
                        } else {
                            Force::Grounded(source.facing.fix_collision(collision::Vec2::new(
                                block_info.ground_pushback,
                                0_00,
                            )))
                        },
                    },
                }
            }

            pub fn append_block(
                mut self,
                attack_info: &AttackInfo,
                source: &Source,
                airborne: bool,
            ) -> Self {
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
                    self.defender.set_stun = block_info.air_stun;
                    self.defender.set_stop = block_info.defender_stop;
                    self.defender.set_should_pushback = source.source_type == HitSource::Character;
                }

                self
            }
            pub fn append_wrongblock(
                self,
                attack_info: &AttackInfo,
                source: &Source,
            ) -> wrong_block::Effect {
                let mut effect = wrong_block::Effect::build(attack_info, source);

                effect.attacker.modify_meter += self.attacker.modify_meter;

                effect.defender.add_spirit_delay += self.defender.add_spirit_delay;
                effect.defender.modify_meter += self.defender.modify_meter;
                effect.defender.reset_spirit_delay |= self.defender.reset_spirit_delay;
                effect.defender.take_spirit_gauge += self.defender.take_spirit_gauge;
                effect.defender.take_damage += self.defender.take_damage;

                effect
            }

            pub fn append_hit(
                self,
                attack_info: &AttackInfo,
                source: &Source,
                airborne: bool,
            ) -> hit::Effect {
                let mut effect = hit::Effect::build_starter(attack_info, source, airborne);

                effect.attacker.modify_meter += self.attacker.modify_meter;

                effect.defender.add_spirit_delay += self.defender.add_spirit_delay;
                effect.defender.modify_meter += self.defender.modify_meter;
                effect.defender.reset_spirit_delay |= self.defender.reset_spirit_delay;
                effect.defender.take_spirit_gauge += self.defender.take_spirit_gauge;
                effect.defender.take_damage += self.defender.take_damage;

                effect
            }

            pub fn append_guard_crush(
                self,
                attack_info: &AttackInfo,
                source: &Source,
                airborne: bool,
            ) -> guard_crush::Effect {
                let mut effect = guard_crush::Effect::build(attack_info, source, airborne);

                effect.attacker.modify_meter += self.attacker.modify_meter;

                effect.defender.modify_meter += self.defender.modify_meter;
                effect.defender.take_damage += self.defender.take_damage;

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

        use super::{guard_crush, AttackerEffect, Source};
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
            pub fn build(attack_info: &AttackInfo, source: &Source) -> Effect {
                let block_info = &attack_info.on_wrongblock;
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
                        set_force: Force::Grounded(
                            source.facing.fix_collision(collision::Vec2::new(
                                block_info.ground_pushback,
                                0_00,
                            )),
                        ),
                    },
                }
            }

            pub fn append_block(mut self, attack_info: &AttackInfo) -> Self {
                let block_info = &attack_info.on_block;

                self.attacker.modify_meter += block_info.attacker_meter;

                self.defender.add_spirit_delay += block_info.spirit_delay;
                self.defender.modify_meter += block_info.defender_meter;
                self.defender.reset_spirit_delay |= block_info.reset_spirit_delay;
                self.defender.take_spirit_gauge += block_info.spirit_cost;
                self.defender.take_damage += block_info.damage;

                self
            }

            pub fn append_wrongblock(mut self, attack_info: &AttackInfo, source: &Source) -> Self {
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

                self
            }
            pub fn append_guard_crush(
                self,
                attack_info: &AttackInfo,
                source: &Source,
                airborne: bool,
            ) -> guard_crush::Effect {
                let mut effect = guard_crush::Effect::build(attack_info, source, airborne);

                effect.attacker.modify_meter += self.attacker.modify_meter;

                effect.defender.modify_meter += self.defender.modify_meter;
                effect.defender.take_damage += self.defender.take_damage;

                effect
            }
        }
    }
}
