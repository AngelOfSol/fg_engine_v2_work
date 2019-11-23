use crate::assets::Assets;
use crate::character::state::components::{AnimationData, MovementData};
use crate::character::state::{EditorCharacterState, State};
use crate::graphics::Animation;
use crate::imgui_extra::UiExtensions;
use crate::imgui_wrapper::ImGuiWrapper;
use crate::timeline::AtTime;
use crate::typedefs::collision::IntoGraphical;
use crate::typedefs::graphics::{Matrix4, Vec3};
use crate::ui::character::state::{CancelSetUi, FlagsUi, StateUi};
use crate::ui::editor::{AnimationEditor, EditorState, MessageData, Mode, Transition};
use ggez::graphics;
use ggez::graphics::{Color, DrawParam, Mesh};
use ggez::{Context, GameResult};
use imgui::*;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

pub struct StateEditor {
    resource: EditorCharacterState,
    frame: usize,
    is_playing: bool,
    transition: Transition,
    ui_data: StateUi,
    draw_mode: DrawMode,
    attack_ids: Vec<String>,
}
struct DrawMode {
    collision_alpha: f32,
    hurtbox_alpha: f32,
    hitbox_alpha: f32,
    debug_animation: bool,
    show_axes: bool,
    show_all_bullets: bool,
}
impl StateEditor {
    pub fn with_state(
        state: EditorCharacterState,
        mut particle_list: Vec<String>,
        mut state_list: Vec<String>,
        bullet_list: HashMap<String, HashSet<String>>,
        mut attack_ids: Vec<String>,
    ) -> Self {
        particle_list.sort();
        state_list.sort();
        attack_ids.sort();
        Self {
            resource: state,
            transition: Transition::None,
            frame: 0,
            is_playing: true,
            ui_data: StateUi::new(particle_list, state_list, bullet_list),
            attack_ids,
            draw_mode: DrawMode {
                collision_alpha: 0.15,
                hurtbox_alpha: 0.15,
                hitbox_alpha: 0.15,
                debug_animation: true,
                show_axes: true,
                show_all_bullets: false,
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
                imgui::Window::new(im_str!("Properties"))
                    .size([300.0, 140.0], Condition::Once)
                    .position([0.0, 20.0], Condition::Once)
                    .build(ui, || {
                        self.ui_data.draw_header(ui, &mut self.resource);
                    });
                imgui::Window::new(im_str!("Animations"))
                    .size([300.0, 345.0], Condition::Once)
                    .position([0.0, 160.0], Condition::Once)
                    .build(ui, || {
                        let result = self.ui_data.draw_animation_editor(
                            ctx,
                            assets,
                            ui,
                            &mut self.resource.animations,
                        );
                        self.resource.fix_duration();
                        self.handle_transition(result);
                    });
                imgui::Window::new(im_str!("Playback"))
                    .size([300.0, 215.0], Condition::Once)
                    .position([0.0, 505.0], Condition::Once)
                    .build(ui, || {
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
                        ui.same_line(0.0);
                        ui.checkbox(im_str!("All Bullets"), &mut self.draw_mode.show_all_bullets);
                        ui.text(im_str!("Alpha"));

                        ui.separator();

                        imgui::Slider::new(im_str!("Collision"), 0.0..=1.0)
                            .build(ui, &mut self.draw_mode.collision_alpha);
                        imgui::Slider::new(im_str!("Hurtbox"), 0.0..=1.0)
                            .build(ui, &mut self.draw_mode.hurtbox_alpha);
                        imgui::Slider::new(im_str!("Hitbox"), 0.0..=1.0)
                            .build(ui, &mut self.draw_mode.hitbox_alpha);
                    });
                imgui::Window::new(im_str!("Particles##State"))
                    .size([300.0, 420.0], Condition::Once)
                    .position([300.0, 283.0], Condition::Once)
                    .collapsed(true, Condition::Once)
                    .build(ui, || {
                        self.ui_data
                            .draw_particle_editor(ui, &mut self.resource.particles);
                    });
                imgui::Window::new(im_str!("Bullets##State"))
                    .size([300.0, 400.0], Condition::Once)
                    .position([300.0, 303.0], Condition::Once)
                    .collapsed(true, Condition::Once)
                    .build(ui, || {
                        self.ui_data
                            .draw_bullet_editor(ui, &mut self.resource.bullets);
                    });
                imgui::Window::new(im_str!("Flags"))
                    .size([300.0, 420.0], Condition::Once)
                    .position([600.0, 283.0], Condition::Once)
                    .build(ui, || {
                        self.ui_data.draw_flags_editor(ui, &mut self.resource.flags);
                    });
                imgui::Window::new(im_str!("Cancels"))
                    .size([300.0, 420.0], Condition::Once)
                    .position([900.0, 283.0], Condition::Once)
                    .build(ui, || {
                        self.ui_data
                            .draw_cancels_editor(ui, &mut self.resource.cancels);
                    });
                imgui::Window::new(im_str!("Hitboxes"))
                    .size([300.0, 700.0], Condition::Once)
                    .position([1200.0, 20.0], Condition::Once)
                    .build(ui, || {
                        self.ui_data.draw_hitbox_editor(
                            ui,
                            &mut self.resource.hitboxes,
                            &self.attack_ids,
                        );
                    });
                imgui::Window::new(im_str!("Animation"))
                    .size([300.0, 263.0], Condition::Always)
                    .position([300.0, 20.0], Condition::Always)
                    .resizable(false)
                    .movable(false)
                    .collapsible(false)
                    .build(ui, || {});
                imgui::Window::new(im_str!("Current Flags"))
                    .size([300.0, 263.0], Condition::Once)
                    .position([600.0, 20.0], Condition::Once)
                    .build(ui, || {
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
                imgui::Window::new(im_str!("Current Cancels"))
                    .size([300.0, 263.0], Condition::Once)
                    .position([900.0, 20.0], Condition::Once)
                    .build(ui, || {
                        if let Some(data) = self.resource.cancels.try_time(self.frame) {
                            CancelSetUi::draw_display_ui(ui, data);
                        }
                    });
                ui.main_menu_bar(|| {
                    ui.menu(im_str!("State Editor"), true, || {
                        if imgui::MenuItem::new(im_str!("Reset")).build(ui) {
                            self.resource = State::new();
                            self.ui_data = StateUi::new(
                                self.ui_data.particle_list.clone(),
                                self.ui_data.state_list.clone(),
                                self.ui_data.bullet_list.clone(),
                            );
                        }
                        if imgui::MenuItem::new(im_str!("Save to file")).build(ui) {
                            if let Ok(nfd::Response::Okay(path)) =
                                nfd::open_save_dialog(Some("json"), None)
                            {
                                let mut path = PathBuf::from(path);
                                path.set_extension("json");
                                editor_result = State::save(ctx, assets, &self.resource, path);
                            }
                        }
                        if imgui::MenuItem::new(im_str!("Load from file")).build(ui) {
                            if let Ok(nfd::Response::Okay(path)) =
                                nfd::open_file_dialog(Some("json"), None)
                            {
                                match State::load_from_json(ctx, assets, PathBuf::from(path)) {
                                    Ok(state) => {
                                        self.resource = state;
                                        self.ui_data = StateUi::new(
                                            self.ui_data.particle_list.clone(),
                                            self.ui_data.state_list.clone(),
                                            self.ui_data.bullet_list.clone(),
                                        );
                                    }
                                    Err(err) => editor_result = Err(err),
                                }
                            }
                        }
                        ui.separator();

                        if imgui::MenuItem::new(im_str!("Save and back")).build(ui) {
                            let ret = std::mem::replace(&mut self.resource, State::new());
                            self.ui_data = StateUi::new(
                                self.ui_data.particle_list.clone(),
                                self.ui_data.state_list.clone(),
                                self.ui_data.bullet_list.clone(),
                            );
                            self.transition = Transition::Pop(Some(MessageData::State(ret)));
                        }
                        if imgui::MenuItem::new(im_str!("Back without saving")).build(ui) {
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
        for bullet_spawn in self
            .resource
            .bullets
            .iter()
            .filter(|item| item.frame == self.frame || self.draw_mode.show_all_bullets)
        {
            let offset = bullet_spawn.offset.into_graphical();
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
