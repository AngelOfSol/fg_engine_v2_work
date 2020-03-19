use crate::character::state::components::SoundPlayInfo;
use crate::game_match::sounds::ChannelName;
use crate::imgui_extra::UiExtensions;
use imgui::*;
use strum::IntoEnumIterator;

pub struct SoundPlayInfoUi {}

impl SoundPlayInfoUi {
    pub fn draw_ui(ui: &Ui<'_>, sound_list: &[String], data: &mut SoundPlayInfo<String>) {
        ui.combo_items(im_str!("Name"), &mut data.name, &sound_list, &|item| {
            im_str!("{}", item).into()
        });
        ui.combo_items(
            im_str!("Channel"),
            &mut data.channel,
            &ChannelName::iter().collect::<Vec<_>>(),
            &|item| im_str!("{}", item).into(),
        );

        let _ = ui.input_whole(im_str!("Spawn Frame"), &mut data.frame);
    }
}
