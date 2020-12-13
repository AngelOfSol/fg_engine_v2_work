use super::constructors::{ConstructError, ConstructTag};
use crate::typedefs::collision;
use hecs::EntityBuilder;
use serde::{Deserialize, Serialize};
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct Position {
    pub value: collision::Vec2,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct Render;
impl ConstructTag for Render {}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct RenderGraphic<T> {
    id: T,
}
