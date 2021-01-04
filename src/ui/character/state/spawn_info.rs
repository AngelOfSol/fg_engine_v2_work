use crate::imgui_extra::UiExtensions;
use crate::{character::state::SpawnerInfo, game_object::constructors::Constructor};
use imgui::*;

use inspect_design::traits::*;

pub struct SpawnerUi {
    state: <Vec<Constructor> as Inspect>::State,
}

impl SpawnerUi {
    pub fn new() -> Self {
        Self {
            state: Default::default(),
        }
    }
    pub fn draw_ui(&mut self, ui: &Ui<'_>, data: &mut SpawnerInfo) {
        let _ = ui.input_whole(im_str!("Spawn Frame"), &mut data.frame);

        data.data.inspect_mut("constructors", &mut self.state, ui);
    }
}
