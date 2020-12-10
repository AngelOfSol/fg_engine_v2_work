use super::{Construct, ConstructError};
use crate::{game_object::state::Position, typedefs::collision};
use hecs::EntityBuilder;

impl Construct for Position {
    type Context = collision::Vec2;
    fn construct_on_to<'constructor, 'builder>(
        &'constructor self,
        builder: &'builder mut EntityBuilder,
        offset: collision::Vec2,
    ) -> Result<&'builder mut EntityBuilder, ConstructError> {
        Ok(builder.add(Self {
            value: self.value + offset,
        }))
    }
}
