use crate::{
    roster::character::typedefs::{Character, HitId, Timed},
    typedefs::collision,
};
use inspect_design::Inspect;
use serde::{Deserialize, Serialize};
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Default, Inspect)]
pub struct Position {
    pub value: collision::Vec2,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Default, Inspect)]
pub struct Velocity {
    pub value: collision::Vec2,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Default, Inspect)]
pub struct Timer(pub usize);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Default, Inspect)]
pub struct ExpiresAfterAnimation;

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, Default, Inspect)]
pub struct Rotation(pub f32);

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, Default, Inspect)]
pub struct HasHitbox;

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, Inspect, Eq, PartialOrd, Ord)]
pub enum BulletTier {
    S,
    A,
    B,
    C,
}

impl Default for BulletTier {
    fn default() -> Self {
        Self::C
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, Default, Inspect)]
pub struct BulletHp {
    pub tier: BulletTier,
    pub health: i32,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, Inspect)]
pub struct ObjectAttack<C: Character> {
    pub command: Timed<C::Command>,
    pub multi_hit: MultiHitType<C>,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Inspect)]
pub enum MultiHitType<C: Character> {
    LastHitUsing(Option<HitId<C::Attack>>),
    RemainingHits(i32),
}

impl<C: Character> PartialEq for MultiHitType<C> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::RemainingHits(lhs), Self::RemainingHits(rhs)) => lhs == rhs,
            (Self::LastHitUsing(lhs), Self::LastHitUsing(rhs)) => lhs == rhs,
            _ => false,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, Default, Inspect)]
pub struct Hitstop(pub i32);

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, Default, Inspect)]
pub struct HitDelay(pub i32);

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, Default, Inspect)]
pub struct GrazeResistance(pub i32);
