use crate::imgui_wrapper::ImGuiWrapper;
use fg_controller::{
    backend::{Button, ControllerBackend},
    control_mapping::ControlMapping,
};
use fg_input::axis::Axis;
use fg_netcode::Networking;
use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::input::mouse::MouseButton;
use ggez::{Context, GameResult};
use imgui::NavInput;
use laminar::{Config, Socket};
use sdl_controller_backend::{ControllerId, SdlController};
use std::collections::HashMap;
use std::net::{SocketAddr, ToSocketAddrs};
use std::time::Duration;

pub enum Transition {
    Push(Box<dyn AppState>),
    Replace(Box<dyn AppState>),
    Pop,
    None,
}

pub struct AppContext {
    pub controllers: SdlController,
    pub imgui: ImGuiWrapper,
    pub control_schemes: HashMap<ControllerId, ControlMapping>,
    pub audio: rodio::Device,
    pub socket: Option<Socket>,
    pub networking: Networking,
}

pub trait AppState {
    fn update(&mut self, ctx: &mut Context, app_ctx: &mut AppContext) -> GameResult<Transition>;
    fn on_enter(&mut self, ctx: &mut Context, app_ctx: &mut AppContext) -> GameResult<()>;
    fn draw(&mut self, ctx: &mut Context, app_ctx: &mut AppContext) -> GameResult<()>;
}

pub struct AppStateRunner {
    history: Vec<Box<dyn AppState>>,
    app_ctx: AppContext,
    last_draw_time: std::time::Instant,
}

impl AppStateRunner {
    pub fn new(ctx: &mut Context, mut start: Box<dyn AppState>) -> GameResult<Self> {
        let audio = rodio::default_output_device().unwrap();

        let adapter = ipconfig::get_adapters().ok().and_then(|adapters| {
            adapters
                .into_iter()
                .find(|x| x.friendly_name() == "Ethernet")
        });

        let mut app_ctx = AppContext {
            controllers: SdlController::new().unwrap(),
            imgui: ImGuiWrapper::new(ctx),
            control_schemes: HashMap::new(),
            audio,
            networking: Networking::new(),
            socket: Socket::bind_with_config(
                adapter
                    .and_then(|adapter| {
                        adapter
                            .ip_addresses()
                            .iter()
                            .find(|item| item.is_ipv4())
                            .cloned()
                    })
                    .map(|ip| {
                        (0..10)
                            .into_iter()
                            .map(|i| format!("{}:1080{}", ip, i))
                            .collect()
                    })
                    .unwrap_or_else(|| vec!["127.0.0.1:10800".to_owned()])
                    .into_iter()
                    .filter_map(|item| item.to_socket_addrs().ok())
                    .flatten()
                    .collect::<Vec<SocketAddr>>()
                    .as_slice(),
                Config {
                    blocking_mode: false,
                    rtt_max_value: 2000,
                    idle_connection_timeout: Duration::from_secs(5),
                    heartbeat_interval: Some(Duration::from_millis(200)),
                    ..Config::default()
                },
            )
            .ok(),
        };
        start.on_enter(ctx, &mut app_ctx)?;
        Ok(AppStateRunner {
            history: vec![start],
            app_ctx,
            last_draw_time: std::time::Instant::now(),
        })
    }
}

impl EventHandler for AppStateRunner {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.app_ctx.controllers.poll();

        if let Some((_, controller)) = self.app_ctx.controllers.active_state() {
            self.app_ctx.imgui.handle_gamepad_input(
                NavInput::Activate,
                if controller[Button::A] { 1.0 } else { 0.0 },
            );
            self.app_ctx.imgui.handle_gamepad_input(
                NavInput::Cancel,
                if controller[Button::X] { 1.0 } else { 0.0 },
            );
            self.app_ctx.imgui.handle_gamepad_input(
                NavInput::Input,
                if controller[Button::B] { 1.0 } else { 0.0 },
            );
            self.app_ctx.imgui.handle_gamepad_input(
                NavInput::Menu,
                if controller[Button::Y] { 1.0 } else { 0.0 },
            );
            self.app_ctx.imgui.handle_gamepad_input(
                NavInput::FocusPrev,
                if controller[Button::L1] { 1.0 } else { 0.0 },
            );
            self.app_ctx.imgui.handle_gamepad_input(
                NavInput::FocusNext,
                if controller[Button::R1] { 1.0 } else { 0.0 },
            );

            self.app_ctx.imgui.handle_gamepad_input(
                NavInput::DpadLeft,
                if controller.axis().x() == Axis::Left {
                    1.0
                } else {
                    0.0
                },
            );
            self.app_ctx.imgui.handle_gamepad_input(
                NavInput::DpadRight,
                if controller.axis().x() == Axis::Right {
                    1.0
                } else {
                    0.0
                },
            );
            self.app_ctx.imgui.handle_gamepad_input(
                NavInput::DpadUp,
                if controller.axis().y() == Axis::Up {
                    1.0
                } else {
                    0.0
                },
            );
            self.app_ctx.imgui.handle_gamepad_input(
                NavInput::DpadDown,
                if controller.axis().y() == Axis::Down {
                    1.0
                } else {
                    0.0
                },
            );
        }

        if let Some(state) = self.history.last_mut() {
            match state.update(ctx, &mut self.app_ctx)? {
                Transition::Push(new_state) => {
                    self.history.push(new_state);
                    self.history
                        .last_mut()
                        .unwrap()
                        .on_enter(ctx, &mut self.app_ctx)?;
                }
                Transition::Replace(new_state) => {
                    self.history.pop();
                    self.history.push(new_state);
                    self.history
                        .last_mut()
                        .unwrap()
                        .on_enter(ctx, &mut self.app_ctx)?;
                }
                Transition::Pop => {
                    self.history.pop();
                    if let Some(ref mut state) = self.history.last_mut() {
                        state.on_enter(ctx, &mut self.app_ctx)?;
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
        let time = std::time::Instant::now();
        // draw at most 100FPS
        if time - self.last_draw_time > std::time::Duration::from_millis(6) {
            self.last_draw_time = time;
            if let Some(state) = self.history.last_mut() {
                state.draw(ctx, &mut self.app_ctx)?;
            }
        }
        ggez::timer::yield_now();
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
        self.app_ctx.imgui.update_mouse_up((
            button == MouseButton::Left,
            button == MouseButton::Right,
            button == MouseButton::Middle,
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
