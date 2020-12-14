use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Display, EnumIter, Copy, Hash)]
pub enum GlobalGraphic {
    SuperJump,
}

impl Default for GlobalGraphic {
    fn default() -> Self {
        Self::SuperJump
    }
}
