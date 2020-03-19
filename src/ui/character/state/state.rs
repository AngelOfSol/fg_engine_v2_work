use super::{
    AnimationDataUi, BulletSpawnUi, CancelSetUi, FlagsUi, HitboxSetUi, ParticleSpawnUi,
    SoundPlayInfoUi,
};
use crate::character::state::components::{
    AnimationData, BulletSpawn, CancelSet, Flags, HitboxSet, MoveType, ParticleSpawn, SoundPlayInfo,
};

use crate::assets::Assets;
use crate::character::state::EditorCharacterState;
use crate::graphics::Animation;
use crate::imgui_extra::UiExtensions;
use crate::timeline::Timeline;
use ggez::Context;
use imgui::*;
use nfd::Response;
use std::cmp;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

pub struct StateUi {
    current_animation: Option<usize>,
    current_flags: Option<usize>,
    current_cancels: Option<usize>,
    current_particle: Option<usize>,
    current_bullet: Option<usize>,
    current_hitboxes: Option<usize>,
    current_hitbox_ui: Option<HitboxSetUi>,
    current_cancel_set_ui: Option<CancelSetUi>,
    particle_ui_data: ParticleSpawnUi,
}

impl StateUi {
    pub fn new() -> Self {
        Self {
            current_animation: None,
            current_flags: None,
            current_cancels: None,
            current_particle: None,
            current_bullet: None,
            current_hitboxes: None,
            current_hitbox_ui: None,
            current_cancel_set_ui: None,
            particle_ui_data: ParticleSpawnUi::new(),
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
            MoveType::all(),
            &|item| im_str!("{}", item).into(),
        );

        ui.combo_items(
            im_str!("On Expire"),
            &mut data.on_expire_state,
            &state_list,
            &|item| im_str!("{}", item).into(),
        );
        ui.input_whole(
            im_str!("Minmum Spirit Requried"),
            &mut data.minimum_spirit_required,
        )
        .unwrap();
    }
    pub fn draw_animation_editor(
        &mut self,
        ctx: &mut Context,
        assets: &mut Assets,
        ui: &Ui<'_>,
        data: &mut Vec<AnimationData>,
    ) -> Option<String> {
        let mut ret = None;

        let id = ui.push_id("Animations");
        ui.rearrangable_list_box(
            im_str!("List"),
            &mut self.current_animation,
            data,
            |item| im_str!("{}", item.name().to_owned()),
            5,
        );
        if ui.small_button(im_str!("Load")) {
            let paths = match nfd::open_file_multiple_dialog(Some("json"), None) {
                Ok(Response::Okay(path)) => vec![path],
                Ok(Response::OkayMultiple(paths)) => paths,
                _ => vec![],
            };
            for path in paths {
                data.push(AnimationData::new(
                    Animation::load_from_json(ctx, assets, PathBuf::from(path)).unwrap(),
                ));
            }
        }

        ui.same_line(0.0);
        if ui.small_button(im_str!("New")) {
            // TODO add popup?
            data.push(AnimationData::new(Animation::new("new")));
        }

        if let Some(animation) = self.current_animation {
            if let Some(animation) = data.get_mut(animation) {
                ui.same_line(0.0);
                if ui.small_button(im_str!("Edit")) {
                    ret = Some(animation.animation.name.clone());
                }
            }
            ui.same_line(0.0);
            if ui.small_button(im_str!("Delete")) {
                data.remove(animation);
            }
            if let Some(animation) = data.get_mut(animation) {
                ui.separator();
                AnimationDataUi::draw_ui(ui, animation);
            }
        }
        id.pop(ui);

        ret
    }

