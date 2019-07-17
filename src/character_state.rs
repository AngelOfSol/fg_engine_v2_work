pub mod animation_data;
pub mod cancel_set;
pub mod flags;
pub mod hitbox_set;
pub mod particle_spawn_data;

use crate::animation::Animation;
use crate::assets::Assets;

use crate::imgui_extra::UiExtensions;

use imgui::*;

use ggez::{Context, GameResult};
use serde::{Deserialize, Serialize};

use std::cmp;

use nfd::Response;

pub use animation_data::{AnimationData, AnimationDataUi};
pub use cancel_set::{CancelSet, CancelSetUi, MoveType};
pub use flags::{Flags, FlagsUi, MovementData};
pub use hitbox_set::{HitboxSet, HitboxSetUi};
pub use particle_spawn_data::ParticleSpawn;

use crate::timeline::{AtTime, Timeline};

use crate::typedefs::graphics::Matrix4;

use crate::typedefs::{HashId, StateId};

use crate::editor::Mode;

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use ggez::GameError;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CharacterState<Id>
where
    Id: HashId,
{
    pub animations: Vec<AnimationData>,
    pub flags: Timeline<Flags>,
    pub cancels: Timeline<CancelSet<Id>>,
    pub hitboxes: Timeline<HitboxSet>,
    #[serde(default)]
    pub particles: Vec<ParticleSpawn>,
    #[serde(default = "default_move_type")]
    pub state_type: MoveType,
    #[serde(default)]
    pub on_expire_state: Id,
}
fn default_move_type() -> MoveType {
    MoveType::Idle
}

impl<Id: StateId> CharacterState<Id> {
    pub fn load_from_json(
        ctx: &mut Context,
        assets: &mut Assets,
        mut path: PathBuf,
    ) -> GameResult<Self> {
        let file = File::open(&path).unwrap();
        let buf_read = BufReader::new(file);
        let state = serde_json::from_reader::<_, Self>(buf_read).unwrap();
        let name = path.file_stem().unwrap().to_str().unwrap().to_owned();
        path.pop();
        CharacterState::load(ctx, assets, &state, &name, path)?;
        Ok(state)
    }
    pub fn load(
        ctx: &mut Context,
        assets: &mut Assets,
        state: &Self,
        name: &str,
        mut path: PathBuf,
    ) -> GameResult<()> {
        path.push(name);
        for animation in &state.animations {
            Animation::load(ctx, assets, &animation.animation, path.clone())?;
        }
        Ok(())
    }
    pub fn save(
        ctx: &mut Context,
        assets: &mut Assets,
        state: &Self,
        mut path: PathBuf,
    ) -> GameResult<()> {
        let name = path.file_stem().unwrap().to_str().unwrap().to_owned();

        let mut json = File::create(&path)?;
        serde_json::to_writer(&mut json, &state)
            .map_err(|err| GameError::FilesystemError(format!("{}", err)))?;

        path.pop();
        path.push(&name);
        std::fs::create_dir_all(&path)?;
        for animation in &state.animations {
            path.push(&format!("{}.json", &animation.animation.name));
            Animation::save(ctx, assets, &animation.animation, path.clone())?;
            path.pop();
        }
        Ok(())
    }

    pub fn duration(&self) -> usize {
        self.animations
            .iter()
            .map(|item| item.delay + item.duration())
            .fold(0, cmp::max)
    }

    pub fn fix_duration(&mut self) {
        if self.duration() > 0 {
            self.flags.fix_duration(self.duration());
            self.cancels.fix_duration(self.duration());
            self.hitboxes.fix_duration(self.duration());
        }
    }

    pub fn draw_at_time(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        time: usize,
        world: Matrix4,
    ) -> GameResult<()> {
        if time < self.duration() {
            for animation in self.animations.iter() {
                animation.draw_at_time(ctx, assets, time, world)?
            }
        }
        Ok(())
    }
    pub fn draw_at_time_debug(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        time: usize,
        world: Matrix4,
    ) -> GameResult<()> {
        if time < self.duration() {
            for animation in self.animations.iter() {
                animation.draw_at_time_debug(ctx, assets, time, world)?
            }
        }
        Ok(())
    }
}

impl CharacterState<String> {
    pub fn new() -> Self {
        Self {
            animations: vec![],
            flags: vec![(Flags::new(), 1)],
            cancels: vec![(CancelSet::new(), 1)],
            hitboxes: vec![(HitboxSet::new(), 1)],
            state_type: default_move_type(),
            on_expire_state: "stand".to_owned(),
            particles: Vec::new(),
        }
    }
}

pub struct CharacterStateUi {
    current_animation: Option<usize>,
    current_flags: Option<usize>,
    current_cancels: Option<usize>,
    current_particle: Option<usize>,
    current_hitboxes: Option<usize>,
    current_hitbox_ui: Option<HitboxSetUi>,
    current_cancel_set_ui: Option<CancelSetUi>,
}

impl CharacterStateUi {
    pub fn new() -> Self {
        Self {
            current_animation: None,
            current_flags: None,
            current_cancels: None,
            current_particle: None,
            current_hitboxes: None,
            current_hitbox_ui: None,
            current_cancel_set_ui: None,
        }
    }

