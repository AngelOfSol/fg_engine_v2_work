use std::marker::PhantomData;

use hecs::{Component, EntityBuilder};
use inspect_design::Inspect;
use serde::{Deserialize, Serialize};

use crate::roster::character::{data::Data, player_state::PlayerState, typedefs::Character};

use super::{Construct, ConstructError};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default, Inspect)]
pub struct ConstructId<Id> {
    pub value: Id,
}

impl<C: Character, T: Component + Clone> Construct<C> for ConstructId<T> {
    fn construct_on_to<'constructor, 'builder>(
        &'constructor self,
        builder: &'builder mut EntityBuilder,
        _context: &PlayerState<C>,
        _data: &Data<C>,
    ) -> Result<&'builder mut EntityBuilder, ConstructError> {
        builder.add(self.value.clone());
        Ok(builder)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default, Inspect)]
pub struct ConstructDefault<T> {
    _marker: PhantomData<T>,
}

impl<C: Character, T: Component + Default> Construct<C> for ConstructDefault<T> {
    fn construct_on_to<'constructor, 'builder>(
        &'constructor self,
        builder: &'builder mut EntityBuilder,
        _context: &PlayerState<C>,
        _data: &Data<C>,
    ) -> Result<&'builder mut EntityBuilder, ConstructError> {
        builder.add(T::default());
        Ok(builder)
    }
}
