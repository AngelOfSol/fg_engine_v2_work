use crate::character::state::AnimationData;
use crate::imgui_extra::UiExtensions;
use imgui::*;

pub struct AnimationDataUi;

impl AnimationDataUi {
    pub fn draw_ui(ui: &Ui<'_>, data: &mut AnimationData) {
        ui.label_text(im_str!("Name"), &im_str!("{}", data.animation.name.clone()));
        let _ = ui.input_whole(im_str!("Delay"), &mut data.delay);

        ui.input_vec2_float(im_str!("Offset"), &mut data.offset);
        ui.input_vec2_float(im_str!("Scale"), &mut data.scale);
    }
}
