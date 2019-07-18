use crate::editor::{EditorState, MessageData, Mode, Transition};

use ggez::graphics;
use ggez::graphics::{Color, DrawParam, Mesh};
use ggez::{Context, GameResult};

use crate::assets::Assets;
use crate::timeline::AtTime;

use crate::imgui_wrapper::ImGuiWrapper;

use crate::typedefs::graphics::{Matrix4, Vec3};

use crate::typedefs::collision::IntoGraphical;

use crate::character_state::{
    AnimationData, CancelSetUi, CharacterState, CharacterStateUi, FlagsUi, MovementData,
};

use crate::editor::AnimationEditor;

use crate::graphics::Animation;

use crate::imgui_extra::UiExtensions;
use imgui::*;

use std::path::PathBuf;

pub struct StateEditor {
    resource: CharacterState<String, String>,
    frame: usize,
    is_playing: bool,
    transition: Transition,
    ui_data: CharacterStateUi,
    draw_mode: DrawMode,
}
struct DrawMode {
    collision_alpha: f32,
    hurtbox_alpha: f32,
    hitbox_alpha: f32,
    debug_animation: bool,
    show_axes: bool,
}

impl StateEditor {
    pub fn with_state(state: CharacterState<String, String>, particle_list: Vec<String>) -> Self {
        Self {
            resource: state,
            transition: Transition::None,
            frame: 0,
            is_playing: true,
            ui_data: CharacterStateUi::new(particle_list),
            draw_mode: DrawMode {
                collision_alpha: 0.15,
                hurtbox_alpha: 0.15,
                hitbox_alpha: 0.15,
                debug_animation: true,
                show_axes: true,
            },
        }
    }

    pub fn handle_message(&mut self, data: MessageData, mode: Mode) {
        if let MessageData::Animation(mut animation) = data {
            let mut temp_name = animation.name.clone();
            let mut counter = 1;
            loop {
                if self
                    .resource
                    .animations
                    .iter()
                    .map(|item| &item.animation.name)
                    .any(|name| name == &temp_name)
                {
                    temp_name = format!("{} ({})", &animation.name, counter);
                    counter += 1;
                } else {
                    break;
                }
            }
            match mode {
                Mode::Standalone => (),
                Mode::New => {
                    animation.name = temp_name;
                    self.resource.animations.push(AnimationData::new(animation));
                }
                Mode::Edit(name) => {
                    let index = self
                        .resource
                        .animations
                        .iter()
                        .position(|item| item.animation.name == name);
                    match index {
                        Some(index) => self.resource.animations[index].animation = animation,
                        None => self.resource.animations.push(AnimationData::new(animation)),
                    }
                }
            }
        }
    }

    pub fn update(&mut self) -> GameResult<Transition> {
        if self.is_playing {
            self.frame = self.frame.wrapping_add(1);
            if self.resource.duration() > 0 {
                self.frame %= self.resource.duration();
            } else {
                self.frame = 0;
            }
        }

        let ret = std::mem::replace(&mut self.transition, Transition::None);
        Ok(ret)
    }

