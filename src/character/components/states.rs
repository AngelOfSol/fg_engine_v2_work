use crate::character::state::State;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub struct States<Id, AttackId, SoundType> {
    #[serde(flatten)]
    #[serde(bound(
        serialize = "HashMap<String, State<Id, AttackId, SoundType>>: Serialize",
        deserialize = "HashMap<String, State<Id, AttackId, SoundType>>: Deserialize<'de>"
    ))]
    pub rest: HashMap<String, State<Id, AttackId, SoundType>>,
}

impl<Id, AttackId, SoundType> std::fmt::Debug for States<Id, AttackId, SoundType>
where
    HashMap<String, State<Id, AttackId, SoundType>>: std::fmt::Debug,
{
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let mut builder = fmt.debug_struct("States");
        let _ = builder.field("rest", &self.rest);
        builder.finish()
    }
}
