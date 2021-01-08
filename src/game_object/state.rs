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
pub struct Hitbox<T>(pub T);

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
    pub id: C::ObjectData,
    pub command: Timed<C::Command>,
    pub last_hit_using: Option<HitId<C::Attack>>,
}
