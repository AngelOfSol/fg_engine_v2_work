use ggez::{Context, GameResult};

use crate::button_check::ButtonCheck;
use crate::editor::GameEditor;
use crate::game_match::Match;

pub trait AppState {
    fn next_appstate(&mut self, ctx: &mut Context) -> Option<RunnerState>;
}

#[allow(clippy::large_enum_variant)]
pub enum RunnerState {
    Editor(GameEditor),
    Match(Match),
    ButtonCheck(ButtonCheck),
}

pub struct Runner {
    state: RunnerState,
}

impl Runner {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let state = {
            let mut state = None;
            for arg in std::env::args() {
                if arg == "--editor" {
                    state = Some(RunnerState::Editor(GameEditor::new(ctx).unwrap()));
                }
            }
            state.unwrap_or_else(|| RunnerState::ButtonCheck(ButtonCheck::new(ctx).unwrap()))
            //state.unwrap_or_else(|| RunnerState::Match(Match::new(ctx).unwrap()))
        };
        Ok(Runner { state })
    }
    pub fn run(&mut self, ctx: &mut Context, event_loop: &mut winit::EventsLoop) {
        loop {
            let next = match &mut self.state {
                RunnerState::Editor(ref mut editor) => {
                    println!("{:?}", ggez::event::run(ctx, event_loop, editor));
                    editor.next_appstate(ctx)
                }
                RunnerState::Match(ref mut game_match) => {
                    println!("{:?}", ggez::event::run(ctx, event_loop, game_match));
                    None
                }
                RunnerState::ButtonCheck(ref mut button_check) => {
                    println!("{:?}", ggez::event::run(ctx, event_loop, button_check));
                    button_check.next_appstate(ctx)
                }
            };
            if let Some(new_state) = next {
                self.state = new_state;
                ctx.continuing = true;
            } else {
                break;
            }
        }
    }
}
