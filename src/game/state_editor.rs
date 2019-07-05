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

use crate::character_state::{CharacterState, CharacterStateUi};
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
        imgui
            .frame()
            .run(|ui| {
                ui.main_menu_bar(|| {
                    ui.window(im_str!("Editor"))
                        .size([300.0, 526.0], Condition::Always)
                        .position([0.0, 20.0], Condition::Always)
                        .resizable(false)
                        .movable(false)
                        .collapsible(false)
                        .build(|| {
                            editor_result =
                                self.ui_data.draw_ui(ctx, assets, ui, &mut self.resource);
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
        self.resource.draw_at_time(
            ctx,
            assets,
            self.frame,
            Matrix4::new_translation(&Vec3::new(600.0, 200.0, 0.0)),
        )?;


        Ok(())
    }
}

impl Into<GameState> for StateEditor {
    fn into(self) -> GameState {
        GameState::StateEditor(self)
    }
}