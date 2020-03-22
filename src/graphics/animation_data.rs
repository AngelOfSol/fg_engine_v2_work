use crate::graphics::{animation::AnimationV1, keyframe::Modifiers, Animation};
use crate::typedefs::graphics::Vec2;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AnimationData {
    pub animation: AnimationV1,
    pub delay: usize,
    pub offset: Vec2,
    pub scale: Vec2,
}

impl AnimationData {
    pub fn to_modern(self) -> Animation {
        AnimationV1 {
            delay: self.delay,
            modifiers: Modifiers::with_basic(0.0, self.scale, self.offset),

            ..self.animation
        }
        .to_modern()
    }
}
