use clipboard::ClipboardProvider;
use gfx_core::{handle::RenderTargetView, memory::Typed};
use gfx_device_gl;
use ggez::graphics;
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::Context;
use imgui::*;
use imgui::{ImStr, ImString};
use imgui_gfx_renderer::*;
use std::marker::PhantomData;
use std::time::Instant;

type RendererType = imgui_gfx_renderer::Renderer<gfx::format::Rgba8, gfx_device_gl::Resources>;

struct ClipboardBackend {
    ctx: clipboard::ClipboardContext,
}

impl ClipboardBackend {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        //TODO log clipboard errors
        Ok(ClipboardBackend {
            ctx: clipboard::ClipboardContext::new()?,
        })
    }
}

impl imgui::ClipboardBackend for ClipboardBackend {
    fn get(&mut self) -> Option<ImString> {
        //TODO log clipboard errors
        Some(im_str!("{}", self.ctx.get_contents().ok()?))
    }
    fn set(&mut self, value: &ImStr) {
        //TODO log clipboard errors
        let _ = self.ctx.set_contents(value.to_str().to_owned());
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
struct MouseState {
    pos: (f32, f32),
    pressed: (bool, bool, bool),
    wheel: f32,
}

pub struct ImGuiWrapper {
    pub imgui: imgui::Context,
    pub renderer: RendererType,
    last_frame: Instant,
    mouse_state: MouseState,
    gamepad_state: [f32; 21],
}

pub struct ImguiFrameRunner<'ui, 'parent: 'ui, Stage> {
    ui: Ui<'ui>,
    renderer: &'parent mut RendererType,

    data: PhantomData<Stage>,
}
pub struct RunUi;
pub struct Render;

impl<'ui, 'parent: 'ui> ImguiFrameRunner<'ui, 'parent, RunUi> {
    fn new(holding: &'parent mut ImGuiWrapper) -> ImguiFrameRunner<'ui, 'parent, RunUi> {
        ImguiFrameRunner {
            ui: holding.imgui.frame(), //(frame_size, delta_s),
            renderer: &mut holding.renderer,
            data: PhantomData,
        }
    }

    pub fn run<F: FnOnce(&Ui<'ui>) -> ()>(
        self,
        run_ui: F,
    ) -> ImguiFrameRunner<'ui, 'parent, Render> {
        run_ui(&self.ui);

        ImguiFrameRunner {
            ui: self.ui,
            renderer: self.renderer,
            data: PhantomData,
        }
    }
}

impl<'ui, 'parent: 'ui> ImguiFrameRunner<'ui, 'parent, Render> {
    pub fn render(self, ctx: &mut Context) {
        let (factory, _, encoder, _, render_target) = graphics::gfx_objects(ctx);
        self.renderer
            .render(
                &mut *factory,
                encoder,
                &mut RenderTargetView::new(render_target.clone()),
                self.ui.render(),
            )
            .unwrap();
    }
}

