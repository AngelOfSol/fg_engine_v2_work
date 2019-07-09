use crate::imgui_extra::UiExtensions;
use imgui::*;

use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Properties {
    pub health: u32,
    pub name: String,
    #[serde(skip)]
    _secret: (),
}

impl Properties {
    pub fn new() -> Self {
        Self {
            health: 1,
            name: "new_chara".to_owned(),
            _secret: (),
        }
    }
}

pub struct PropertiesUi {}
impl PropertiesUi {
    pub fn draw_ui(ui: &Ui<'_>, data: &mut Properties) {
        let _ = ui.input_whole(im_str!("Health"), &mut data.health);
        ui.input_string(im_str!("Name"), &mut data.name);
    }
}
