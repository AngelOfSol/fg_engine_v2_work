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

pub type EditorStates = States<String, String, String>;

impl<Id, AttackId, SoundType> States<Id, AttackId, SoundType> {
    pub fn new() -> Self {
        Self {
            rest: HashMap::new(),
        }
    }

    pub fn guarentee_unique_key<S: Into<String>>(&self, key: S) -> String {
        let base = key.into();
        let mut new_key = base.clone();
        let mut counter = 1;
        loop {
            if self.rest.contains_key(&new_key) {
                new_key = format!("{} ({})", base, counter);
                counter += 1;
            } else {
                break;
            }
        }
        new_key
    }
}
