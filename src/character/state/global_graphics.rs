use std::collections::HashMap;

use crate::graphics::animation_group::AnimationGroup;
use inspect_design::Inspect;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Display, EnumIter, Copy, Hash, Inspect,
)]
pub enum GlobalGraphic {
    SuperJump,
}

impl Default for GlobalGraphic {
    fn default() -> Self {
        Self::SuperJump
    }
}

pub type GlobalGraphicMap = HashMap<GlobalGraphic, AnimationGroup>;
