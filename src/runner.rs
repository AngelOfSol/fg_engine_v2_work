use ggez::{Context, GameResult};

use crate::editor::GameEditor;

pub trait AppState {
    fn next_appstate(&self) -> Option<RunnerState>;
}

pub enum RunnerState {
    Editor(GameEditor),
}

pub struct Runner {
    state: RunnerState,
}

impl Runner {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(Runner {
            state: RunnerState::Editor(GameEditor::new(ctx)?),
        })
    }
    pub fn run(&mut self, ctx: &mut Context, event_loop: &mut winit::EventsLoop) {
        loop {
            let next = match &mut self.state {
                RunnerState::Editor(ref mut editor) => {
                    ggez::event::run(ctx, event_loop, editor);
                    editor.next_appstate()
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
