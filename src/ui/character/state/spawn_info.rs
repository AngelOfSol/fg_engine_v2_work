use crate::imgui_extra::UiExtensions;
use crate::{character::state::SpawnerInfo, game_object::constructors::Constructor};
use imgui::*;

pub struct SpawnerUi {
    current_constructor_type: Constructor,
    current_constructor: Option<usize>,
}

impl SpawnerUi {
    pub fn new() -> Self {
        Self {
            current_constructor_type: Constructor::iter().next().unwrap(),
            current_constructor: None,
        }
    }
    pub fn draw_ui(&mut self, ui: &Ui<'_>, data: &mut SpawnerInfo) {
        let _ = ui.input_whole(im_str!("Spawn Frame"), &mut data.frame);
        ui.separator();
        ui.combo_items(
            im_str!("Types"),
            &mut self.current_constructor_type,
            &Constructor::iter().collect::<Vec<_>>(),
            &|item| im_str!("{}", item).into(),
        );

        let selected_type = self.current_constructor_type.clone();

        let id = ui.push_id("Constructors");
        if let (_, Some(spawner)) = ui.new_delete_list_box(
            im_str!("Constructors"),
            &mut self.current_constructor,
            &mut data.data,
            |item| im_str!("{}", item),
            || selected_type,
            |_| {},
            5,
        ) {}
        id.pop(ui);
    }
}
