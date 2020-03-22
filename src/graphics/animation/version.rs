use crate::graphics::animation_data::AnimationData;
use serde::Deserialize;

use super::{Animation, AnimationV1};

pub mod vec {
    use super::{Animation, AnimationVersioned};

    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Animation>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Vec::<AnimationVersioned>::deserialize(deserializer)?
            .into_iter()
            .map(|item| item.to_modern())
            .collect())
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum AnimationVersioned {
    AnimationV1(AnimationV1),
    AnimationDataLegacy(AnimationData),
}

impl AnimationVersioned {
    fn to_modern(self) -> Animation {
        match self {
            Self::AnimationV1(value) => value.to_modern(),
            Self::AnimationDataLegacy(value) => value.to_modern(),
        }
    }
}
