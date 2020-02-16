use crate::imgui_wrapper::ImGuiWrapper;

use crate::app_state::{AppState, Transition};
use crate::game_match::Match;
use crate::input::control_scheme::{is_valid_input_button, render_button_list, PadControlScheme};
use ggez::graphics;
use ggez::timer;
use ggez::{Context, GameResult};
use gilrs::{Button, Event, EventType, GamepadId, Gilrs};
use imgui::*;

pub struct ButtonCheck {
    p1_control_scheme: CreateScheme,
    p2_control_scheme: CreateScheme,
    pads_context: Gilrs,
}

struct CreateScheme {
    scheme: Option<PadControlScheme>,
    selected_cell: usize,
    ready: bool,
}

impl CreateScheme {
    fn new() -> Self {
        Self {
            scheme: None,
            selected_cell: 0,
            ready: false,
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
                    self.ready = !self.ready;
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
    pub fn new(_: &mut Context) -> GameResult<Self> {
        Ok(ButtonCheck {
            pads_context: Gilrs::new()?,
            p1_control_scheme: CreateScheme::new(),
            p2_control_scheme: CreateScheme::new(),
        })
    }
}

impl AppState for ButtonCheck {
    fn update(&mut self, ctx: &mut Context) -> GameResult<Transition> {
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
        if self.p1_control_scheme.ready
            && self.p2_control_scheme.ready
            && self.p1_control_scheme.scheme.is_some()
            && self.p2_control_scheme.scheme.is_some()
        {
            let p1 = std::mem::replace(&mut self.p1_control_scheme.scheme, None).unwrap();
            let p2 = std::mem::replace(&mut self.p2_control_scheme.scheme, None).unwrap();

            Ok(Transition::Replace(Box::new(Match::new(ctx, p1, p2)?)))
        } else {
            Ok(Transition::None)
        }
    }
    fn draw(&mut self, ctx: &mut Context, imgui: &mut ImGuiWrapper) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        let (p1, p2, pads_context) = (
            &mut self.p1_control_scheme,
            &mut self.p2_control_scheme,
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
}

const GREEN: [f32; 4] = [0.2, 1.0, 0.2, 1.0];
