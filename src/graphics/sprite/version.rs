use super::{Sprite, SpriteV1};
use crate::graphics::keyframe::Modifiers;
use crate::typedefs::graphics::Vec2;
use serde::Deserialize;

pub mod vec {
    use super::{Sprite, SpriteVersioned};
    use crate::timeline::Timeline;
    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Timeline<Sprite>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Timeline::<SpriteVersioned>::deserialize(deserializer)?
            .into_iter()
            .map(|(sprite, time)| (sprite.into_modern(), time))
            .collect())
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum SpriteVersioned {
    V1(SpriteV1),
    Legacy(SpriteLegacy),
}
impl SpriteVersioned {
    fn into_modern(self) -> Sprite {
        match self {
            SpriteVersioned::V1(value) => value.into_modern(),
            SpriteVersioned::Legacy(value) => value.into_modern(),
        }
    }
}

impl From<SpriteV1> for SpriteVersioned {
    fn from(value: SpriteV1) -> Self {
        Self::V1(value)
    }
}

#[derive(Deserialize)]
struct SpriteLegacy {
    pub offset: Vec2,
    pub rotation: f32,
    pub scale: Vec2,
}

impl SpriteLegacy {
    fn into_modern(self) -> Sprite {
        SpriteV1 {
            image: None,
            modifiers: Modifiers::with_basic(self.rotation, self.scale, self.offset),
        }
        .into_modern()
    }
}