impl ImGuiWrapper {
    pub fn new(ctx: &mut Context) -> Self {
        // Create the imgui object
        let mut imgui = imgui::Context::create();

        if let Ok(ctx) = ClipboardBackend::new() {
            imgui.set_clipboard_backend(Box::new(ctx));
        }

        let mut io = imgui.io_mut();
        io.key_map[Key::Tab as usize] = KeyCode::Tab as u32;
        io.key_map[Key::LeftArrow as usize] = KeyCode::Left as u32;
        io.key_map[Key::RightArrow as usize] = KeyCode::Right as u32;
        io.key_map[Key::UpArrow as usize] = KeyCode::Up as u32;
        io.key_map[Key::DownArrow as usize] = KeyCode::Down as u32;
        io.key_map[Key::PageUp as usize] = KeyCode::PageUp as u32;
        io.key_map[Key::PageDown as usize] = KeyCode::PageDown as u32;
        io.key_map[Key::Home as usize] = KeyCode::Home as u32;
        io.key_map[Key::End as usize] = KeyCode::End as u32;
        io.key_map[Key::Delete as usize] = KeyCode::Delete as u32;
        io.key_map[Key::Backspace as usize] = KeyCode::Back as u32;
        io.key_map[Key::Enter as usize] = KeyCode::Return as u32;
        io.key_map[Key::Escape as usize] = KeyCode::Escape as u32;
        io.key_map[Key::Space as usize] = KeyCode::Space as u32;
        io.key_map[Key::A as usize] = KeyCode::A as u32;
        io.key_map[Key::C as usize] = KeyCode::C as u32;
        io.key_map[Key::V as usize] = KeyCode::V as u32;
        io.key_map[Key::X as usize] = KeyCode::X as u32;
        io.key_map[Key::Y as usize] = KeyCode::Y as u32;
        io.key_map[Key::Z as usize] = KeyCode::Z as u32;
        io.config_flags
            .set(imgui::ConfigFlags::NAV_ENABLE_KEYBOARD, true);
        io.config_flags
            .set(imgui::ConfigFlags::NAV_ENABLE_GAMEPAD, true);
        io.backend_flags.set(imgui::BackendFlags::HAS_GAMEPAD, true);
        io.nav_visible = true;
        io.nav_active = true;

        // Shaders
        let shaders = {
            let (_, device, _, _, _) = graphics::gfx_objects(ctx);

            let version = device.get_info().shading_language;
            if version.is_embedded {
                if version.major >= 3 {
                    Shaders::GlSlEs300
                } else {
                    Shaders::GlSlEs100
                }
            } else if version.major >= 4 {
                Shaders::GlSl400
            } else if version.major >= 3 {
                if version.minor >= 2 {
                    Shaders::GlSl150
                } else {
                    Shaders::GlSl130
                }
            } else {
                Shaders::GlSl110
            }
        };

        // Renderer
        let (factory, _, _, _, _) = graphics::gfx_objects(ctx);

        let renderer = RendererType::init(&mut imgui, &mut *factory, shaders).unwrap();

        let screen_size = graphics::drawable_size(ctx);
        imgui.io_mut().display_size = [screen_size.0, screen_size.1];
        imgui.io_mut().font_global_scale = 1.0;
        // Create new frame

        /*let frame_size = FrameSize {
            logical_size: (f64::from(w), f64::from(h)),
            hidpi_factor: f64::from(graphics::hidpi_factor(ctx)),
        };*/

        let now = Instant::now();
        imgui.io_mut().update_delta_time(now);
        // Create instace
        Self {
            imgui,
            renderer,
            last_frame: Instant::now(),
            mouse_state: MouseState::default(),
            gamepad_state: [0.0; 21],
        }
    }
    pub fn resize(&mut self, ctx: &Context) {
        let screen_size = graphics::drawable_size(ctx);
        self.imgui.io_mut().display_size = [screen_size.0, screen_size.1];
        self.imgui.io_mut().font_global_scale = 1.0;
    }

    pub fn frame(&mut self) -> ImguiFrameRunner<'_, '_, RunUi> {
        // Update mouse
        self.update_mouse();
        self.update_gamepad();

        //let now = Instant::now();
        self.last_frame = self.imgui.io_mut().update_delta_time(self.last_frame);

        ImguiFrameRunner::new(self)
    }

    fn update_gamepad(&mut self) {
        let mut io = self.imgui.io_mut();
        io.nav_inputs = self.gamepad_state;
    }
    fn update_mouse(&mut self) {
        let mut io = self.imgui.io_mut();
        io.mouse_pos = [self.mouse_state.pos.0, self.mouse_state.pos.1];

        io.mouse_down = [
            self.mouse_state.pressed.0,
            self.mouse_state.pressed.1,
            self.mouse_state.pressed.2,
            false,
            false,
        ];

        io.mouse_wheel = self.mouse_state.wheel;
        self.mouse_state.wheel = 0.0;
    }

    pub fn update_mouse_pos(&mut self, x: f32, y: f32) {
        self.mouse_state.pos = (x, y);
    }
    pub fn update_mouse_scroll(&mut self, value: f32) {
        self.mouse_state.wheel = value;
    }

    pub fn update_mouse_down(&mut self, pressed: (bool, bool, bool)) {
        self.mouse_state.pressed = pressed;

        if pressed.0 {}
    }

    pub fn handle_gamepad_input(&mut self, input: NavInput, value: f32) {
        self.gamepad_state[input as usize] = value;
    }
    pub fn handle_keyboard_input(&mut self, keycode: KeyCode, keymod: KeyMods, is_down: bool) {
        let mut io = self.imgui.io_mut();
        io.key_shift = keymod.contains(KeyMods::SHIFT);
        io.key_ctrl = keymod.contains(KeyMods::CTRL);
        io.key_alt = keymod.contains(KeyMods::ALT);
        io.key_super = keymod.contains(KeyMods::LOGO);
        io.keys_down[keycode as usize] = is_down;
        match keycode {
            KeyCode::LShift | KeyCode::RShift => io.key_shift = is_down,
            KeyCode::LControl | KeyCode::RControl => io.key_ctrl = is_down,
            KeyCode::LAlt | KeyCode::RAlt => io.key_alt = is_down,
            KeyCode::LWin | KeyCode::RWin => io.key_super = is_down,
            _ => (),
        }
    }

    pub fn handle_text_input(&mut self, character: char) {
        Io::add_input_character(self.imgui.io_mut(), character);
    }
}
