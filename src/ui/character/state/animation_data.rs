use crate::graphics::{keyframe::Modifiers, Animation};
use crate::imgui_extra::UiExtensions;
use imgui::*;
use inspect_design::traits::*;

#[derive(Default)]
pub struct AnimationUi {
    modifiers_state: <Modifiers as Inspect>::State,
}

impl AnimationUi {
    pub fn draw_ui(&mut self, ui: &Ui<'_>, data: &mut Animation) {
        ui.label_text(im_str!("Name"), &im_str!("{}", data.name));
        let _ = ui.input_whole(im_str!("Delay"), &mut data.delay);

        data.modifiers
            .inspect_mut("modifiers", &mut self.modifiers_state, ui);
    }
}
