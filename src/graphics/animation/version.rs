use super::{Animation, AnimationV1};
use crate::graphics::keyframe::Modifiers;
use crate::typedefs::graphics::Vec2;
use serde::Deserialize;

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

#[derive(Deserialize)]
struct AnimationData {
    pub animation: AnimationV1,
    pub delay: usize,
    pub offset: Vec2,
    pub scale: Vec2,
}

impl AnimationData {
    fn to_modern(self) -> Animation {
        AnimationV1 {
            delay: self.delay,
            modifiers: Modifiers::with_basic(0.0, self.scale, self.offset),

            ..self.animation
        }
        .to_modern()
    }
}
