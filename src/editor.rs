mod animation_editor;
mod bullet_editor;
mod character_editor;
mod main_menu;
mod state_editor;

use crate::assets::Assets;
use crate::imgui_wrapper::ImGuiWrapper;

use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::graphics;
use ggez::input::mouse::MouseButton;
use ggez::timer;
use ggez::{Context, GameResult};

pub use animation_editor::AnimationEditor;
pub use bullet_editor::BulletInfoEditor;
pub use character_editor::CharacterEditor;
pub use main_menu::MainMenu;
pub use state_editor::StateEditor;

use crate::character::{BulletInfo, EditorCharacterState};
use crate::graphics::Animation;

use crate::runner::{AppState, RunnerState};

pub struct GameEditor {
    game_state: Vec<(EditorState, Mode)>,
    assets: Assets,
    imgui: ImGuiWrapper,
}

#[allow(clippy::large_enum_variant)]
pub enum EditorState {
    Animating(AnimationEditor),
    MainMenu(MainMenu),
    StateEditor(StateEditor),
    CharacterEditor(CharacterEditor),
    BulletInfoEditor(BulletInfoEditor),
}

impl EditorState {
    fn handle_event(&mut self, passed_data: MessageData, mode: Mode) {
        match self {
            EditorState::StateEditor(ref mut editor) => {
                editor.handle_message(passed_data, mode);
            }
            EditorState::CharacterEditor(ref mut editor) => {
                editor.handle_message(passed_data, mode);
            }
            EditorState::BulletInfoEditor(ref mut editor) => {
                editor.handle_message(passed_data, mode);
            }
            _ => (),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    Standalone,
    Edit(String),
    New,
}

pub enum MessageData {
    Animation(Animation),
    State(EditorCharacterState),
    BulletInfo(BulletInfo),
}

pub enum Transition {
    None,
    Pop(Option<MessageData>),
    Push(Box<EditorState>, Mode),
}

impl GameEditor {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(GameEditor {
            imgui: ImGuiWrapper::new(ctx),
            game_state: vec![(MainMenu::new().into(), Mode::Standalone)],
            assets: Assets::new(),
        })
    }
}

impl AppState for GameEditor {
    fn next_appstate(&mut self, _: &mut Context) -> Option<RunnerState> {
        None
    }
}

impl EventHandler for GameEditor {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while timer::check_update_time(ctx, 60) {
            let transition = match self
                .game_state
                .last_mut()
                .expect("should have at least one gamestate")
                .0
            {
                EditorState::Animating(ref mut editor) => editor.update(),
                EditorState::MainMenu(ref mut menu) => menu.update(),
                EditorState::StateEditor(ref mut editor) => editor.update(),
                EditorState::CharacterEditor(ref mut editor) => editor.update(),
                EditorState::BulletInfoEditor(ref mut editor) => editor.update(),
            }?;
            match transition {
                Transition::None => (),
                Transition::Pop(Some(passed_data)) => {
                    let mode = self.game_state.pop().unwrap().1;
                    self.game_state
                        .last_mut()
                        .unwrap()
                        .0
                        .handle_event(passed_data, mode);
                }
                Transition::Pop(None) => {
                    self.game_state.pop();
                }
                Transition::Push(state, mode) => self.game_state.push((*state, mode)),
            }

            if self.game_state.is_empty() {
                ggez::event::quit(ctx);
            }
        }
        Ok(())
        // Update code here...
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        if self.game_state.is_empty() {
            return Ok(());
        }

        match self
            .game_state
            .last_mut()
            .expect("should have at least one gamestate")
            .0
        {
            EditorState::Animating(ref mut editor) => {
                editor.draw(ctx, &mut self.assets, &mut self.imgui)
            }
            EditorState::MainMenu(ref mut menu) => {
                menu.draw(ctx, &mut self.assets, &mut self.imgui)
            }
            EditorState::StateEditor(ref mut editor) => {
                editor.draw(ctx, &mut self.assets, &mut self.imgui)
            }
            EditorState::CharacterEditor(ref mut editor) => {
                editor.draw(ctx, &mut self.assets, &mut self.imgui)
            }
            EditorState::BulletInfoEditor(ref mut editor) => {
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

    fn resize_event(&mut self, ctx: &mut Context, _width: f32, _height: f32) {
        self.imgui.resize(ctx);
    }
}
