use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

use crate::game_object::constructors::Inspect;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Display, EnumIter, Copy, Hash)]
pub enum GlobalGraphic {
    SuperJump,
}
impl Inspect for GlobalGraphic {
    fn inspect_mut(&mut self, ui: &imgui::Ui<'_>) {
        use crate::imgui_extra::UiExtensions;
        ui.combo_enum(imgui::im_str!("Value"), self);
    }
}

impl Default for GlobalGraphic {
    fn default() -> Self {
        Self::SuperJump
    }
}
