use crate::imgui_wrapper::ImGuiWrapper;
use crate::input::control_scheme::PadControlScheme;
use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::input::mouse::MouseButton;
use ggez::{Context, GameResult};
use gilrs::{Button, EventType, GamepadId, Gilrs, GilrsBuilder};
use std::collections::HashMap;

pub enum Transition {
    Push(Box<dyn AppState>),
    Replace(Box<dyn AppState>),
    Pop,
    None,
}

pub struct AppContext {
    pub pads: Gilrs,
    pub imgui: ImGuiWrapper,
    pub control_schemes: HashMap<GamepadId, PadControlScheme>,
}

pub trait AppState {
    fn update(&mut self, ctx: &mut Context, app_ctx: &mut AppContext) -> GameResult<Transition>;
    fn on_enter(&mut self, ctx: &mut Context, app_ctx: &mut AppContext) -> GameResult<()>;
    fn draw(&mut self, ctx: &mut Context, app_ctx: &mut AppContext) -> GameResult<()>;
}

pub struct AppStateRunner {
    history: Vec<Box<dyn AppState>>,
    app_ctx: AppContext,
}

impl AppStateRunner {
    pub fn new(ctx: &mut Context, mut start: Box<dyn AppState>) -> GameResult<Self> {
        let mut app_ctx = AppContext {
            pads: GilrsBuilder::new().build()?,
            imgui: ImGuiWrapper::new(ctx),
            control_schemes: HashMap::new(),
        };
        start.on_enter(ctx, &mut app_ctx)?;
        Ok(AppStateRunner {
            history: vec![start],
            app_ctx,
        })
    }
}

impl EventHandler for AppStateRunner {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut events = Vec::new();

        while let Some(event) = self.app_ctx.pads.next_event() {
            match &event.event {
                EventType::ButtonPressed(button, _) => {
                    let nav_input = match button {
                        Button::South => Some(imgui::NavInput::Activate),
                        Button::East => Some(imgui::NavInput::Cancel),
                        Button::West => Some(imgui::NavInput::Input),
                        Button::North => Some(imgui::NavInput::Menu),
                        Button::DPadLeft => Some(imgui::NavInput::DpadLeft),
                        Button::DPadRight => Some(imgui::NavInput::DpadRight),
                        Button::DPadUp => Some(imgui::NavInput::DpadUp),
                        Button::DPadDown => Some(imgui::NavInput::DpadDown),
                        Button::LeftTrigger => Some(imgui::NavInput::FocusPrev),
                        Button::RightTrigger => Some(imgui::NavInput::FocusNext),
                        Button::LeftTrigger2 => Some(imgui::NavInput::TweakSlow),
                        Button::RightTrigger2 => Some(imgui::NavInput::TweakFast),
                        _ => None,
                    };
                    if let Some(nav_input) = nav_input {
                        self.app_ctx.imgui.handle_gamepad_input(nav_input, 1.0);
                    }
                }
                EventType::ButtonReleased(button, _) => {
                    let nav_input = match button {
                        Button::South => Some(imgui::NavInput::Activate),
                        Button::East => Some(imgui::NavInput::Cancel),
                        Button::West => Some(imgui::NavInput::Input),
                        Button::North => Some(imgui::NavInput::Menu),
                        Button::DPadLeft => Some(imgui::NavInput::DpadLeft),
                        Button::DPadRight => Some(imgui::NavInput::DpadRight),
                        Button::DPadUp => Some(imgui::NavInput::DpadUp),
                        Button::DPadDown => Some(imgui::NavInput::DpadDown),
                        Button::LeftTrigger => Some(imgui::NavInput::FocusPrev),
                        Button::RightTrigger => Some(imgui::NavInput::FocusNext),
                        Button::LeftTrigger2 => Some(imgui::NavInput::TweakSlow),
                        Button::RightTrigger2 => Some(imgui::NavInput::TweakFast),
                        _ => None,
                    };
                    if let Some(nav_input) = nav_input {
                        self.app_ctx.imgui.handle_gamepad_input(nav_input, 0.0);
                    }
                }
                _ => {}
            }
            events.push(event);
        }

        for event in events {
            self.app_ctx.pads.insert_event(event);
        }

        if let Some(state) = self.history.last_mut() {
            match state.update(ctx, &mut self.app_ctx)? {
                Transition::Push(new_state) => {
                    self.history.push(new_state);
                    self.history
                        .last_mut()
                        .unwrap()
                        .on_enter(ctx, &mut self.app_ctx)?;

                    while let Some(_) = self.app_ctx.pads.next_event() {}
                }
                Transition::Replace(new_state) => {
                    self.history.pop();
                    self.history.push(new_state);
                    self.history
                        .last_mut()
                        .unwrap()
                        .on_enter(ctx, &mut self.app_ctx)?;
                    while let Some(_) = self.app_ctx.pads.next_event() {}
                }
                Transition::Pop => {
                    self.history.pop();
                    if let Some(ref mut state) = self.history.last_mut() {
                        state.on_enter(ctx, &mut self.app_ctx)?;
                        while let Some(_) = self.app_ctx.pads.next_event() {}
                    }
                }
                Transition::None => (),
            }
        } else {
            ggez::event::quit(ctx);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        if let Some(state) = self.history.last_mut() {
            state.draw(ctx, &mut self.app_ctx)?;
        }
        Ok(())
    }
    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _xrel: f32, _yrel: f32) {
        self.app_ctx.imgui.update_mouse_pos(x, y);
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, y: f32) {
        self.app_ctx.imgui.update_mouse_scroll(y);
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.app_ctx.imgui.update_mouse_down((
            button == MouseButton::Left,
            button == MouseButton::Right,
            button == MouseButton::Middle,
        ));
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: f32, _y: f32) {
        self.app_ctx.imgui.update_mouse_down((
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
        self.app_ctx
            .imgui
            .handle_keyboard_input(keycode, keymod, true);
    }
    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, keymod: KeyMods) {
        self.app_ctx
            .imgui
            .handle_keyboard_input(keycode, keymod, false);
    }
    fn text_input_event(&mut self, _ctx: &mut Context, character: char) {
        self.app_ctx.imgui.handle_text_input(character);
    }

    fn resize_event(&mut self, ctx: &mut Context, _width: f32, _height: f32) {
        self.app_ctx.imgui.resize(ctx);
    }
}
