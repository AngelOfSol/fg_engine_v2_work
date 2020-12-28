use super::{AnimationUi, CancelSetUi, HitboxSetUi, SoundPlayInfoUi, SpawnerUi};
use crate::{
    character::state::{
        components::{CancelSet, Flags, HitboxSet, SoundPlayInfo, StateType},
        SpawnerInfo,
    },
    timeline,
};

use crate::assets::Assets;
use crate::character::state::EditorCharacterState;
use crate::graphics::Animation;
use crate::imgui_extra::UiExtensions;
use crate::timeline::Timeline;
use ggez::Context;
use imgui::*;
use inspect_design::traits::{Inspect, InspectMut};
use nfd::Response;
use std::path::PathBuf;

pub struct StateUi {
    current_animation: Option<usize>,
    current_sound: Option<usize>,
    current_spawner: Option<usize>,
    current_hitbox_ui: Option<HitboxSetUi>,
    current_cancel_set_ui: Option<CancelSetUi>,
    spawner_ui_data: SpawnerUi,
    flags_state: <Timeline<Flags> as Inspect>::State,
    cancels_state: <Timeline<CancelSet> as Inspect>::State,
    hitbox_state: <Timeline<HitboxSet<String>> as Inspect>::State,
    animation_state: AnimationUi,
}

impl StateUi {
    pub fn new() -> Self {
        Self {
            current_animation: None,
            current_sound: None,
            current_hitbox_ui: None,
            current_cancel_set_ui: None,
            current_spawner: None,
            spawner_ui_data: SpawnerUi::new(),
            flags_state: Default::default(),
            cancels_state: Default::default(),
            hitbox_state: Default::default(),
            animation_state: Default::default(),
        }
    }

    pub fn draw_header(
        &mut self,
        ui: &Ui<'_>,
        state_list: &[String],
        data: &mut EditorCharacterState,
    ) {
        ui.label_text(im_str!("Duration"), &im_str!("{}", data.duration()));

        ui.combo_items(
            im_str!("State Type"),
            &mut data.state_type,
            StateType::all(),
            &|item| im_str!("{}", item).into(),
        );

        ui.combo_items(
            im_str!("On Expire"),
            &mut data.on_expire.state_id,
            &state_list,
            &|item| im_str!("{}", item).into(),
        );

        ui.same_line(0.0);
        let _ = ui.input_whole(im_str!("Frame"), &mut data.on_expire.frame);
    }
    pub fn draw_animation_editor(
        &mut self,
        ctx: &mut Context,
        assets: &mut Assets,
        ui: &Ui<'_>,
        data: &mut Vec<Animation>,
    ) -> Option<String> {
        let mut ret = None;

        let id = ui.push_id("Animations");
        ui.rearrangable_list_box(
            im_str!("List"),
            &mut self.current_animation,
            data,
            |item| im_str!("{}", item.name.to_owned()),
            5,
        );
        if ui.small_button(im_str!("Load")) {
            let paths = match nfd::open_file_multiple_dialog(Some("json"), None) {
                Ok(Response::Okay(path)) => vec![path],
                Ok(Response::OkayMultiple(paths)) => paths,
                _ => vec![],
            };
            for path in paths {
                data.push(Animation::load_from_json(ctx, assets, PathBuf::from(path)).unwrap());
            }
        }

        ui.same_line(0.0);
        if ui.small_button(im_str!("New")) {
            // TODO add popup?
            data.push(Animation::new("new"));
        }

        if let Some(animation) = self.current_animation {
            if let Some(animation) = data.get_mut(animation) {
                ui.same_line(0.0);
                if ui.small_button(im_str!("Edit")) {
                    ret = Some(animation.name.clone());
                }
            }
            ui.same_line(0.0);
            if ui.small_button(im_str!("Delete")) {
                data.remove(animation);
            }
            if let Some(animation) = data.get_mut(animation) {
                ui.separator();
                self.animation_state.draw_ui(ui, animation);
            }
        }
        id.pop(ui);

        ret
    }

    pub fn draw_spawner_editor(&mut self, ui: &Ui<'_>, data: &mut Vec<SpawnerInfo>) {
        let id = ui.push_id("Spawner");
        if let (_, Some(spawner)) = ui.new_delete_list_box(
            im_str!("List"),
            &mut self.current_spawner,
            data,
            |item| im_str!("(frame {})", item.frame),
            SpawnerInfo::default,
            |_| {},
            5,
        ) {
            self.spawner_ui_data.draw_ui(ui, spawner);
        }
        id.pop(ui);
    }
    pub fn draw_sounds_editor(
        &mut self,
        ui: &Ui<'_>,
        sounds_list: &[String],
        data: &mut Vec<SoundPlayInfo<String>>,
    ) {
        if !sounds_list.is_empty() {
            let id = ui.push_id("Sounds");
            let default_bullet = sounds_list[0].clone();
            if let (_, Some(sound)) = ui.new_delete_list_box(
                im_str!("List"),
                &mut self.current_sound,
                data,
                |item| im_str!("{} @ {} (frame {})", item.name, item.channel, item.frame),
                || SoundPlayInfo::new(default_bullet.clone().into()),
                |_| {},
                5,
            ) {
                SoundPlayInfoUi::draw_ui(ui, sounds_list, sound);
            }
            id.pop(ui);
        }
    }

    pub fn draw_flags_editor(&mut self, ui: &Ui<'_>, data: &mut Timeline<Flags>) {
        data.inspect_mut("flags", &mut self.flags_state, ui);
    }
    pub fn draw_cancels_editor(&mut self, ui: &Ui<'_>, data: &mut Timeline<CancelSet>) {
        let id = ui.push_id("Cancels");
        let current_cancel_set_ui = &mut self.current_cancel_set_ui;

        timeline::inspect::inspect_mut_custom(
            data,
            "cancels",
            &mut self.cancels_state,
            ui,
            |_, data| {
                if current_cancel_set_ui.is_none() {
                    *current_cancel_set_ui = Some(CancelSetUi::new());
                }
                let ui_data = current_cancel_set_ui.as_mut().unwrap();

                ui.separator();
                imgui::ChildWindow::new(im_str!("child frame"))
                    .size([0.0, 0.0])
                    .build(ui, || {
                        ui_data.draw_ui(ui, data);
                    });
            },
        );

        id.pop(ui);
    }

    pub fn draw_hitbox_editor(
        &mut self,
        ui: &Ui<'_>,
        data: &mut Timeline<HitboxSet<String>>,
        attack_ids: &[String],
    ) {
        let id = ui.push_id("Hitboxes");

        let current_hitbox_ui = &mut self.current_hitbox_ui;

        timeline::inspect::inspect_mut_custom(
            data,
            "hitboxes",
            &mut self.hitbox_state,
            ui,
            |_, data| {
                let ui_data = current_hitbox_ui.get_or_insert_with(HitboxSetUi::new);

                ui.separator();
                ui_data.draw_ui(ui, data, attack_ids);
            },
        );

        id.pop(ui);
    }
}
