use crate::character::state::components::BulletSpawn;
use crate::character::state::State;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct States<Id, ParticleId, BulletSpawnInfo, AttackId, SoundType> {
    #[serde(flatten)]
    #[serde(bound(
        serialize = "HashMap<String, State<Id, ParticleId, BulletSpawnInfo, AttackId, SoundType>>: Serialize",
        deserialize = "HashMap<String, State<Id, ParticleId, BulletSpawnInfo, AttackId, SoundType>>: Deserialize<'de>"
    ))]
    pub rest: HashMap<String, State<Id, ParticleId, BulletSpawnInfo, AttackId, SoundType>>,
    #[serde(skip)]
    _secret: (),
}

impl<Id, ParticleId, BulletSpawnInfo, AttackId, SoundType> std::fmt::Debug
    for States<Id, ParticleId, BulletSpawnInfo, AttackId, SoundType>
where
    HashMap<String, State<Id, ParticleId, BulletSpawnInfo, AttackId, SoundType>>: std::fmt::Debug,
{
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let mut builder = fmt.debug_struct("States");
        let _ = builder.field("rest", &self.rest);
        builder.finish()
    }
}

pub type EditorStates = States<String, String, BulletSpawn, String, String>;

impl<Id, ParticleId, BulletSpawnInfo: Eq + Default, AttackId, SoundType>
    States<Id, ParticleId, BulletSpawnInfo, AttackId, SoundType>
{
    pub fn new() -> Self {
        Self {
            rest: HashMap::new(),
            _secret: (),
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