    pub fn draw_particle_editor(
        &mut self,
        ui: &Ui<'_>,
        particle_list: &[String],
        data: &mut Vec<ParticleSpawn<String>>,
    ) {
        if !particle_list.is_empty() {
            let id = ui.push_id("Particles");
            let default_particle = particle_list[0].clone();
            if let (_, Some(particle)) = ui.new_delete_list_box(
                im_str!("List"),
                &mut self.current_particle,
                data,
                |item| im_str!("{}", item.particle_id.clone()),
                || ParticleSpawn::new(default_particle.clone()),
                |_| {},
                5,
            ) {
                self.particle_ui_data.draw_ui(ui, particle_list, particle);
            }
            id.pop(ui);
        }
    }
    pub fn draw_bullet_editor(
        &mut self,
        ui: &Ui<'_>,
        bullet_list: &HashMap<String, HashSet<String>>,
        data: &mut Vec<BulletSpawn>,
    ) {
        if !bullet_list.is_empty() {
            let id = ui.push_id("Bullets");
            let default_bullet = bullet_list.keys().next().cloned().unwrap();
            let default_properties = bullet_list[&default_bullet].clone();
            if let (_, Some(bullet)) = ui.new_delete_list_box(
                im_str!("List"),
                &mut self.current_bullet,
                data,
                |item| im_str!("{} (frame {})", item.bullet_id.clone(), item.frame),
                || BulletSpawn::new(default_bullet.clone(), &default_properties),
                |_| {},
                5,
            ) {
                BulletSpawnUi::draw_ui(ui, bullet, &bullet_list);
            }
            id.pop(ui);
        }
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
                &mut self.current_bullet,
                data,
                |item| im_str!("{} @ {} (frame {})", item.name, item.channel, item.frame),
                || SoundPlayInfo::new(default_bullet.clone()),
                |_| {},
                5,
            ) {
                SoundPlayInfoUi::draw_ui(ui, sounds_list, sound);
            }
            id.pop(ui);
        }
    }

    pub fn draw_flags_editor(&mut self, ui: &Ui<'_>, data: &mut Timeline<Flags>) {
        let id = ui.push_id("Flags");
        let mut counter = 0;
        ui.rearrangable_list_box(
            im_str!("List\n[Start, End]"),
            &mut self.current_flags,
            data,
            |(_, duration)| {
                let start = counter;
                let end = counter + duration - 1;
                counter += duration;
                im_str!("[{}, {}]", start, end)
            },
            5,
        );

        if let Some(ref mut idx) = self.current_flags {
            ui.timeline_modify(idx, data);

            let (ref mut flags, ref mut duration) = &mut data[*idx];

            let _ = ui.input_whole(im_str!("Duration"), duration);
            *duration = cmp::max(*duration, 1);

            ui.separator();
            FlagsUi::draw_ui(ui, flags);
        }

        id.pop(ui);
    }
    pub fn draw_cancels_editor(
        &mut self,
        ui: &Ui<'_>,
        state_list: &[String],
        data: &mut Timeline<CancelSet<String>>,
    ) {
        let id = ui.push_id("Cancels");
        let mut counter = 0;
        ui.rearrangable_list_box(
            im_str!("List\n[Start, End]"),
            &mut self.current_cancels,
            data,
            |(_, duration)| {
                let start = counter;
                let end = counter + duration - 1;
                counter += duration;
                im_str!("[{}, {}]", start, end)
            },
            5,
        );

        if let Some(ref mut idx) = self.current_cancels {
            if self.current_cancel_set_ui.is_none() {
                self.current_cancel_set_ui = Some(CancelSetUi::new(state_list[0].clone()));
            }
            let ui_data = self.current_cancel_set_ui.as_mut().unwrap();
            ui.timeline_modify(idx, data);

            let (ref mut cancels, ref mut duration) = &mut data[*idx];

            let _ = ui.input_whole(im_str!("Duration"), duration);
            *duration = cmp::max(*duration, 1);

            ui.separator();
            imgui::ChildWindow::new(im_str!("child frame"))
                .size([0.0, 0.0])
                .build(ui, || {
                    ui_data.draw_ui(ui, state_list, cancels);
                });
        }
        id.pop(ui);
    }

    pub fn draw_hitbox_editor(
        &mut self,
        ui: &Ui<'_>,
        data: &mut Timeline<HitboxSet<String>>,
        attack_ids: &[String],
    ) {
        let id = ui.push_id("Hitboxes");
        let mut counter = 0;
        let format_entry = |(_, duration): &(_, usize)| {
            let start = counter;
            let end = counter + duration - 1;
            counter += duration;
            im_str!("[{}, {}]", start, end)
        };
        if ui.rearrangable_list_box(
            im_str!("List\n[Start, End]"),
            &mut self.current_hitboxes,
            data,
            format_entry,
            5,
        ) {
            self.current_hitbox_ui = None;
        }

        if let Some(ref mut idx) = self.current_hitboxes {
            let ui_data = self.current_hitbox_ui.get_or_insert_with(HitboxSetUi::new);

            ui.timeline_modify(idx, data);

            let (ref mut hitboxes, ref mut duration) = &mut data[*idx];

            let _ = ui.input_whole(im_str!("Duration"), duration);
            *duration = cmp::max(*duration, 1);

            ui.separator();
            ui_data.draw_ui(ui, hitboxes, attack_ids);
        }

        id.pop(ui);
    }
}
