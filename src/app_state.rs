use crate::imgui_wrapper::ImGuiWrapper;
use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::input::mouse::MouseButton;
use ggez::{Context, GameResult};

pub enum Transition {
    Push(Box<dyn AppState>),
    Replace(Box<dyn AppState>),
    Pop,
    None,
}

pub trait AppState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<Transition>;
    fn draw(&mut self, ctx: &mut Context, imgui: &mut ImGuiWrapper) -> GameResult<()>;
}

pub struct AppStateRunner {
    history: Vec<Box<dyn AppState>>,
    imgui: ImGuiWrapper,
}

impl AppStateRunner {
    pub fn new(ctx: &mut Context, start: Box<dyn AppState>) -> GameResult<Self> {
        Ok(AppStateRunner {
            history: vec![start],
            imgui: ImGuiWrapper::new(ctx),
        })
    }
}

impl EventHandler for AppStateRunner {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if let Some(state) = self.history.last_mut() {
            match state.update(ctx)? {
                Transition::Push(new_state) => {
                    self.history.push(new_state);
                }
                Transition::Replace(new_state) => {
                    self.history.pop();
                    self.history.push(new_state);
                }
                Transition::Pop => {
                    self.history.pop();
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
            state.draw(ctx, &mut self.imgui)?;
        }
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
