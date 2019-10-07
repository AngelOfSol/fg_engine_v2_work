use crate::imgui_wrapper::ImGuiWrapper;

use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::graphics;
use ggez::input::mouse::MouseButton;
use ggez::timer;
use ggez::{Context, GameResult};

use crate::runner::{AppState, RunnerState};

use crate::game_match::Match;

use crate::input::control_scheme::{is_valid_input_button, render_button_list, PadControlScheme};

use gilrs::{Button, Event, EventType, Gilrs};

use imgui::*;

pub struct ButtonCheck {
    imgui: ImGuiWrapper,
    control_scheme: Option<PadControlScheme>,
    pads_context: Gilrs,
    selected_cell: usize,
}

impl ButtonCheck {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(ButtonCheck {
            imgui: ImGuiWrapper::new(ctx),
            pads_context: Gilrs::new()?,
            control_scheme: None,
            selected_cell: 0,
        })
    }
}

impl AppState for ButtonCheck {
    fn next_appstate(&mut self, ctx: &mut Context) -> Option<RunnerState> {
        let control_scheme = std::mem::replace(&mut self.control_scheme, None);
        control_scheme.map(|item| RunnerState::Match(Match::new(ctx, item).unwrap()))
    }
}
const GREEN: [f32; 4] = [0.2, 1.0, 0.2, 1.0];

impl EventHandler for ButtonCheck {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while timer::check_update_time(ctx, 60) {
            while let Some(event) = self.pads_context.next_event() {
                // let id = event.id;
                let Event { id, event, .. } = event;
                if let EventType::ButtonPressed(button, _) = event {
                    if button == Button::Start {
                        self.control_scheme = Some(PadControlScheme::new(id))
                    } else if self.control_scheme.is_some() {
                        let scheme = self.control_scheme.as_mut().unwrap();
                        if is_valid_input_button(button) {
                            if self.selected_cell == 4 {
                                ggez::event::quit(ctx);
                            } else {
                                let buttons = &mut scheme.buttons[self.selected_cell];
                                if buttons.contains(&button) {
                                    buttons.remove(&button);
                                } else {
                                    buttons.insert(button);
                                }
                            }
                        } else {
                            match button {
                                Button::DPadUp => {
                                    if self.selected_cell > 0 {
                                        self.selected_cell -= 1;
                                    }
                                }
                                Button::DPadDown => {
                                    if self.selected_cell < 4 {
                                        self.selected_cell += 1
                                    }
                                }
                                _ => (),
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        let (control_scheme, imgui, selected) = (
            &mut self.control_scheme,
            &mut self.imgui,
            &mut self.selected_cell,
        );
        imgui
            .frame()
            .run(|ui| {
                ui.window(im_str!("Button Check"))
                    .build(|| match control_scheme {
                        Some(ref mut control_scheme) => {
                            ui.text(format!("Gamepad: {}", control_scheme.gamepad));
                            ui.columns(2, im_str!("Columns"), true);
                            for index in 0..4 {
                                let _token = if index == *selected {
                                    Some(ui.push_style_color(StyleColor::Text, GREEN))
                                } else {
                                    None
                                };

                                ui.text(match index {
                                    0 => "A",
                                    1 => "B",
                                    2 => "C",
                                    3 => "D",
                                    _ => unreachable!(),
                                });
                                ui.next_column();
                                ui.text(render_button_list(&control_scheme.buttons[index]));
                                ui.next_column();
                            }
                            ui.columns(1, im_str!("Columns##End"), false);

                            let _token = if 4 == *selected {
                                Some(ui.push_style_color(StyleColor::Text, GREEN))
                            } else {
                                None
                            };
                            ui.text("Finish")
                        }
                        None => ui.text("Press Start to select a controller."),
                    });
            })
            .render(ctx);

        graphics::present(ctx)?;

        Ok(())
    }
    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _xrel: f32, _yrel: f32) {
        self.imgui.update_mouse_pos(x, y);
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, y: f32) {
        self.imgui.update_mouse_scroll(y);
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.imgui.update_mouse_down((
            button == MouseButton::Left,
            button == MouseButton::Right,
            button == MouseButton::Middle,
        ));
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: f32, _y: f32) {
        self.imgui.update_mouse_down((
            match button {
                MouseButton::Left => false,
                _ => true,
            },
            match button {
                MouseButton::Right => false,
                _ => true,
            },
            match button {
                MouseButton::Middle => false,
                _ => true,
            },
        ));
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        keymod: KeyMods,
        _repeat: bool,
    ) {
        self.imgui.handle_keyboard_input(keycode, keymod, true);
    }
    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, keymod: KeyMods) {
        self.imgui.handle_keyboard_input(keycode, keymod, false);
    }
    fn text_input_event(&mut self, _ctx: &mut Context, character: char) {
        self.imgui.handle_text_input(character);
    }

    fn resize_event(&mut self, ctx: &mut Context, _width: f32, _height: f32) {
        self.imgui.resize(ctx);
    }
}
