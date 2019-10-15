use crate::character::state::components::BulletSpawn;
use crate::character::state::State;
use crate::typedefs::HashId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct States<Id, ParticleId, BulletInfo>
where
    Id: HashId,
    ParticleId: HashId,
    BulletInfo: Default,
{
    #[serde(flatten)]
    pub rest: HashMap<String, State<Id, ParticleId, BulletInfo>>,
    #[serde(skip)]
    _secret: (),
}

pub type EditorStates = States<String, String, BulletSpawn>;

impl<Id: HashId, ParticleId: HashId, BulletInfo: Eq + Default> States<Id, ParticleId, BulletInfo> {
    pub fn new() -> Self {
        Self {
            rest: HashMap::new(),
            _secret: (),
        }
    }

    pub fn get_state(&self, key: &str) -> &State<Id, ParticleId, BulletInfo> {
        match key {
            _ => &self.rest[key],
        }
    }
    pub fn replace_state(&mut self, key: String, data: State<Id, ParticleId, BulletInfo>) {
        match key.as_str() {
            _ => {
                self.rest.insert(key, data);
            }
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
