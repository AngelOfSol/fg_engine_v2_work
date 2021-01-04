use crate::typedefs::collision;
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
