mod animation_editor;
mod main_menu;
mod state_editor;

use crate::assets::Assets;
use crate::imgui_wrapper::ImGuiWrapper;

use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::graphics;
use ggez::input::mouse::MouseButton;
use ggez::timer;
use ggez::{Context, GameResult};


use animation_editor::AnimationEditor;
use main_menu::MainMenu;
use state_editor::StateEditor;

pub struct FightingGame {
    game_state: Vec<GameState>,
    assets: Assets,
    imgui: ImGuiWrapper,
}

pub enum GameState {
    Animating(AnimationEditor),
    MainMenu(MainMenu),
    StateEditor(StateEditor),
}


pub enum Transition {
    None,
    Pop,
    Push(Box<GameState>),
}

impl FightingGame {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {

        Ok(Self {
            imgui: ImGuiWrapper::new(ctx),
            game_state: vec![MainMenu::new().into()],
            assets: Assets::new(),
        })
    }
}

impl EventHandler for FightingGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while timer::check_update_time(ctx, 60) {
            let transition = match self
                .game_state
                .last_mut()
                .expect("should have at least one gamestate")
            {
                GameState::Animating(ref mut editor) => editor.update(),
                GameState::MainMenu(ref mut menu) => menu.update(),
                GameState::StateEditor(ref mut editor) => editor.update(),
            }?;
            match transition {
                Transition::None => (),
                Transition::Pop => {
                    self.game_state.pop();
                }
                Transition::Push(state) => {
                    self.game_state.push(*state);
                }
            }
        }
        Ok(())
        // Update code here...
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        match self
            .game_state
            .last_mut()
            .expect("should have at least one gamestate")
        {
            GameState::Animating(ref mut editor) => {
                editor.draw(ctx, &mut self.assets, &mut self.imgui)
            }

            GameState::MainMenu(ref mut menu) => menu.draw(ctx, &mut self.imgui),
            GameState::StateEditor(ref mut editor) => {
                editor.draw(ctx, &mut self.assets, &mut self.imgui)
            }
        }?;

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
}
