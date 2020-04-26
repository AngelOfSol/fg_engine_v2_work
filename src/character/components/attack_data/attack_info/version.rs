use super::super::{AttackLevel, GroundAction, Guard};
use super::{
    default_air_force, default_ground_pushback, default_hitstop, AttackInfo, AttackInfoV1,
    BlockInfo, CounterHitInfo, GrazeInfo, GuardCrushInfo, HitInfo, WrongBlockInfo,
};
use crate::typedefs::collision::{Int, Vec2};
use serde::Deserialize;

pub mod hashmap {
    use super::{AttackInfo, AttackInfoVersioned};

    use serde::{Deserialize, Deserializer};
    use std::collections::HashMap;

    pub fn deserialize<'de, Key: std::hash::Hash + Eq + serde::de::Deserialize<'de>, D>(
        deserializer: D,
    ) -> Result<HashMap<Key, AttackInfo>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(
            HashMap::<Key, AttackInfoVersioned>::deserialize(deserializer)?
                .into_iter()
                .map(|(key, item)| (key, item.to_modern()))
                .collect(),
        )
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum AttackInfoVersioned {
    AttackInfoV1(AttackInfoV1),
    AttackInfoLegacy(AttackLegacy),
}

impl AttackInfoVersioned {
    fn to_modern(self) -> AttackInfo {
        match self {
            Self::AttackInfoV1(value) => value.to_modern(),
            Self::AttackInfoLegacy(value) => value.to_modern(),
        }
    }
}

impl AttackLegacy {
    pub fn to_modern(self) -> AttackInfoV1 {
        AttackInfoV1 {
            melee: self.melee,
            magic: !self.melee,
            guard: self.guard,
            air_unblockable: self.air_unblockable,
            grazeable: self.grazeable,
            can_counter_hit: self.can_counter_hit,
            on_graze: GrazeInfo::default(),
            on_hit: HitInfo {
                attacker_stop: self.on_hit.attacker_stop,
                defender_stop: self.on_hit.defender_stop,
                stun: self.level.hitstun(),
                air_stun: self.level.hitstun(),
                damage: self.hit_damage,
                lethal: true,
                spirit_cost: 0,
                spirit_delay: 0,
                reset_spirit_delay: false,
                air_force: self.on_hit.air_force,
                ground_pushback: self.on_hit.ground_pushback,
                launcher: self.launcher,
                ground_action: self.ground_action,
                starter_limit: self.starter_limit,
                limit_cost: self.limit_cost,
                proration: self.proration,
            },
            on_counter_hit: CounterHitInfo {
                attacker_stop: self.on_hit.attacker_stop,
                defender_stop: self.on_hit.defender_stop,
                stun: self.level.counter_hitstun(),
                air_stun: self.level.counter_hitstun(),
                damage: self.hit_damage,
                lethal: true,
                spirit_cost: 0,
                spirit_delay: 0,
                reset_spirit_delay: false,
                air_force: self.on_hit.air_force,
                ground_pushback: self.on_hit.ground_pushback,
                launcher: self.launcher,
                ground_action: self.ground_action,
                starter_limit: self.counter_hit_limit,
                proration: self.proration,
            },
            on_guard_crush: GuardCrushInfo {
                attacker_stop: self.on_hit.attacker_stop,
                defender_stop: self.on_hit.defender_stop,
                stun: self.level.crush_stun(),
                air_stun: self.level.crush_stun(),
                damage: self.hit_damage,
                lethal: true,
                air_force: self.on_hit.air_force,
                ground_pushback: self.on_hit.ground_pushback,
                launcher: self.launcher,
                ground_action: self.ground_action,
                starter_limit: self.starter_limit,
                proration: self.proration,
            },
            on_block: BlockInfo {
                attacker_stop: self.on_block.attacker_stop,
                defender_stop: self.on_block.defender_stop,
                stun: self.level.blockstun(),
                air_stun: self.level.blockstun(),
                damage: self.chip_damage,
                spirit_cost: self.spirit_cost,
                spirit_delay: self.spirit_delay,
                reset_spirit_delay: self.reset_spirit_delay,
                air_force: self.on_block.air_force,
                ground_pushback: self.on_block.ground_pushback,
            },
            on_wrongblock: WrongBlockInfo {
                attacker_stop: self.on_block.attacker_stop,
                defender_stop: self.on_block.defender_stop,
                stun: self.level.wrongblockstun(),
                damage: self.chip_damage,
                spirit_cost: self.level.wrongblock_cost(),
                spirit_delay: self.level.wrongblock_delay(),
                reset_spirit_delay: true,
                ground_pushback: self.on_block.ground_pushback,
            },
        }
        .to_modern()
    }
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct AttackLegacy {
    pub level: AttackLevel,
    #[serde(default)]
    pub guard: Guard,
    #[serde(default)]
    pub air_unblockable: bool,
    #[serde(default)]
    pub grazeable: bool,
    #[serde(default)]
    pub melee: bool,

    #[serde(default)]
    pub on_hit: HitInfoLegacy,

    #[serde(default)]
    pub hit_damage: i32,
    #[serde(default)]
    pub proration: i32,
    #[serde(default)]
    pub launcher: bool,
    #[serde(default)]
    pub ground_action: GroundAction,

    #[serde(default)]
    pub starter_limit: i32,
    #[serde(default)]
    pub limit_cost: i32,

    #[serde(default)]
    pub counter_hit_limit: i32,
    #[serde(default)]
    pub can_counter_hit: bool,

    #[serde(default)]
    pub on_block: HitInfoLegacy,

    #[serde(default)]
    pub spirit_cost: i32,
    #[serde(default)]
    pub spirit_delay: i32,
    #[serde(default)]
    pub reset_spirit_delay: bool,
    #[serde(default)]
    pub chip_damage: i32,
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct HitInfoLegacy {
    #[serde(default = "default_hitstop")]
    pub attacker_stop: i32,
    #[serde(default = "default_hitstop")]
    pub defender_stop: i32,
    #[serde(default = "default_air_force")]
    pub air_force: Vec2,
    #[serde(default = "default_ground_pushback")]
    pub ground_pushback: Int,
}

impl Default for HitInfoLegacy {
    fn default() -> Self {
        Self {
            attacker_stop: default_hitstop(),
            defender_stop: default_hitstop(),
            air_force: default_air_force(),
            ground_pushback: default_ground_pushback(),
        }
    }
}
