use crate::imgui_wrapper::ImGuiWrapper;

use crate::game_match::Match;
use crate::input::control_scheme::{is_valid_input_button, render_button_list, PadControlScheme};
use crate::runner::{AppState, RunnerState};
use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::graphics;
use ggez::input::mouse::MouseButton;
use ggez::timer;
use ggez::{Context, GameResult};
use gilrs::{Button, Event, EventType, GamepadId, Gilrs};
use imgui::*;

pub struct ButtonCheck {
    imgui: ImGuiWrapper,
    p1_control_scheme: CreateScheme,
    p2_control_scheme: CreateScheme,
    pads_context: Gilrs,
}

struct CreateScheme {
    scheme: Option<PadControlScheme>,
    selected_cell: usize,
}

impl CreateScheme {
    fn new() -> Self {
        Self {
            scheme: None,
            selected_cell: 0,
        }
    }

    pub fn assign_controller(&mut self, id: GamepadId) {
        if let Some(ref mut scheme) = self.scheme {
            scheme.gamepad = id;
        } else {
            self.scheme = Some(PadControlScheme::new(id));
        }
    }

    pub fn update_button(&mut self, id: GamepadId, button: Button) {
        if self.scheme.is_none() || self.scheme.as_ref().unwrap().gamepad != id {
            return;
        }
        if let Some(ref mut scheme) = self.scheme {
            if is_valid_input_button(button) {
                if self.selected_cell == 4 {
                    //TODO add quit conditions; ggez::event::quit(ctx);
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
                        } else {
                            self.selected_cell = 4;
                        }
                    }
                    Button::DPadDown => {
                        if self.selected_cell < 4 {
                            self.selected_cell += 1
                        } else {
                            self.selected_cell = 0;
                        }
                    }
                    _ => (),
                }
            }
        }
    }

    pub fn draw_ui(&mut self, ui: &Ui<'_>, pads_context: &Gilrs) {
        match self.scheme {
            Some(ref mut control_scheme) => {
                let gamepad = pads_context.gamepad(control_scheme.gamepad);
                let id = ui.push_id(&format!("{}", gamepad.id()));

                ui.text(format!("Gamepad: {}", gamepad.name()));

                ui.columns(2, im_str!("Columns"), true);
                for index in 0..4 {
                    let token = if index == self.selected_cell {
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

                    if let Some(token) = token {
                        token.pop(ui);
                    }
                }
                ui.columns(1, im_str!("Columns##End"), false);

                let token = if 4 == self.selected_cell {
                    Some(ui.push_style_color(StyleColor::Text, GREEN))
                } else {
                    None
                };
                ui.text("Finish");
                if let Some(token) = token {
                    token.pop(ui);
                }
                id.pop(ui);
            }
            None => ui.text("Press Left or Right to select a controller."),
        }
    }
}

impl ButtonCheck {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(ButtonCheck {
            imgui: ImGuiWrapper::new(ctx),
            pads_context: Gilrs::new()?,
            p1_control_scheme: CreateScheme::new(),
            p2_control_scheme: CreateScheme::new(),
        })
    }
}

impl AppState for ButtonCheck {
    fn next_appstate(&mut self, ctx: &mut Context) -> Option<RunnerState> {
        // TODO make sure that people confirmed to exit
        let control_scheme = std::mem::replace(&mut self.p1_control_scheme.scheme, None);
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
                    match button {
                        Button::DPadLeft => {
                            self.p1_control_scheme.assign_controller(id);
                        }
                        Button::DPadRight => {
                            self.p2_control_scheme.assign_controller(id);
                        }
                        _ => {}
                    }

                    self.p1_control_scheme.update_button(id, button);
                    self.p2_control_scheme.update_button(id, button);
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        let (p1, p2, imgui, pads_context) = (
            &mut self.p1_control_scheme,
            &mut self.p2_control_scheme,
            &mut self.imgui,
            &self.pads_context,
        );
        imgui
            .frame()
            .run(|ui| {
                imgui::Window::new(im_str!("P1 Button Check")).build(ui, || {
                    p1.draw_ui(ui, pads_context);
                });
                imgui::Window::new(im_str!("P2 Button Check")).build(ui, || {
                    p2.draw_ui(ui, pads_context);
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
