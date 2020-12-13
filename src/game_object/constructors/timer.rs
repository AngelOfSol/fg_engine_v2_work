use super::{Construct, ConstructError, Inspect};
use crate::game_object::state::Timer;
use hecs::EntityBuilder;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimerConstructor;

impl Construct for TimerConstructor {
    type Context = ();
    fn construct_on_to<'constructor, 'builder>(
        &'constructor self,
        builder: &'builder mut EntityBuilder,
        _: Self::Context,
    ) -> Result<&'builder mut EntityBuilder, ConstructError> {
        Ok(builder.add(Timer(0)))
    }
}

impl Inspect for TimerConstructor {}
