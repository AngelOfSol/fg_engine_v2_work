use super::{Construct, ConstructError};
use crate::roster::character::typedefs::Character;
use crate::{
    game_object::state::Position,
    roster::character::{data::Data, player_state::PlayerState},
};
use hecs::EntityBuilder;

impl<C: Character> Construct<C> for Position {
    fn construct_on_to<'constructor, 'builder>(
        &'constructor self,
        builder: &'builder mut EntityBuilder,
        context: &PlayerState<C>,
        _data: &Data<C>,
    ) -> Result<&'builder mut EntityBuilder, ConstructError> {
        builder.add(Self {
            value: context.facing.fix(self.value) + context.position,
        });
        Ok(builder)
    }
}
