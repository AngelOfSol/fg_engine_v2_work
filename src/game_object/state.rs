use crate::typedefs::collision;
use serde::{Deserialize, Serialize};
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct Position {
    pub value: collision::Vec2,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct Timer(pub usize);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct ExpiresAfterAnimation;

impl super::constructors::Construct for ExpiresAfterAnimation {
    type Context = ();
    fn construct_on_to<'constructor, 'builder>(
        &'constructor self,
        builder: &'builder mut hecs::EntityBuilder,
        context: Self::Context,
    ) -> Result<&'builder mut hecs::EntityBuilder, super::constructors::ConstructError> {
        Ok(builder)
        //
    }
}

impl super::constructors::Inspect for ExpiresAfterAnimation {}
