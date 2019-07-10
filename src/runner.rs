use ggez::{Context, GameResult};

use crate::editor::GameEditor;
use crate::game_match::Match;

pub trait AppState {
    fn next_appstate(&self) -> Option<RunnerState>;
}

#[allow(clippy::large_enum_variant)]
pub enum RunnerState {
    Editor(GameEditor),
    Match(Match),
}

pub struct Runner {
    state: RunnerState,
}

impl Runner {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let state = {
            let mut state = RunnerState::Match(Match::new(ctx)?);
            for arg in std::env::args() {
                if arg == "--editor" {
                    state = RunnerState::Editor(GameEditor::new(ctx)?);
                }
            }
            state
        };
        Ok(Runner { state })
    }
    pub fn run(&mut self, ctx: &mut Context, event_loop: &mut winit::EventsLoop) {
        loop {
            let next = match &mut self.state {
                RunnerState::Editor(ref mut editor) => {
                    println!("{:?}", ggez::event::run(ctx, event_loop, editor));
                    editor.next_appstate()
                }
                RunnerState::Match(ref mut game_match) => {
                    println!("{:?}", ggez::event::run(ctx, event_loop, game_match));
                    None
                }
            };
            if let Some(new_state) = next {
                self.state = new_state;
            } else {
                break;
            }
        }
    }
}
