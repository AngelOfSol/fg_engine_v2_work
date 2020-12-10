use hecs::EntityBuilder;

use crate::game_object::state::Position;

use super::{Construct, ConstructError};

impl Construct for Position {
    fn construct_on_to<'c, 'eb>(
        &'c self,
        builder: &'eb mut EntityBuilder,
    ) -> Result<&'eb mut EntityBuilder, ConstructError> {
        Ok(builder.add(*self))
    }
}
