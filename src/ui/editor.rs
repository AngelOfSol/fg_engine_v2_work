mod animation_editor;
mod attack_editor;
mod bullet_editor;
mod character_editor;
mod main_menu;
mod state_editor;

use crate::app_state::AppState;
use crate::assets::Assets;
use crate::character::components::{AttackInfo, BulletInfo};
use crate::character::state::EditorCharacterState;
use crate::graphics::Animation;
use crate::imgui_wrapper::ImGuiWrapper;
pub use animation_editor::AnimationEditor;
pub use attack_editor::AttackInfoEditor;
pub use bullet_editor::BulletInfoEditor;
pub use character_editor::CharacterEditor;
use ggez::graphics;
use ggez::timer;
use ggez::{Context, GameResult};
pub use main_menu::MainMenu;
pub use state_editor::StateEditor;

pub struct GameEditor {
    game_state: Vec<(EditorState, Mode)>,
    assets: Assets,
}
#[allow(clippy::large_enum_variant)]
pub enum EditorState {
    Animating(AnimationEditor),
    MainMenu(MainMenu),
    StateEditor(StateEditor),
    CharacterEditor(CharacterEditor),
    BulletInfoEditor(BulletInfoEditor),
    AttackInfoEditor(AttackInfoEditor),
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
    AttackInfo(AttackInfo),
}

pub enum Transition {
    None,
    Pop(Option<MessageData>),
    Push(Box<EditorState>, Mode),
}

impl GameEditor {
    pub fn new(_: &mut Context) -> GameResult<Self> {
        Ok(GameEditor {
            game_state: vec![(MainMenu::new().into(), Mode::Standalone)],
            assets: Assets::new(),
        })
    }
}

impl AppState for GameEditor {
    fn on_enter(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::set_mode(
            ctx,
            ggez::conf::WindowMode::default().dimensions(1500.0, 720.0),
        )?;
        graphics::set_screen_coordinates(ctx, ggez::graphics::Rect::new(0.0, 0.0, 1500.0, 720.0))
    }
    // TODO remove teh full qualify
    fn update(&mut self, ctx: &mut Context) -> GameResult<crate::app_state::Transition> {
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
                EditorState::AttackInfoEditor(ref mut editor) => editor.update(),
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
        }
        // TODO remove teh full qualify
        if self.game_state.is_empty() {
            Ok(crate::app_state::Transition::Pop)
        } else {
            Ok(crate::app_state::Transition::None)
        }
    }
    fn draw(&mut self, ctx: &mut Context, imgui: &mut ImGuiWrapper) -> GameResult<()> {
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
            EditorState::Animating(ref mut editor) => editor.draw(ctx, &mut self.assets, imgui),
            EditorState::MainMenu(ref mut menu) => menu.draw(ctx, &mut self.assets, imgui),
            EditorState::StateEditor(ref mut editor) => editor.draw(ctx, &mut self.assets, imgui),
            EditorState::CharacterEditor(ref mut editor) => {
                editor.draw(ctx, &mut self.assets, imgui)
            }
            EditorState::BulletInfoEditor(ref mut editor) => {
                editor.draw(ctx, &mut self.assets, imgui)
            }
            EditorState::AttackInfoEditor(ref mut editor) => {
                editor.draw(ctx, &mut self.assets, imgui)
            }
        }?;

        graphics::present(ctx)?;

        Ok(())
    }
}
