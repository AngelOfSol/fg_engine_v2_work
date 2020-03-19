use crate::app_state::{AppContext, AppState, Transition};
use crate::input::control_scheme::{is_valid_input_button, render_button_list, PadControlScheme};
use ggez::graphics;
use ggez::timer;
use ggez::{Context, GameResult};
use gilrs::{Button, Event, EventType, GamepadId, Gilrs};
use imgui::*;

pub struct ButtonCheck {
    active_control_schemes: Vec<CreateScheme>,
}

struct CreateScheme {
    scheme: PadControlScheme,
    selected_cell: usize,
    ready: bool,
}

impl CreateScheme {
    fn new(scheme: PadControlScheme) -> Self {
        Self {
            scheme,
            selected_cell: 0,
            ready: false,
        }
    }

    pub fn update_button(&mut self, id: GamepadId, button: Button) {
        if self.scheme.gamepad != id {
            return;
        }
        if is_valid_input_button(button) {
            if self.selected_cell == 4 {
                self.ready = !self.ready;
            } else {
                let buttons = &mut self.scheme.buttons[self.selected_cell];
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

    pub fn draw_ui(&mut self, ui: &Ui<'_>, pads: &Gilrs) {
        let gamepad = pads.gamepad(self.scheme.gamepad);
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
            ui.text(render_button_list(&self.scheme.buttons[index]));
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
}

impl ButtonCheck {
    pub fn new(_: &mut Context) -> GameResult<Self> {
        Ok(ButtonCheck {
            active_control_schemes: Vec::new(),
        })
    }
}

impl AppState for ButtonCheck {
    fn on_enter(
        &mut self,
        _: &mut Context,
        AppContext {
            ref mut pads,
            ref control_schemes,
            ..
        }: &mut AppContext,
    ) -> GameResult<()> {
        if let Some((id, _)) = pads.gamepads().next() {
            let scheme = control_schemes
                .get(&id)
                .cloned()
                .unwrap_or(PadControlScheme::new(id));
            self.active_control_schemes.push(CreateScheme::new(scheme));
        }
        Ok(())
    }
    fn update(
        &mut self,
        ctx: &mut Context,
        AppContext {
            ref mut pads,
            ref mut control_schemes,
            ..
        }: &mut AppContext,
    ) -> GameResult<Transition> {
        while timer::check_update_time(ctx, 60) {
            while let Some(event) = pads.next_event() {
                // let id = event.id;
                let Event { id, event, .. } = event;
                if let EventType::ButtonPressed(button, _) = event {
                    match button {
                        Button::Start => {
                            if !self
                                .active_control_schemes
                                .iter()
                                .any(|item| item.scheme.gamepad == id)
                            {
                                let scheme = control_schemes
                                    .get(&id)
                                    .cloned()
                                    .unwrap_or(PadControlScheme::new(id));
                                self.active_control_schemes.push(CreateScheme::new(scheme));
                            }
                        }
                        _ => {}
                    }
                    for scheme in self.active_control_schemes.iter_mut() {
                        scheme.update_button(id, button);
                    }
                    let (retain, updated): (Vec<_>, Vec<_>) = self
                        .active_control_schemes
                        .drain(..)
                        .partition(|item| !item.ready);
                    self.active_control_schemes = retain;
                    for scheme in updated {
                        control_schemes.insert(scheme.scheme.gamepad, scheme.scheme);
                    }
                }
            }
        }

        if self.active_control_schemes.len() > 0 {
            Ok(Transition::None)
        } else {
            Ok(Transition::Pop)
        }
    }
    fn draw(
        &mut self,
        ctx: &mut Context,
        AppContext {
            ref mut imgui,
            ref mut pads,
            ..
        }: &mut AppContext,
    ) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        // TODO organize windows positioning wise
        imgui
            .frame()
            .run(|ui| {
                let schemes = &mut self.active_control_schemes;
                let offset = schemes.len() as f32 / 2.0;
                let middle = graphics::drawable_size(ctx).0 / 2.0;
                for (idx, scheme) in schemes.iter_mut().enumerate() {
                    imgui::Window::new(&im_str!("Gamepad {}", scheme.scheme.gamepad))
                        .position(
                            [(idx as f32 - offset) * 200.0 + middle, 200.0],
                            Condition::Always,
                        )
                        .build(ui, || {
                            scheme.draw_ui(ui, pads);
                        });
                }

                imgui::Window::new(im_str!("Controller List"))
                    .position([middle - 150.0, 0.0], Condition::Always)
                    .size([300.0, 200.0], Condition::Always)
                    .build(ui, || {
                        for (id, _) in pads.gamepads().filter(|(id, _)| {
                            !self
                                .active_control_schemes
                                .iter()
                                .any(|scheme| scheme.scheme.gamepad == *id)
                        }) {
                            ui.text(format!("Gamepad {}", id.to_string()));
                        }
                    });
            })
            .render(ctx);

        graphics::present(ctx)?;

        Ok(())
    }
}

const GREEN: [f32; 4] = [0.2, 1.0, 0.2, 1.0];