    pub fn draw_ui(
        &mut self,
        ctx: &mut Context,
        assets: &mut Assets,
        ui: &Ui<'_>,
        data: &mut CharacterState<String>,
    ) -> GameResult<Option<Mode>> {
        let mut ret = None;

        ui.label_text(im_str!("Duration"), &im_str!("{}", data.duration()));
        let mut buffer = MoveType::all()
            .iter()
            .position(|item| *item == data.state_type)
            .unwrap() as i32;
        ui.combo(
            im_str!("State Type"),
            &mut buffer,
            &MoveType::all()
                .iter()
                .map(|item| im_str!("{}", item))
                .collect::<Vec<_>>()
                .iter()
                .collect::<Vec<_>>(),
            5,
        );
        let _ = ui.input_string(im_str!("On Expire"), &mut data.on_expire_state);

        if buffer >= 0 {
            data.state_type = MoveType::all()[buffer as usize];
        }

        if ui.collapsing_header(im_str!("Animations")).build() {
            ui.push_id("Animations");
            ui.rearrangable_list_box(
                im_str!("List"),
                &mut self.current_animation,
                &mut data.animations,
                |item| im_str!("{}", item.name().to_owned()),
                5,
            );
            if ui.small_button(im_str!("Add")) {
                let path_result = nfd::open_file_multiple_dialog(Some("json"), None);
                match path_result {
                    Ok(path) => match path {
                        Response::Cancel => (),
                        Response::Okay(path) => {
                            data.animations.push(AnimationData::new(
                                Animation::load_from_json(ctx, assets, PathBuf::from(path))
                                    .unwrap(),
                            ));
                        }
                        Response::OkayMultiple(paths) => {
                            for path in paths {
                                data.animations.push(AnimationData::new(
                                    Animation::load_from_json(ctx, assets, PathBuf::from(path))
                                        .unwrap(),
                                ));
                            }
                        }
                    },
                    Err(err) => {
                        dbg!(err);
                    }
                }
            }
            ui.same_line(0.0);
            if ui.small_button(im_str!("New")) {
                ret = Some(Mode::New);
            }
            if let Some(animation) = self.current_animation {
                if let Some(animation) = &mut data.animations.get_mut(animation) {
                    ui.same_line(0.0);
                    if ui.small_button(im_str!("Edit")) {
                        ret = Some(Mode::Edit(animation.animation.name.clone()));
                    }
                    AnimationDataUi::new().draw_ui(ui, animation)?;
                }
            }
            ui.separator();
            ui.pop_id();
        }
        data.fix_duration();

        if ui.collapsing_header(im_str!("Particles")).build() {
            ui.push_id("Particles");
            if let Some(particle) = ui.new_delete_list_box(
                im_str!("List"),
                &mut self.current_particle,
                &mut data.particles,
                |item| im_str!("{}", item.particle_id.clone()),
                ParticleSpawn::new,
                |_| {},
                5,
            ) {
                ui.input_string(im_str!("ID"), &mut particle.particle_id);

                let _ = ui.input_whole(im_str!("Spawn Frame"), &mut particle.frame);
                if ui
                    .collapsing_header(im_str!("Offset"))
                    .default_open(true)
                    .build()
                {
                    particle.offset.x /= 100;
                    particle.offset.y /= 100;
                    let _ = ui.input_whole(im_str!("X##Offset"), &mut particle.offset.x);
                    let _ = ui.input_whole(im_str!("Y##Offset"), &mut particle.offset.y);
                    particle.offset.x *= 100;
                    particle.offset.y *= 100;
                }
            }
            ui.pop_id();
        }

        if ui.collapsing_header(im_str!("Flags")).build() {
            ui.push_id("Flags");
            let mut counter = 0;
            ui.rearrangable_list_box(
                im_str!("List\n[Start, End]"),
                &mut self.current_flags,
                &mut data.flags,
                |(_, duration)| {
                    let start = counter;
                    let end = counter + duration - 1;
                    counter += duration;
                    im_str!("[{}, {}]", start, end)
                },
                5,
            );

            if let Some(ref mut idx) = self.current_flags {
                ui.timeline_modify(idx, &mut data.flags);

                let (ref mut flags, ref mut duration) = &mut data.flags[*idx];

                let _ = ui.input_whole(im_str!("Duration"), duration);
                *duration = cmp::max(*duration, 1);

                ui.separator();
                FlagsUi::new().draw_ui(ui, flags);
            }

            ui.separator();
            ui.pop_id();
        }
        if ui.collapsing_header(im_str!("Cancels")).build() {
            ui.push_id("Cancels");
            let mut counter = 0;
            ui.rearrangable_list_box(
                im_str!("List\n[Start, End]"),
                &mut self.current_cancels,
                &mut data.cancels,
                |(_, duration)| {
                    let start = counter;
                    let end = counter + duration - 1;
                    counter += duration;
                    im_str!("[{}, {}]", start, end)
                },
                5,
            );

            if let Some(ref mut idx) = self.current_cancels {
                let ui_data = self
                    .current_cancel_set_ui
                    .get_or_insert_with(CancelSetUi::new);
                ui.timeline_modify(idx, &mut data.cancels);

                let (ref mut cancels, ref mut duration) = &mut data.cancels[*idx];

                let _ = ui.input_whole(im_str!("Duration"), duration);
                *duration = cmp::max(*duration, 1);

                ui.separator();
                ui_data.draw_ui(ui, cancels);
            }
            ui.separator();
            ui.pop_id();
        }
        if ui.collapsing_header(im_str!("Hitboxes")).build() {
            ui.push_id("Hitboxes");
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
                &mut data.hitboxes,
                format_entry,
                5,
            ) {
                self.current_hitbox_ui = None;
            }

            if let Some(ref mut idx) = self.current_hitboxes {
                let ui_data = self.current_hitbox_ui.get_or_insert_with(HitboxSetUi::new);

                ui.timeline_modify(idx, &mut data.hitboxes);

                let (ref mut hitboxes, ref mut duration) = &mut data.hitboxes[*idx];

                let _ = ui.input_whole(im_str!("Duration"), duration);
                *duration = cmp::max(*duration, 1);

                ui.separator();
                ui_data.draw_ui(ui, hitboxes);
            }

            ui.separator();
            ui.pop_id();
        }
        Ok(ret)
    }
}
