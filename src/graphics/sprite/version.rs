use super::{Sprite, SpriteV1};
use crate::graphics::keyframe::{Coordinates, EaseType, Keyframe, Keyframes, Modifiers};
use crate::typedefs::graphics::Vec2;
use serde::{Deserialize, Deserializer};

pub fn deserialize_versioned<'de, D>(deserializer: D) -> Result<Sprite, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(SpriteVersioned::deserialize(deserializer)?.to_modern())
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum SpriteVersioned {
    V1(SpriteV1),
    Legacy(SpriteLegacy),
}

impl SpriteVersioned {
    pub fn to_modern(self) -> Sprite {
        match self {
            SpriteVersioned::V1(value) => value.to_modern(),
            SpriteVersioned::Legacy(value) => value.to_modern(),
        }
    }
}

#[derive(Deserialize)]
pub struct SpriteLegacy {
    pub offset: Vec2,
    pub rotation: f32,
    pub scale: Vec2,
}

impl SpriteLegacy {
    fn to_modern(self) -> Sprite {
        SpriteV1 {
            image: None,
            modifiers: Modifiers {
                rotation: Keyframes::new(Keyframe {
                    frame: 0,
                    value: self.rotation,
                    function: EaseType::Constant,
                }),
                coords: [
                    Keyframes::new(Keyframe {
                        frame: 0,
                        value: self.offset.x,
                        function: EaseType::Constant,
                    }),
                    Keyframes::new(Keyframe {
                        frame: 0,
                        value: self.offset.y,
                        function: EaseType::Constant,
                    }),
                ],
                scale: [
                    Keyframes::new(Keyframe {
                        frame: 0,
                        value: self.scale.x,
                        function: EaseType::Constant,
                    }),
                    Keyframes::new(Keyframe {
                        frame: 0,
                        value: self.scale.y,
                        function: EaseType::Constant,
                    }),
                ],
                coord_type: Coordinates::Cartesian,
            },
        }
        .to_modern()
    }
}
