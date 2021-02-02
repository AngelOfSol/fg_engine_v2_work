use crate::app_state::{AppContext, AppState, Transition};
use fg_controller::{
    backend::{Button, ControllerBackend, ControllerState},
    control_mapping::ControlMapping,
};
use fg_input::{axis::Axis, button::ButtonSet};
use ggez::graphics;
use ggez::timer;
use ggez::{Context, GameResult};
use imgui::*;
use sdl_controller_backend::ControllerId;
use strum::IntoEnumIterator;

pub struct ButtonCheck {
    active_control_mappings: Vec<CreateMapping>,
    delay: u32,
}

struct CreateMapping {
    id: ControllerId,
    mapping: ControlMapping,
    selected_cell: usize,
    movement_delay: usize,
    button_change_delay: usize,
    start_delay: usize,
}

impl CreateMapping {
    fn new(id: ControllerId, scheme: ControlMapping) -> Self {
        Self {
            id,
            mapping: scheme,
            selected_cell: 0,
            movement_delay: 0,
            button_change_delay: 0,
            start_delay: 60,
        }
    }

    pub fn update(&mut self, state: ControllerState) -> bool {
        self.start_delay = self.start_delay.saturating_sub(1);
        self.movement_delay = self.movement_delay.saturating_sub(1);
        self.button_change_delay = self.button_change_delay.saturating_sub(1);

        if state[Button::Start] {
            return self.start_delay == 0;
        }

        if self.movement_delay == 0 && (state.dpad + state.left_stick) & Axis::Up == Axis::Up {
            if self.selected_cell > 0 {
                self.selected_cell -= 1;
            } else {
                self.selected_cell = 5;
            }
            self.movement_delay = 30;
        } else if self.movement_delay == 0
            && (state.dpad + state.left_stick) & Axis::Down == Axis::Down
        {
            if self.selected_cell < 5 {
                self.selected_cell += 1
            } else {
                self.selected_cell = 0;
            }
            self.movement_delay = 30;
        }
        if let Some(new_button) = Button::iter().find(|button| state[*button]) {
            if self.button_change_delay == 0 {
                let new_set = ButtonSet::from_id(self.selected_cell);

                if let Some(old_button) = self.mapping.find_button(&new_set) {
                    if let Some(old_set) = self.mapping.buttons.get(&new_button).copied() {
                        self.mapping.buttons.insert(old_button, old_set);
                    } else {
                        self.mapping.buttons.remove(&old_button);
                    }
                }
                self.mapping.buttons.insert(new_button, new_set);

                self.button_change_delay = 30;
            }
        }
        false
    }

    pub fn draw_ui(&mut self, ui: &Ui<'_>) {
        let id = ui.push_id(&format!("{:?}", self.id));

        ui.text(format!("Gamepad: {:?}", self.id));

        ui.columns(2, im_str!("Columns"), true);

        for index in 0..5 {
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
                4 => "E",
                _ => unreachable!(),
            });
            ui.next_column();

            if let Some(button) = self.mapping.find_button(&ButtonSet::from_id(index)) {
                ui.text(im_str!("{:?}", button))
            }

            ui.next_column();

            if let Some(token) = token {
                token.pop(ui);
            }
        }
        ui.columns(1, im_str!("Columns##End"), false);

        let token = if 5 == self.selected_cell {
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
            active_control_mappings: Vec::new(),
            delay: 30,
        })
    }
}

impl AppState for ButtonCheck {
    fn on_enter(
        &mut self,
        _: &mut Context,
        AppContext {
            ref mut controllers,
            ref control_schemes,
            ..
        }: &mut AppContext,
    ) -> GameResult<()> {
        if let Some(id) = controllers.active_controller() {
            let mapping = control_schemes.get(&id).cloned().unwrap_or_default();
            self.active_control_mappings
                .push(CreateMapping::new(id, mapping));
        }
        Ok(())
    }
    fn update(
        &mut self,
        ctx: &mut Context,
        AppContext {
            ref mut controllers,
            ref mut control_schemes,
            ..
        }: &mut AppContext,
    ) -> GameResult<Transition> {
        while timer::check_update_time(ctx, 60) {
            self.delay = self.delay.saturating_sub(1);
            if self.delay > 0 {
                // delay so the button moving into controller select does not
                continue;
            }
            for id in controllers.controllers() {
                let state = controllers.current_state(&id);
                let retain = if let Some(mapping) = self
                    .active_control_mappings
                    .iter_mut()
                    .find(|item| item.id == id)
                {
                    mapping.update(state)
                } else {
                    if state[Button::Start] {
                        let mapping = control_schemes.get(&id).cloned().unwrap_or_default();
                        self.active_control_mappings
                            .push(CreateMapping::new(id, mapping));
                    }
                    true
                };

                if !retain {
                    if let Some(index) = self
                        .active_control_mappings
                        .iter()
                        .position(|item| item.id == id)
                    {
                        self.active_control_mappings.remove(index);
                    }
                }
            }
        }

        if !self.active_control_mappings.is_empty() {
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
            ref mut controllers,
            ..
        }: &mut AppContext,
    ) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        imgui
            .frame()
            .run(|ui| {
                let schemes = &mut self.active_control_mappings;
                let offset = schemes.len() as f32 / 2.0;
                let middle = graphics::drawable_size(ctx).0 / 2.0;
                for (idx, scheme) in schemes.iter_mut().enumerate() {
                    imgui::Window::new(&im_str!("Gamepad {:?}", scheme.id))
                        .size([190.0, 210.0], Condition::Always)
                        .position(
                            [(idx as f32 - offset) * 200.0 + middle, 200.0],
                            Condition::Always,
                        )
                        .build(ui, || {
                            scheme.draw_ui(ui);
                        });
                }

                imgui::Window::new(im_str!("Controller List"))
                    .position([middle - 150.0, 0.0], Condition::Always)
                    .size([300.0, 200.0], Condition::Always)
                    .build(ui, || {
                        for id in controllers.controllers().filter(|id| {
                            !self
                                .active_control_mappings
                                .iter()
                                .any(|scheme| scheme.id == *id)
                        }) {
                            ui.text(format!("Gamepad {:?}", id));
                        }
                    });
            })
            .render(ctx);

        graphics::present(ctx)?;

        Ok(())
    }
}

const GREEN: [f32; 4] = [0.2, 1.0, 0.2, 1.0];
