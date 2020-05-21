use crate::character::components::AttackInfo;
use crate::input::Facing;
use crate::roster::combo_state::ComboState;

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub enum HitSource {
    Character,
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
    pub fn new() -> EffectDataBuilder<()> {
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

        EffectData::new()
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

        EffectData::new()
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

        EffectData::new()
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

        EffectData::new()
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
        EffectData::new()
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
        EffectData::new()
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
}

#[derive(Debug, Clone)]
pub struct HitResult {
    pub action: HitAction,
    pub hit_type: HitEffectType,
}
