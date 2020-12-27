use crate::graphics::Animation;
use crate::imgui_extra::UiExtensions;

use crate::ui::graphics::modifiers::ModifiersUi;
use imgui::*;

#[derive(Default)]
pub struct AnimationUi {
    modifiers_state: ModifiersUi,
}

impl AnimationUi {
    pub fn draw_ui(&mut self, ui: &Ui<'_>, data: &mut Animation) {
        ui.label_text(im_str!("Name"), &im_str!("{}", data.name));
        let _ = ui.input_whole(im_str!("Delay"), &mut data.delay);

        self.modifiers_state.draw_ui(ui, &mut data.modifiers);
    }
}
