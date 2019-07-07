use crate::game::{GameState, Transition};

use ggez::graphics;
use ggez::graphics::{Color, DrawParam, Mesh};
use ggez::{Context, GameResult};

use crate::assets::Assets;
use crate::timeline::AtTime;

use crate::imgui_wrapper::ImGuiWrapper;

use crate::typedefs::graphics::{Matrix4, Vec3};

use crate::typedefs::collision::IntoGraphical;

use crate::character_state::{
    CancelSetUi, CharacterState, CharacterStateUi, FlagsUi, MovementData,
};
use crate::imgui_extra::UiExtensions;
use imgui::*;

use std::path::PathBuf;

pub struct StateEditor {
    resource: CharacterState,
    frame: usize,
    is_playing: bool,
    done: bool,
    ui_data: CharacterStateUi,
    draw_mode: DrawMode,
}
struct DrawMode {
    collision_alpha: f32,
    hurtbox_alpha: f32,
    hitbox_alpha: f32,
    debug_animation: bool,
    show_travel: bool,
    show_axes: bool,
}

impl StateEditor {
    pub fn new() -> Self {
        Self {
            resource: CharacterState::new(),
            done: false,
            frame: 0,
            is_playing: true,
            ui_data: CharacterStateUi::new(),
            draw_mode: DrawMode {
                collision_alpha: 0.15,
                hurtbox_alpha: 0.15,
                hitbox_alpha: 0.15,
                debug_animation: true,
                show_travel: true,
                show_axes: true,
            },
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

        if self.done {
            Ok(Transition::Pop)
        } else {
            Ok(Transition::None)
        }
    }

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        assets: &mut Assets,
        imgui: &mut ImGuiWrapper,
    ) -> GameResult<()> {
        let mut editor_result = Ok(());
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

        imgui
            .frame()
            .run(|ui| {
                ui.window(im_str!("Editor"))
                    .size([300.0, 526.0], Condition::Always)
                    .position([0.0, 20.0], Condition::Always)
                    .resizable(false)
                    .movable(false)
                    .collapsible(false)
                    .build(|| {
                        editor_result = self.ui_data.draw_ui(ctx, assets, ui, &mut self.resource);
                    });
                ui.window(im_str!("Animation"))
                    .size([600.0, 263.0], Condition::Always)
                    .position([300.0, 20.0], Condition::Always)
                    .resizable(false)
                    .movable(false)
                    .collapsible(false)
                    .build(|| {});
                ui.window(im_str!("Current Flags"))
                    .size([200.0, 263.0], Condition::Always)
                    .position([300.0, 263.0 + 20.0], Condition::Always)
                    .resizable(false)
                    .movable(false)
                    .collapsible(false)
                    .build(|| {
                        if let Some(data) = self.resource.flags.try_time(self.frame) {
                            FlagsUi::draw_display_ui(ui, data, &move_data);
                        }
                    });
                ui.window(im_str!("Current Cancels"))
                    .size([200.0, 263.0], Condition::Always)
                    .position([500.0, 263.0 + 20.0], Condition::Always)
                    .resizable(false)
                    .movable(false)
                    .collapsible(false)
                    .build(|| {
                        if let Some(data) = self.resource.cancels.try_time(self.frame) {
                            CancelSetUi::draw_display_ui(ui, data);
                        }
                    });
                ui.window(im_str!("Playback"))
                    .size([300.0, 160.0], Condition::Always)
                    .position([0.0, 546.0], Condition::Always)
                    .resizable(false)
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
                        ui.checkbox(im_str!("Movement"), &mut self.draw_mode.show_travel);
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
                ui.main_menu_bar(|| {
                    ui.menu(im_str!("State Editor")).build(|| {
                        if ui.menu_item(im_str!("New")).build() {
                            self.resource = CharacterState::new();
                            self.ui_data = CharacterStateUi::new();
                        }
                        if ui.menu_item(im_str!("Save")).build() {
                            if let Ok(nfd::Response::Okay(path)) = nfd::open_pick_folder(None) {
                                editor_result = CharacterState::save(
                                    ctx,
                                    assets,
                                    &self.resource,
                                    PathBuf::from(path),
                                );
                            }
                        }
                        if ui.menu_item(im_str!("Open")).build() {
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
                                        self.ui_data = CharacterStateUi::new();
                                    }
                                    Err(err) => editor_result = Err(err),
                                }
                            }
                        }
                        ui.separator();

                        if ui.menu_item(im_str!("Back")).build() {
                            self.done = true;
                        }
                    });
                });
            })
            .render(ctx);
        editor_result?;

        if self.draw_mode.show_axes {
            graphics::set_transform(ctx, Matrix4::new_translation(&Vec3::new(600.0, 240.0, 0.0)));
            graphics::apply_transformations(ctx)?;
            let x_axis = Mesh::new_line(
                ctx,
                &[[-280.0, 0.0], [280.0, 0.0]],
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

        let movement_offset = move_data.pos.into_graphical();
        let offset = {
            let mut offset = Vec3::new(600.0, 240.0, 0.0);
            if self.draw_mode.show_travel {
                offset += Vec3::new(movement_offset.x, movement_offset.y, 0.0);
            }

            if let Some(boxes) = self.resource.hitboxes.try_time(self.frame) {
                let recenter = boxes.collision.collision_graphic_recenter();
                offset.x -= recenter.x;
                offset.y -= recenter.y;
            }
            offset
        };

        let offset = Matrix4::new_translation(&offset);

        if self.draw_mode.debug_animation {
            self.resource
                .draw_at_time_debug(ctx, assets, self.frame, offset)?;
        } else {
            self.resource
                .draw_at_time(ctx, assets, self.frame, offset)?;
        }
        if let Some(boxes) = self.resource.hitboxes.try_time(self.frame) {
            boxes.collision.draw(
                ctx,
                offset,
                Color::new(1.0, 1.0, 1.0, self.draw_mode.collision_alpha),
            )?;

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

impl Into<GameState> for StateEditor {
    fn into(self) -> GameState {
        GameState::StateEditor(self)
    }
}