    fn handle_transition(&mut self, result: Option<Mode>) {
        if let Some(mode) = &result {
            let animation = match &mode {
                Mode::Edit(name) => self
                    .resource
                    .animations
                    .iter()
                    .map(|item| &item.animation)
                    .find(|item| &item.name == name)
                    .cloned()
                    .unwrap(),
                _ => Animation::new("new"),
            };
            self.transition = Transition::Push(
                Box::new(AnimationEditor::with_animation(animation).into()),
                mode.clone(),
            );
        }
    }

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        assets: &mut Assets,
        imgui: &mut ImGuiWrapper,
    ) -> GameResult<()> {
        let mut editor_result = Ok(());
        imgui
            .frame()
            .run(|ui| {
                ui.window(im_str!("Properties"))
                    .size([300.0, 100.0], Condition::Always)
                    .position([0.0, 20.0], Condition::Always)
                    .resizable(false)
                    .movable(false)
                    .collapsible(false)
                    .build(|| {
                        //let result = self.ui_data.draw_ui(ctx, assets, ui, &mut self.resource);
                        //self.handle_transition(result);
                        self.ui_data.draw_header(ui, &mut self.resource);
                    });
                ui.window(im_str!("Animations"))
                    .size([300.0, 345.0], Condition::Always)
                    .position([0.0, 120.0], Condition::Always)
                    .resizable(false)
                    .movable(false)
                    .collapsible(false)
                    .build(|| {
                        //let result = self.ui_data.draw_ui(ctx, assets, ui, &mut self.resource);
                        //
                        let result = self.ui_data.draw_animation_editor(
                            ctx,
                            assets,
                            ui,
                            &mut self.resource.animations,
                        );
                        self.resource.fix_duration();
                        self.handle_transition(result);
                    });
                ui.window(im_str!("Playback"))
                    .size([300.0, 200.0], Condition::Always)
                    .position([0.0, 465.0], Condition::Always)
                    .resizable(false)
                    .movable(false)
                    .collapsible(false)
                    .build(|| {
                        if self.resource.duration() > 0 {
                            if ui
                                .slider_whole(
                                    im_str!("Frame"),
                                    &mut self.frame,
                                    0,
                                    self.resource.duration() - 1,
                                )
                                .unwrap_or(false)
                            {
                                self.is_playing = false;
                            }
                            if ui.small_button(im_str!("Play")) {
                                self.is_playing = true;
                            };
                            ui.same_line(0.0);
                            if ui.small_button(im_str!("Stop")) {
                                self.is_playing = false;
                            };
                        }
                        ui.text(im_str!("Draw"));
                        ui.separator();
                        ui.checkbox(im_str!("Debug"), &mut self.draw_mode.debug_animation);
                        ui.same_line(0.0);
                        ui.checkbox(im_str!("Axes"), &mut self.draw_mode.show_axes);
                        ui.text(im_str!("Alpha"));
                        ui.separator();
                        ui.slider_float(
                            im_str!("Collision"),
                            &mut self.draw_mode.collision_alpha,
                            0.0,
                            1.0,
                        )
                        .build();
                        ui.slider_float(
                            im_str!("Hurtbox"),
                            &mut self.draw_mode.hurtbox_alpha,
                            0.0,
                            1.0,
                        )
                        .build();
                        ui.slider_float(
                            im_str!("Hitbox"),
                            &mut self.draw_mode.hitbox_alpha,
                            0.0,
                            1.0,
                        )
                        .build();
                    });
                ui.window(im_str!("Particles"))
                    .size([300.0, 280.0], Condition::Always)
                    .position([300.0, 283.0], Condition::Always)
                    .resizable(false)
                    .movable(false)
                    .collapsible(false)
                    .build(|| {
                        self.ui_data
                            .draw_particle_editor(ui, &mut self.resource.particles);
                    });
                ui.window(im_str!("Flags"))
                    .size([300.0, 420.0], Condition::Always)
                    .position([600.0, 283.0], Condition::Always)
                    .resizable(false)
                    .movable(false)
                    .collapsible(false)
                    .build(|| {
                        self.ui_data.draw_flags_editor(ui, &mut self.resource.flags);
                    });
                ui.window(im_str!("Cancels"))
                    .size([300.0, 420.0], Condition::Always)
                    .position([900.0, 283.0], Condition::Always)
                    .resizable(false)
                    .movable(false)
                    .collapsible(false)
                    .build(|| {
                        self.ui_data
                            .draw_cancels_editor(ui, &mut self.resource.cancels);
                    });
                ui.window(im_str!("Hitboxes"))
                    .size([300.0, 700.0], Condition::Always)
                    .position([1200.0, 20.0], Condition::Always)
                    .resizable(false)
                    .movable(false)
                    .collapsible(false)
                    .build(|| {
                        self.ui_data
                            .draw_hitbox_editor(ui, &mut self.resource.hitboxes);
                    });
                ui.window(im_str!("Animation"))
                    .size([300.0, 263.0], Condition::Always)
                    .position([300.0, 20.0], Condition::Always)
                    .resizable(false)
                    .movable(false)
                    .collapsible(false)
                    .build(|| {});
                ui.window(im_str!("Current Flags"))
                    .size([300.0, 263.0], Condition::Always)
                    .position([600.0, 20.0], Condition::Always)
                    .resizable(false)
                    .movable(false)
                    .collapsible(false)
                    .build(|| {
                        if let Some(data) = self.resource.flags.try_time(self.frame) {
                            let move_data = {
                                let mut move_data = MovementData::new();

                                for frame in 0..self.frame {
                                    let flags = self.resource.flags.try_time(frame);
                                    if let Some(flags) = flags {
                                        move_data = flags.apply_movement(move_data);
                                    } else {
                                        move_data.vel += move_data.accel;
                                        move_data.pos += move_data.vel;
                                    }
                                }
                                move_data
                            };
                            FlagsUi::draw_display_ui(ui, data, &move_data);
                        }
                    });
                ui.window(im_str!("Current Cancels"))
                    .size([300.0, 263.0], Condition::Always)
                    .position([900.0, 20.0], Condition::Always)
                    .resizable(false)
                    .movable(false)
                    .collapsible(false)
                    .build(|| {
                        if let Some(data) = self.resource.cancels.try_time(self.frame) {
                            CancelSetUi::draw_display_ui(ui, data);
                        }
                    });
                ui.main_menu_bar(|| {
                    ui.menu(im_str!("State Editor")).build(|| {
                        if ui.menu_item(im_str!("Reset")).build() {
                            self.resource = CharacterState::new();
                            self.ui_data =
                                CharacterStateUi::new(self.ui_data.particle_list.clone());
                        }
                        if ui.menu_item(im_str!("Save to file")).build() {
                            if let Ok(nfd::Response::Okay(path)) =
                                nfd::open_save_dialog(Some("json"), None)
                            {
                                let mut path = PathBuf::from(path);
                                path.set_extension("json");
                                editor_result =
                                    CharacterState::save(ctx, assets, &self.resource, path);
                            }
                        }
                        if ui.menu_item(im_str!("Load from file")).build() {
                            if let Ok(nfd::Response::Okay(path)) =
                                nfd::open_file_dialog(Some("json"), None)
                            {
                                match CharacterState::load_from_json(
                                    ctx,
                                    assets,
                                    PathBuf::from(path),
                                ) {
                                    Ok(state) => {
                                        self.resource = state;
                                        self.ui_data = CharacterStateUi::new(
                                            self.ui_data.particle_list.clone(),
                                        );
                                    }
                                    Err(err) => editor_result = Err(err),
                                }
                            }
                        }
                        ui.separator();

                        if ui.menu_item(im_str!("Save and back")).build() {
                            let ret = std::mem::replace(&mut self.resource, CharacterState::new());
                            self.ui_data =
                                CharacterStateUi::new(self.ui_data.particle_list.clone());
                            self.transition = Transition::Pop(Some(MessageData::State(ret)));
                        }
                        if ui.menu_item(im_str!("Back without saving")).build() {
                            self.transition = Transition::Pop(None);
                        }
                    });
                });
            })
            .render(ctx);
        editor_result?;
        let animation_window_center = Matrix4::new_translation(&Vec3::new(450.0, 270.0, 0.0));
        if self.draw_mode.show_axes {
            graphics::set_transform(ctx, animation_window_center);
            graphics::apply_transformations(ctx)?;
            let x_axis = Mesh::new_line(
                ctx,
                &[[-140.0, 0.0], [140.0, 0.0]],
                1.0,
                Color::new(0.0, 0.0, 0.0, 1.0),
            )?;
            let y_axis = Mesh::new_line(
                ctx,
                &[[0.0, 0.0], [0.0, -150.0]],
                1.0,
                Color::new(0.0, 0.0, 0.0, 1.0),
            )?;
            graphics::draw(ctx, &x_axis, DrawParam::default())?;
            graphics::draw(ctx, &y_axis, DrawParam::default())?;
        }

        let offset = {
            let mut offset = Vec3::zeros();

            if let Some(boxes) = self.resource.hitboxes.try_time(self.frame) {
                let recenter = boxes.collision.collision_graphic_recenter();
                offset.x -= recenter.x;
                offset.y -= recenter.y;
            }
            offset
        };

        let offset = animation_window_center * Matrix4::new_translation(&offset);

        if self.draw_mode.debug_animation {
            self.resource
                .draw_at_time_debug(ctx, assets, self.frame, offset)?;
        } else {
            self.resource
                .draw_at_time(ctx, assets, self.frame, offset)?;
        }

        let offset = {
            let mut offset = Vec3::zeros();
            if let Some(boxes) = self.resource.hitboxes.try_time(self.frame) {
                offset.x -= boxes.collision.center.x.into_graphical();
                offset.y -= boxes.collision.half_size.y.into_graphical()
                    - boxes.collision.center.y.into_graphical();
            }

            offset
        };
        let offset = animation_window_center * Matrix4::new_translation(&offset);
        if let Some(boxes) = self.resource.hitboxes.try_time(self.frame) {
            boxes.collision.draw(
                ctx,
                offset,
                Color::new(1.0, 1.0, 1.0, self.draw_mode.collision_alpha),
            )?;
        }

        let offset = {
            let mut offset = Vec3::zeros();
            if let Some(boxes) = self.resource.hitboxes.try_time(self.frame) {
                offset.y -= boxes.collision.half_size.y.into_graphical();
            }

            offset
        };
        let offset = animation_window_center * Matrix4::new_translation(&offset);
        let draw_cross = |ctx: &mut Context, origin: crate::typedefs::graphics::Vec2| {
            let vertical = Mesh::new_line(
                ctx,
                &[[0.0, -10.0], [0.0, 10.0]],
                1.0,
                Color::new(0.0, 1.0, 0.0, 1.0),
            )?;

            let horizontal = Mesh::new_line(
                ctx,
                &[[-10.0, 0.0], [10.0, 0.0]],
                1.0,
                Color::new(0.0, 1.0, 0.0, 1.0),
            )?;
            graphics::draw(
                ctx,
                &vertical,
                DrawParam::default().dest([origin.x, origin.y]),
            )?;
            graphics::draw(
                ctx,
                &horizontal,
                DrawParam::default().dest([origin.x, origin.y]),
            )
        };
        for particle_spawn in self
            .resource
            .particles
            .iter()
            .filter(|item| item.frame == self.frame)
        {
            let offset = particle_spawn.offset.into_graphical();
            draw_cross(ctx, offset)?;
        }
        if let Some(boxes) = self.resource.hitboxes.try_time(self.frame) {
            for hurtbox in boxes.hurtbox.iter() {
                hurtbox.draw(
                    ctx,
                    offset,
                    Color::new(0.0, 1.0, 0.0, self.draw_mode.hurtbox_alpha),
                )?;
            }
            if let Some(attack_data) = &boxes.hitbox {
                for hitbox in attack_data.boxes.iter() {
                    hitbox.draw(
                        ctx,
                        offset,
                        Color::new(1.0, 0.0, 0.0, self.draw_mode.hitbox_alpha),
                    )?;
                }
            }
        }

        Ok(())
    }
}

impl Into<EditorState> for StateEditor {
    fn into(self) -> EditorState {
        EditorState::StateEditor(self)
    }
}
