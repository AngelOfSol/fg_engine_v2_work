use ggez::graphics;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum BlendMode {
    Alpha,
    Add,
}

impl Into<graphics::BlendMode> for BlendMode {
    fn into(self) -> graphics::BlendMode {
        match self {
            BlendMode::Add => graphics::BlendMode::Lighten,
            BlendMode::Alpha => graphics::BlendMode::Alpha,
        }
    }
}
