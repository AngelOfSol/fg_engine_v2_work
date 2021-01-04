use std::any::type_name;

use hecs::EntityBuilder;

use crate::roster::character::{data::Data, player_state::PlayerState, typedefs::Character};

use super::ConstructError;

pub trait Construct<C: Character> {
    fn construct_on_to<'constructor, 'builder>(
        &'constructor self,
        _builder: &'builder mut EntityBuilder,
        _context: &PlayerState<C>,
        _data: &Data<C>,
    ) -> Result<&'builder mut EntityBuilder, ConstructError> {
        panic!(
            "{} is not a valid constructor for {}.",
            type_name::<Self>(),
            type_name::<C>()
        )
    }
}
