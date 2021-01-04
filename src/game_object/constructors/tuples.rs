use hecs::EntityBuilder;

use crate::roster::character::{data::Data, player_state::PlayerState, typedefs::Character};

use super::{Construct, ConstructError};

impl<C, T, U> Construct<C> for (T, U)
where
    C: Character,
    T: Construct<C>,
    U: Construct<C>,
{
    fn construct_on_to<'constructor, 'builder>(
        &'constructor self,
        builder: &'builder mut EntityBuilder,
        context: &PlayerState<C>,
        data: &Data<C>,
    ) -> Result<&'builder mut EntityBuilder, ConstructError> {
        self.0.construct_on_to(builder, context, data)?;
        self.1.construct_on_to(builder, context, data)?;
        Ok(builder)
    }
}
impl<C, T, U, V> Construct<C> for (T, U, V)
where
    C: Character,
    T: Construct<C>,
    U: Construct<C>,
    V: Construct<C>,
{
    fn construct_on_to<'constructor, 'builder>(
        &'constructor self,
        builder: &'builder mut EntityBuilder,
        context: &PlayerState<C>,
        data: &Data<C>,
    ) -> Result<&'builder mut EntityBuilder, ConstructError> {
        self.0.construct_on_to(builder, context, data)?;
        self.1.construct_on_to(builder, context, data)?;
        self.2.construct_on_to(builder, context, data)?;
        Ok(builder)
    }
}

impl<C, T, U, V, W> Construct<C> for (T, U, V, W)
where
    C: Character,
    T: Construct<C>,
    U: Construct<C>,
    V: Construct<C>,
    W: Construct<C>,
{
    fn construct_on_to<'constructor, 'builder>(
        &'constructor self,
        builder: &'builder mut EntityBuilder,
        context: &PlayerState<C>,
        data: &Data<C>,
    ) -> Result<&'builder mut EntityBuilder, ConstructError> {
        self.0.construct_on_to(builder, context, data)?;
        self.1.construct_on_to(builder, context, data)?;
        self.2.construct_on_to(builder, context, data)?;
        self.3.construct_on_to(builder, context, data)?;
        Ok(builder)
    }
}
