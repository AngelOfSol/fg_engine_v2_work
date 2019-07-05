use crate::hitbox::Hitbox;

use serde::{Deserialize, Serialize};

use crate::imgui_extra::UiExtensions;
use imgui::*;

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct HitboxSet {
    pub collision: Hitbox,
}

impl HitboxSet {
    pub fn new() -> Self {
        Self {
            collision: Hitbox::new(),
        }
    }
}
pub struct HitboxSetUi;

impl HitboxSetUi {
    pub fn new() -> Self {
        Self
    }

    pub fn draw_ui(&mut self, ui: &Ui<'_>, data: &mut HitboxSet) {
        if ui.collapsing_header(im_str!("Collision")).build() {
            Hitbox::draw_ui(ui, &mut data.collision);
        }
    }
}
