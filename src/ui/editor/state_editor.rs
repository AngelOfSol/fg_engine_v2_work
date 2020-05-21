use super::character_editor::{ItemResource, StateAnimationResource, StateResource};
use crate::app_state::{AppContext, AppState, Transition};
use crate::assets::Assets;
use crate::character::state::components::{MovementData, ParticlePath};
use crate::character::state::{EditorCharacterState, State};
use crate::character::PlayerCharacter;
use crate::imgui_extra::UiExtensions;
use crate::timeline::AtTime;
use crate::typedefs::collision::IntoGraphical;
use crate::typedefs::graphics::{Matrix4, Vec3};
use crate::ui::character::state::{CancelSetUi, FlagsUi, StateUi};
use crate::ui::editor::AnimationEditor;
use ggez::graphics;
use ggez::graphics::{Color, DrawParam, Mesh};
use ggez::{Context, GameResult};
use imgui::*;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

pub struct StateEditor {
    character_data: Rc<RefCell<PlayerCharacter>>,
    assets: Rc<RefCell<Assets>>,
    resource: Rc<RefCell<EditorCharacterState>>,
    path: StateResource,
    frame: usize,
    is_playing: bool,
    transition: Transition,
    ui_data: StateUi,
    draw_mode: DrawMode,
}
struct DrawMode {
    collision_alpha: f32,
    hurtbox_alpha: f32,
    hitbox_alpha: f32,
    debug_animation: bool,
    show_axes: bool,
    show_all_bullets: bool,
}

impl AppState for StateEditor {
    fn update(&mut self, ctx: &mut Context, _: &mut AppContext) -> GameResult<Transition> {
        while ggez::timer::check_update_time(ctx, 60) {
            if self.is_playing {
                self.frame = self.frame.wrapping_add(1);
                let resource = self.resource.borrow();
                if resource.duration() > 0 {
                    self.frame %= resource.duration();
                } else {
                    self.frame = 0;
                }
            }
        }

        Ok(std::mem::replace(&mut self.transition, Transition::None))
    }

