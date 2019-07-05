use crate::game::{GameState, Transition};

use ggez::graphics;
use ggez::graphics::Color;
use ggez::{Context, GameResult};

use crate::animation::{Animation, AnimationUi};

use crate::assets::Assets;
use crate::game;
use crate::timeline::AtTime;

use crate::imgui_wrapper::ImGuiWrapper;

use crate::typedefs::graphics::{Matrix4, Vec3};

use crate::typedefs::collision::{IntoGraphical, Vec2};

use crate::character_state::{CharacterState, CharacterStateUi, FlagsUi, MovementData, CancelSetUi};
use crate::imgui_extra::UiExtensions;
use imgui::*;

pub struct StateEditor {
    resource: CharacterState,
    frame: usize,
    is_playing: bool,
    done: bool,
    ui_data: CharacterStateUi,
}

impl StateEditor {
    pub fn new() -> Self {
        Self {
            resource: CharacterState::new(),
            done: false,
            frame: 0,
            is_playing: true,
            ui_data: CharacterStateUi::new(),
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
                if self.resource.duration() > 0 {
                    ui.window(im_str!("Playback"))
                        .size([300.0, 80.0], Condition::Always)
                        .position([0.0, 546.0], Condition::Always)
                        .resizable(false)
                        .build(|| {
                            if ui
                                .slider_whole(
                                    im_str!("Frame "),
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
                        });
                }
                ui.main_menu_bar(|| {
                    ui.menu(im_str!("State Editor")).build(|| {
                        if ui.menu_item(im_str!("New")).build() {
                            self.resource = CharacterState::new();
                            self.ui_data = CharacterStateUi::new();
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

        let graphics_offset = move_data.pos.into_graphical();
        let offset =
            Vec3::new(600.0, 200.0, 0.0) + Vec3::new(graphics_offset.x, graphics_offset.y, 0.0);

        self.resource
            .draw_at_time(ctx, assets, self.frame, Matrix4::new_translation(&offset))?;

        Ok(())
    }
}

impl Into<GameState> for StateEditor {
    fn into(self) -> GameState {
        GameState::StateEditor(self)
    }
}
