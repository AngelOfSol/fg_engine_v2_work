use ggez::graphics;
use inspect_design::Inspect;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Inspect)]
pub enum BlendMode {
    Alpha,
    Add,
}

impl Default for BlendMode {
    fn default() -> Self {
        Self::Alpha
    }
}

impl Into<graphics::BlendMode> for BlendMode {
    fn into(self) -> graphics::BlendMode {
        match self {
            BlendMode::Add => graphics::BlendMode::Add,
            BlendMode::Alpha => graphics::BlendMode::Alpha,
        }
    }
}
