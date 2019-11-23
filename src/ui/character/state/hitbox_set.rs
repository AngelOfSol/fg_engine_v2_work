use crate::character::state::components::{AttackData, HitboxSet};
use crate::hitbox::Hitbox;
use crate::imgui_extra::UiExtensions;
use imgui::*;

pub fn draw_attack_ui(
    ui: &Ui<'_>,
    data: &mut AttackData<String>,
    current_attack: &mut Option<usize>,
    attack_ids: &[String],
) {
    let id = ui.push_id("Hitboxes");

    let _ = ui.input_whole(im_str!("ID"), &mut data.id);

    ui.combo_items(
        im_str!("Attack Data##Combo"),
        &mut data.data_id,
        attack_ids,
        &|item| im_str!("{}", item).into(),
    );

    let mut counter = 0;
    ui.rearrangable_list_box(
        im_str!("List\n[Start, End]"),
        current_attack,
        &mut data.boxes,
        |_| {
            counter += 1;
            im_str!("{}", counter)
        },
        5,
    );
    if ui.small_button(im_str!("Add")) {
        data.boxes.push(Hitbox::new());
    }
    if let Some(current_attack_idx) = current_attack {
        ui.same_line(0.0);
        if ui.small_button(im_str!("Delete")) {
            data.boxes.remove(*current_attack_idx);
            if data.boxes.is_empty() {
                *current_attack = None;
            } else {
                *current_attack = Some(std::cmp::min(data.boxes.len() - 1, *current_attack_idx));
            }
        }
    }

    if let Some(ref mut idx) = current_attack {
        let hitbox = &mut data.boxes[*idx];
        Hitbox::draw_ui(ui, hitbox);
    }
    id.pop(ui);
}

pub struct HitboxSetUi {
    current_hurtbox: Option<usize>,
    current_attack: Option<usize>,
}

impl HitboxSetUi {
    pub fn new() -> Self {
        Self {
            current_hurtbox: None,
            current_attack: None,
        }
    }

    pub fn draw_ui(&mut self, ui: &Ui<'_>, data: &mut HitboxSet<String>, attack_ids: &[String]) {
        let id = ui.push_id("Hitbox Set");
        ui.text(im_str!("Collision"));
        {
            let id = ui.push_id("Collision");
            Hitbox::draw_ui(ui, &mut data.collision);
            id.pop(ui);
            ui.separator();
        }

        imgui::ChildWindow::new(im_str!("child frame"))
            .size([0.0, 0.0])
            .build(ui, || {
                ui.text(im_str!("Hurtboxes"));
                let id = ui.push_id("Hurtboxes");
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
                if let Some(current_hurtbox) = self.current_hurtbox {
                    ui.same_line(0.0);
                    if ui.small_button(im_str!("Delete")) {
                        data.hurtbox.remove(current_hurtbox);
                        if data.hurtbox.is_empty() {
                            self.current_hurtbox = None;
                        } else {
                            self.current_hurtbox =
                                Some(std::cmp::min(data.hurtbox.len() - 1, current_hurtbox));
                        }
                    }
                }

                if let Some(ref mut idx) = self.current_hurtbox {
                    let hurtbox = &mut data.hurtbox[*idx];
                    Hitbox::draw_ui(ui, hurtbox);
                }
                id.pop(ui);

                ui.separator();

                let id = ui.push_id("Hitboxes");
                ui.text(im_str!("Hitboxes"));
                {
                    let value = data.hitbox.take();
                    data.hitbox = if let Some(mut hitboxes) = value {
                        if ui.small_button(im_str!("Remove Attack")) {
                            self.current_attack = None;
                            None
                        } else {
                            draw_attack_ui(
                                ui,
                                &mut hitboxes,
                                &mut self.current_attack,
                                &attack_ids,
                            );
                            Some(hitboxes)
                        }
                    } else if !attack_ids.is_empty() && ui.small_button(im_str!("Create Attack")) {
                        Some(AttackData::new(attack_ids[0].clone()))
                    } else {
                        None
                    };
                }
                id.pop(ui);
            });
        id.pop(ui);
    }
}
