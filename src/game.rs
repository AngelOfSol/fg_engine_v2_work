mod animation_editor;

use crate::assets::Assets;
use crate::imgui_wrapper::ImGuiWrapper;

use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::graphics;
use ggez::input::mouse::MouseButton;
use ggez::timer;
use ggez::{Context, GameResult};

use std::collections::HashMap;

use animation_editor::AnimationEditor;

pub struct FightingGame {
    game_state: GameState,
    assets: Assets,
    imgui: ImGuiWrapper,
}

enum GameState {
    Animating(AnimationEditor),
}

impl FightingGame {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let mut assets = Assets {
            images: HashMap::new(),
        };
        Ok(Self {
            imgui: ImGuiWrapper::new(ctx),
            game_state: GameState::Animating(AnimationEditor::new(ctx, &mut assets)?),
            assets,
        })
    }
}

impl EventHandler for FightingGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {

        while timer::check_update_time(ctx, 60) {

            match self.game_state {
                GameState::Animating(ref mut editor) => editor.update(),
            }?;
        }
        Ok(())
        // Update code here...
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        match self.game_state {
            GameState::Animating(ref mut editor) => editor.draw(ctx, & mut self.assets, &mut self.imgui),
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
