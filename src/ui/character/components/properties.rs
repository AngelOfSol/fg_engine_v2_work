use crate::character::components::Properties;
use crate::imgui_extra::UiExtensions;
use crate::roster::RosterCharacter;
use imgui::*;
use strum::IntoEnumIterator;

pub struct PropertiesUi {}
impl PropertiesUi {
    pub fn draw_ui(ui: &Ui<'_>, data: &mut Properties) {
        let _ = ui.input_whole(im_str!("Health"), &mut data.health);
        ui.input_string(im_str!("Name"), &mut data.name);

        ui.input_whole(im_str!("Max Air Actions"), &mut data.max_air_actions)
            .unwrap();
        ui.input_whole(im_str!("Max Spirit Gauge"), &mut data.max_spirit_gauge)
            .unwrap();

        ui.combo_items(
            im_str!("Character"),
            &mut data.character,
            &RosterCharacter::iter().collect::<Vec<_>>(),
            &|item| im_str!("{}", item.to_string()).into(),
        );
    }
}
