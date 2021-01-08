mod butterfly;

pub use butterfly::*;

use inspect_design::Inspect;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

#[derive(
    Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize, EnumIter, Display, Inspect,
)]
pub enum ObjectData {
    Butterfly,
}
impl Default for ObjectData {
    fn default() -> Self {
        Self::Butterfly
    }
}