    fn on_enter(&mut self, _: &mut Context, _: &mut AppContext) -> GameResult<()> {
        Ok(())
    }
    fn draw(
        &mut self,
        ctx: &mut Context,
        AppContext { ref mut imgui, .. }: &mut AppContext,
    ) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        let mut editor_result = Ok(());
        let (state_list, particles_list, bullet_list, attack_ids, sounds_list) = {
            let character_data = self.character_data.borrow();

            let state_list = {
                let mut state_list: Vec<_> = character_data.states.rest.keys().cloned().collect();
                state_list.sort();
                state_list
            };

            let particles_list = {
                let mut particles_list: Vec<_> =
                    character_data.particles.particles.keys().cloned().collect();
                particles_list.sort();
                particles_list
            };

            let bullet_list = character_data
                .bullets
                .bullets
                .iter()
                .map(|(key, value)| (key.clone(), value.properties.clone()))
                .collect();

            let attack_ids: Vec<_> = character_data.attacks.attacks.keys().cloned().collect();

            let sounds_list: Vec<_> = character_data
                .properties
                .character
                .sound_name_iterator()
                .collect();

            (
                state_list,
                particles_list,
                bullet_list,
                attack_ids,
                sounds_list,
            )
        };
        imgui
            .frame()
            .run(|ui| {
                imgui::Window::new(im_str!("Properties"))
                    .size([300.0, 140.0], Condition::Once)
                    .position([0.0, 20.0], Condition::Once)
                    .build(ui, || {
                        self.ui_data
                            .draw_header(ui, &state_list, &mut self.resource.borrow_mut());
                    });
                imgui::Window::new(im_str!("Animations"))
                    .size([300.0, 345.0], Condition::Once)
                    .position([0.0, 160.0], Condition::Once)
                    .build(ui, || {
                        let assets = &mut self.assets.borrow_mut();
                        let result = self.ui_data.draw_animation_editor(
                            ctx,
                            assets,
                            ui,
                            &mut self.resource.borrow_mut().animations,
                        );
                        self.resource.borrow_mut().fix_duration();
                        if let Some(name) = result {
                            self.transition = Transition::Push(Box::new(
                                AnimationEditor::new(
                                    self.assets.clone(),
                                    Box::new(StateAnimationResource {
                                        data: self.resource.clone(),
                                        name,
                                    }),
                                )
                                .unwrap(),
                            ))
                        }
                    });
                imgui::Window::new(im_str!("Playback"))
                    .size([300.0, 215.0], Condition::Once)
                    .position([0.0, 505.0], Condition::Once)
                    .build(ui, || {
                        let resource = self.resource.borrow();
                        if resource.duration() > 0 {
                            if ui
                                .slider_whole(
                                    im_str!("Frame"),
                                    &mut self.frame,
                                    0,
                                    resource.duration() - 1,
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
                        self.ui_data.draw_particle_editor(
                            ui,
                            &particles_list,
                            &mut self.resource.borrow_mut().particles,
                        );
                    });
                imgui::Window::new(im_str!("Bullets##State"))
                    .size([300.0, 400.0], Condition::Once)
                    .position([300.0, 303.0], Condition::Once)
                    .collapsed(true, Condition::Once)
                    .build(ui, || {
                        self.ui_data.draw_bullet_editor(
                            ui,
                            &bullet_list,
                            &mut self.resource.borrow_mut().bullets,
                        );
                    });
                imgui::Window::new(im_str!("Sounds##State"))
                    .size([300.0, 400.0], Condition::Once)
                    .position([300.0, 323.0], Condition::Once)
                    .collapsed(true, Condition::Once)
                    .build(ui, || {
                        self.ui_data.draw_sounds_editor(
                            ui,
                            &sounds_list,
                            &mut self.resource.borrow_mut().sounds,
                        );
                    });
                imgui::Window::new(im_str!("Flags"))
                    .size([300.0, 420.0], Condition::Once)
                    .position([600.0, 283.0], Condition::Once)
                    .build(ui, || {
                        self.ui_data
                            .draw_flags_editor(ui, &mut self.resource.borrow_mut().flags);
                    });
                imgui::Window::new(im_str!("Cancels"))
                    .size([300.0, 420.0], Condition::Once)
                    .position([900.0, 283.0], Condition::Once)
                    .build(ui, || {
                        self.ui_data.draw_cancels_editor(
                            ui,
                            &state_list,
                            &mut self.resource.borrow_mut().cancels,
                        );
                    });
                imgui::Window::new(im_str!("Hitboxes"))
                    .size([300.0, 700.0], Condition::Once)
                    .position([1200.0, 20.0], Condition::Once)
                    .build(ui, || {
                        self.ui_data.draw_hitbox_editor(
                            ui,
                            &mut self.resource.borrow_mut().hitboxes,
                            &attack_ids,
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
                        let resource = self.resource.borrow();
                        if let Some(data) = resource.flags.try_time(self.frame) {
                            let move_data = {
                                let mut move_data = MovementData::new();

                                for frame in 0..self.frame {
                                    let flags = resource.flags.try_time(frame);
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
                        if let Some(data) = self.resource.borrow().cancels.try_time(self.frame) {
                            CancelSetUi::draw_display_ui(ui, data);
                        }
                    });
                ui.main_menu_bar(|| {
                    ui.menu(im_str!("State Editor"), true, || {
                        if imgui::MenuItem::new(im_str!("Reset")).build(ui) {
                            *self.resource.borrow_mut() = State::new();
                            self.ui_data = StateUi::new();
                        }
                        if imgui::MenuItem::new(im_str!("Save to file")).build(ui) {
                            if let Ok(nfd::Response::Okay(path)) =
                                nfd::open_save_dialog(Some("json"), None)
                            {
                                let mut path = PathBuf::from(path);
                                path.set_extension("json");
                                let assets = &mut self.assets.borrow_mut();
                                editor_result =
                                    State::save(ctx, assets, &self.resource.borrow(), path);
                            }
                        }
                        if imgui::MenuItem::new(im_str!("Load from file")).build(ui) {
                            if let Ok(nfd::Response::Okay(path)) =
                                nfd::open_file_dialog(Some("json"), None)
                            {
                                let assets = &mut self.assets.borrow_mut();
                                match State::load_from_json(ctx, assets, PathBuf::from(path)) {
                                    Ok(state) => {
                                        *self.resource.borrow_mut() = state;
                                        self.ui_data = StateUi::new();
                                    }
                                    Err(err) => editor_result = Err(err),
                                }
                            }
                        }
                        ui.separator();

                        if imgui::MenuItem::new(im_str!("Save and back")).build(ui) {
                            let mut overwrite_target = self.path.get_from_mut().unwrap();
                            *overwrite_target =
                                std::mem::replace(&mut self.resource.borrow_mut(), State::new());
                            self.transition = Transition::Pop;
                        }
                        if imgui::MenuItem::new(im_str!("Back without saving")).build(ui) {
                            self.transition = Transition::Pop;
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

        let resource = self.resource.borrow();

        let offset = {
            let mut offset = Vec3::zeros();

            if let Some(boxes) = resource.hitboxes.try_time(self.frame) {
                let recenter = boxes.collision.collision_graphic_recenter();
                offset.x -= recenter.x;
                offset.y -= recenter.y;
            }
            offset
        };

        let offset = animation_window_center * Matrix4::new_translation(&offset);
        let assets = &self.assets.borrow();

        {
            let _lock = graphics::use_shader(ctx, &assets.shader);

            if self.draw_mode.debug_animation {
                resource.draw_at_time_debug(ctx, assets, self.frame, offset)?;
            } else {
                resource.draw_at_time(ctx, assets, self.frame, offset)?;
            }

            for particle_spawn in resource.particles.iter() {
                let current_frame = self.frame.checked_sub(particle_spawn.frame);
                if let Some(current_frame) = current_frame {
                    let offset = offset
                        * Matrix4::new_translation(&Vec3::new(
                            particle_spawn.offset.into_graphical().x,
                            particle_spawn.offset.into_graphical().y,
                            0.0,
                        ));
                    if let ParticlePath::Local(particle) = &particle_spawn.particle_id {
                        dbg!(particle);

                        self.character_data.borrow().particles.particles[particle].draw_at_time(
                            ctx,
                            assets,
                            current_frame,
                            offset,
                        )?;
                    }
                }
            }
        }

        let offset = {
            let mut offset = Vec3::zeros();
            if let Some(boxes) = resource.hitboxes.try_time(self.frame) {
                offset.x -= boxes.collision.center.x.into_graphical();
                offset.y -= boxes.collision.half_size.y.into_graphical()
                    - boxes.collision.center.y.into_graphical();
            }

            offset
        };
        let offset = animation_window_center * Matrix4::new_translation(&offset);
        if let Some(boxes) = resource.hitboxes.try_time(self.frame) {
            boxes.collision.draw(
                ctx,
                offset,
                Color::new(1.0, 1.0, 1.0, self.draw_mode.collision_alpha),
            )?;
        }

        let offset = {
            let mut offset = Vec3::zeros();
            if let Some(boxes) = resource.hitboxes.try_time(self.frame) {
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
        for particle_spawn in resource
            .particles
            .iter()
            .filter(|item| item.frame == self.frame)
        {
            let offset = particle_spawn.offset.into_graphical();
            draw_cross(ctx, offset)?;

            // TODO draw particles too
        }
        for bullet_spawn in resource
            .bullets
            .iter()
            .filter(|item| item.frame == self.frame || self.draw_mode.show_all_bullets)
        {
            let offset = bullet_spawn.offset.into_graphical();
            draw_cross(ctx, offset)?;
        }
        if let Some(boxes) = resource.hitboxes.try_time(self.frame) {
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

        graphics::present(ctx)
    }
}

impl StateEditor {
    pub fn new(
        character_data: Rc<RefCell<PlayerCharacter>>,
        assets: Rc<RefCell<Assets>>,
        path: StateResource,
    ) -> Option<Self> {
        let resource = Rc::new(RefCell::new(path.get_from()?.clone()));
        Some(Self {
            character_data,
            assets,
            path,
            resource,
            transition: Transition::None,
            frame: 0,
            is_playing: true,
            ui_data: StateUi::new(),
            draw_mode: DrawMode {
                collision_alpha: 0.15,
                hurtbox_alpha: 0.15,
                hitbox_alpha: 0.15,
                debug_animation: true,
                show_axes: true,
                show_all_bullets: false,
            },
        })
    }
}
