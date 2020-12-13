use super::{AnimationGroup, AnimationGroupV1};
use crate::graphics::animation::{self, Animation};
use crate::graphics::keyframe::Modifiers;

use serde::Deserialize;

pub mod hash_map {
    use super::{AnimationGroup, AnimationVersioned};
    use serde::{Deserialize, Deserializer};
    use std::collections::HashMap;

    pub fn deserialize<'de, D, K>(deserializer: D) -> Result<HashMap<K, AnimationGroup>, D::Error>
    where
        D: Deserializer<'de>,
        K: std::cmp::Eq + std::hash::Hash + Deserialize<'de>,
        HashMap<K, AnimationGroup>: Deserialize<'de>,
    {
        Ok(HashMap::<K, AnimationVersioned>::deserialize(deserializer)?
            .into_iter()
            .map(|(key, value)| (key, value.into_modern()))
            .collect())
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum AnimationVersioned {
    V1(AnimationGroupV1),
    #[serde(deserialize_with = "animation::version::single::deserialize")]
    Legacy(Animation),
}

impl AnimationVersioned {
    fn into_modern(self) -> AnimationGroup {
        match self {
            Self::V1(value) => value.into_modern(),
            Self::Legacy(value) => AnimationGroupV1 {
                animations: vec![value],
                modifiers: Modifiers::new(),
            }
            .into_modern(),
        }
    }
}
