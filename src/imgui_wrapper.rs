use ggez::graphics;
use ggez::Context;

use gfx_core::{handle::RenderTargetView, memory::Typed};

use gfx_device_gl;

use imgui::*;
use imgui_gfx_renderer::*;

use std::time::Instant;

use ggez::input::keyboard::{KeyCode, KeyMods};

#[derive(Copy, Clone, PartialEq, Debug, Default)]
struct MouseState {
    pos: (f32, f32),
    pressed: (bool, bool, bool),
    wheel: f32,
}

pub struct ImGuiWrapper {
    pub imgui: ImGui,
    pub renderer: Renderer<gfx_device_gl::Resources>,
    last_frame: Instant,
    mouse_state: MouseState,
}

impl ImGuiWrapper {
    pub fn new(ctx: &mut Context) -> Self {
        // Create the imgui object
        let mut imgui = ImGui::init();


        imgui.set_imgui_key(ImGuiKey::Tab, KeyCode::Tab as u8);
        imgui.set_imgui_key(ImGuiKey::LeftArrow, KeyCode::Left as u8);
        imgui.set_imgui_key(ImGuiKey::RightArrow, KeyCode::Right as u8);
        imgui.set_imgui_key(ImGuiKey::UpArrow, KeyCode::Up as u8);
        imgui.set_imgui_key(ImGuiKey::DownArrow, KeyCode::Down as u8);
        imgui.set_imgui_key(ImGuiKey::PageUp, KeyCode::PageUp as u8);
        imgui.set_imgui_key(ImGuiKey::PageDown, KeyCode::PageDown as u8);
        imgui.set_imgui_key(ImGuiKey::Home, KeyCode::Home as u8);
        imgui.set_imgui_key(ImGuiKey::End, KeyCode::End as u8);
        imgui.set_imgui_key(ImGuiKey::Delete, KeyCode::Delete as u8);
        imgui.set_imgui_key(ImGuiKey::Backspace, KeyCode::Back as u8);
        imgui.set_imgui_key(ImGuiKey::Enter, KeyCode::Return as u8);
        imgui.set_imgui_key(ImGuiKey::Escape, KeyCode::Escape as u8);
        imgui.set_imgui_key(ImGuiKey::A, KeyCode::A as u8);
        imgui.set_imgui_key(ImGuiKey::C, KeyCode::C as u8);
        imgui.set_imgui_key(ImGuiKey::V, KeyCode::V as u8);
        imgui.set_imgui_key(ImGuiKey::X, KeyCode::X as u8);
        imgui.set_imgui_key(ImGuiKey::Y, KeyCode::Y as u8);
        imgui.set_imgui_key(ImGuiKey::Z, KeyCode::Z as u8);
        // Shaders
        let shaders = {
            let version = graphics::device(ctx).get_info().shading_language;
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
        let render_target = graphics::screen_render_target(ctx);
        let factory = graphics::factory(ctx);

        let renderer = Renderer::init(
            &mut imgui,
            &mut *factory,
            shaders,
            RenderTargetView::new(render_target.clone()),
        )
        .unwrap();

        // Create instace
        Self {
            imgui,
            renderer,
            last_frame: Instant::now(),
            mouse_state: MouseState::default(),

        }
    }


    pub fn render<F: FnMut(&Ui) -> ()>(&mut self, ctx: &mut Context, mut run_ui: F) {
        // Update mouse
        self.update_mouse();

        // Create new frame
        let screen_size = graphics::drawable_size(ctx);
        let w = screen_size.0;
        let h = screen_size.1;

        let frame_size = FrameSize {
            logical_size: (f64::from(w), f64::from(h)),
            hidpi_factor: f64::from(graphics::hidpi_factor(ctx)),
        };

        let now = Instant::now();
        let delta = now - self.last_frame;
        let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
        self.last_frame = now;

        let ui = self.imgui.frame(frame_size, delta_s);

        run_ui(&ui);

        // Render
        let (factory, _, encoder, _, _) = graphics::gfx_objects(ctx);
        self.renderer.render(ui, &mut *factory, encoder).unwrap();

    }

    fn update_mouse(&mut self) {
        self.imgui
            .set_mouse_pos(self.mouse_state.pos.0, self.mouse_state.pos.1);

        self.imgui.set_mouse_down([
            self.mouse_state.pressed.0,
            self.mouse_state.pressed.1,
            self.mouse_state.pressed.2,
            false,
            false,
        ]);

        self.imgui.set_mouse_wheel(self.mouse_state.wheel);
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
    pub fn handle_keyboard_input(&mut self, keycode: KeyCode, keymod: KeyMods, is_down: bool) {
        self.imgui.set_key_shift(keymod.contains(KeyMods::SHIFT));
        self.imgui.set_key_ctrl(keymod.contains(KeyMods::CTRL));
        self.imgui.set_key_alt(keymod.contains(KeyMods::ALT));
        self.imgui.set_key_super(keymod.contains(KeyMods::LOGO));
        self.imgui.set_key(keycode as _, is_down);
        match keycode {
            KeyCode::LShift | KeyCode::RShift => self.imgui.set_key_shift(is_down),
            KeyCode::LControl | KeyCode::RControl => self.imgui.set_key_ctrl(is_down),
            KeyCode::LAlt | KeyCode::RAlt => self.imgui.set_key_alt(is_down),
            KeyCode::LWin | KeyCode::RWin => self.imgui.set_key_super(is_down),
            _ => (),
        }
    }

    pub fn handle_text_input(&mut self, character: char) {
        self.imgui.add_input_character(character);
    }
}