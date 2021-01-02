use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize, EnumIter, Display)]
pub enum YuyukoDataId {
    Butterfly,
}
impl Default for YuyukoDataId {
    fn default() -> Self {
        Self::Butterfly
    }
}
