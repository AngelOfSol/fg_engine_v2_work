use crate::hitbox::Hitbox;

use serde::{Deserialize, Serialize};

use crate::typedefs::collision::Vec2;

use crate::imgui_extra::UiExtensions;
use imgui::*;

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct HitboxSet {
    pub collision: Hitbox,
    pub hurtbox: Vec<Hitbox>,
    pub hitbox: Vec<Hitbox>,
}

impl HitboxSet {
    pub fn new() -> Self {
        Self {
            collision: Hitbox::with_half_size(Vec2::new(1_000, 5_000)),
            hurtbox: vec![],
            hitbox: vec![],
        }
    }
}
pub struct HitboxSetUi {
    current_hurtbox: Option<usize>,
}

impl HitboxSetUi {
    pub fn new() -> Self {
        Self {
            current_hurtbox: None,
        }
    }

    pub fn draw_ui(&mut self, ui: &Ui<'_>, data: &mut HitboxSet) {
        ui.push_id("HitboxSet");
        if ui.collapsing_header(im_str!("Collision")).build() {
            Hitbox::draw_ui(ui, &mut data.collision);
        }
        if ui.collapsing_header(im_str!("Hurtboxes")).build() {
            ui.push_id("Hurtboxes");
            let mut counter = 0;
            ui.rearrangable_list_box(
                im_str!("List\n[Start, End]"),
                &mut self.current_hurtbox,
                &mut data.hurtbox,
                |_| {
                    counter += 1;
                    im_str!("{}", counter)
                },
                5,
            );
            if ui.small_button(im_str!("Add")) {
                data.hurtbox.push(Hitbox::new());
            }

            if let Some(ref mut idx) = self.current_hurtbox {
                let hurtbox = &mut data.hurtbox[*idx];
                ui.same_line(0.0);
                if ui.small_button(im_str!("Delete")) {
                    //data.hurtbox.
                }
                Hitbox::draw_ui(ui, hurtbox);
            }
            ui.pop_id();
            //Hitbox::draw_ui(ui, &mut data.collision);
        }
        ui.pop_id();
    }
}
