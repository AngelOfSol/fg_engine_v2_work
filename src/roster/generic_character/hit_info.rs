pub mod block;
pub mod counter_hit;
pub mod graze;
pub mod guard_crush;
pub mod hit;
pub mod wrong_block;

use crate::typedefs::collision::Vec2;
use crate::{character::components::GroundAction, input::Facing};

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub enum HitSource {
    Character,
    Object,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Force {
    Grounded(Vec2),
    Airborne(Vec2),
}

#[derive(Debug, Clone)]
pub struct ComboEffect {
    pub hits: u32,
    pub total_damage: i32,
    pub proration: i32,
    pub available_limit: i32,
    pub ground_action: GroundAction,
}

pub struct Source {
    pub source_type: HitSource,
    pub facing: Facing,
}

pub enum HitType {
    Hit,
    GuardCrush,
    Graze,
    CounterHit,
    Block,
    WrongBlock,
}

pub enum HitResult {
    None,
    Pass(HitEffect),
    HitBy(HitType, HitEffect),
}

impl<F> From<(F, HitType)> for HitResult
where
    HitEffect: From<F>,
{
    fn from((effect, hit_type): (F, HitType)) -> Self {
        Self::HitBy(hit_type, HitEffect::from(effect))
    }
}

impl<F> From<F> for HitResult
where
    HitEffect: From<F>,
{
    fn from(effect: F) -> Self {
        Self::Pass(HitEffect::from(effect))
    }
}
impl From<Option<HitEffect>> for HitResult {
    fn from(value: Option<HitEffect>) -> Self {
        match value {
            Some(effect) => Self::Pass(effect),
            None => Self::None,
        }
    }
}

impl HitResult {
    pub fn split(self) -> (Option<HitEffect>, Option<HitType>) {
        match self {
            Self::None => (None, None),
            Self::Pass(effect) => (Some(effect), None),
            Self::HitBy(hit_type, effect) => (Some(effect), Some(hit_type)),
        }
    }
}

pub enum HitEffect {
    Hit(hit::Effect),
    GuardCrush(guard_crush::Effect),
    CounterHit(counter_hit::Effect),
    Graze(graze::Effect),
    Block(block::Effect),
    WrongBlock(wrong_block::Effect),
}

impl From<hit::Effect> for HitEffect {
    fn from(value: hit::Effect) -> Self {
        Self::Hit(value)
    }
}
impl From<guard_crush::Effect> for HitEffect {
    fn from(value: guard_crush::Effect) -> Self {
        Self::GuardCrush(value)
    }
}
impl From<counter_hit::Effect> for HitEffect {
    fn from(value: counter_hit::Effect) -> Self {
        Self::CounterHit(value)
    }
}
impl From<graze::Effect> for HitEffect {
    fn from(value: graze::Effect) -> Self {
        Self::Graze(value)
    }
}
impl From<block::Effect> for HitEffect {
    fn from(value: block::Effect) -> Self {
        Self::Block(value)
    }
}
impl From<wrong_block::Effect> for HitEffect {
    fn from(value: wrong_block::Effect) -> Self {
        Self::WrongBlock(value)
    }
}
