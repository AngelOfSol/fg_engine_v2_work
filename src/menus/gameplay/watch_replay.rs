use crate::app_state::{AppContext, AppState, Transition};
use crate::game_match::{MatchSettings, MatchSettingsError, NoLogMatch};
use crate::input::InputState;
use crate::netcode::RollbackableGameState;
use crate::typedefs::player::PlayerData;
use ggez::{graphics, Context, GameResult};

use std::io::Read;

type ReplayMatch = NoLogMatch;

enum NextState {
    Error,
    Back,
}

pub struct WatchReplay<Reader> {
    next: Option<NextState>,
    inputs: PlayerData<Vec<InputState>>,
    previous_states: Vec<<ReplayMatch as RollbackableGameState>::SavedState>,
    reader: Reader,
    game_state: ReplayMatch,
}

impl<Reader: Read> WatchReplay<Reader> {
    pub fn new(ctx: &mut Context, settings: MatchSettings, reader: Reader) -> GameResult<Self> {
        Ok(Self {
            next: None,
            inputs: [vec![], vec![]].into(),
            previous_states: vec![],
            reader,
            game_state: ReplayMatch::new(ctx, settings, ().into())?,
        })
    }

    pub fn read_match_settings(mut reader: Reader) -> Result<MatchSettings, MatchSettingsError> {
        let settings: MatchSettings = bincode::deserialize_from(&mut reader)?;
        settings.validate()?;

        Ok(settings)
    }
}

impl<Reader: Read> AppState for WatchReplay<Reader> {
    fn update(
        &mut self,
        ctx: &mut Context,
        &mut AppContext { ref audio, .. }: &mut AppContext,
    ) -> GameResult<crate::app_state::Transition> {
        let start = chrono::Utc::now();
        let mut frames_ran = 0;
        'game_play: while ggez::timer::check_update_time(ctx, 60) {
            let speed = 1; // this can be changed to do a performance test
            for _ in 0..speed {
                'stream_inputs: loop {
                    let next_frame: i16 = match bincode::deserialize_from(&mut self.reader) {
                        Ok(value) => value,
                        Err(kind) => {
                            match kind.as_ref() {
                                bincode::ErrorKind::Io(err) => match err.kind() {
                                    std::io::ErrorKind::UnexpectedEof => {
                                        self.next = Some(NextState::Back);
                                        break 'stream_inputs;
                                    }
                                    _ => (),
                                },
                                _ => (),
                            }
                            self.next = Some(NextState::Error);
                            break 'game_play;
                        }
                    };
                    let p1_input: InputState = bincode::deserialize_from(&mut self.reader).unwrap();
                    let p2_input: InputState = bincode::deserialize_from(&mut self.reader).unwrap();
                    if next_frame == self.game_state.current_frame() {
                        self.inputs.p1_mut().push(p1_input);
                        self.inputs.p2_mut().push(p2_input);
                        break 'stream_inputs;
                    } else if next_frame < self.game_state.current_frame() {
                        let next_frame = next_frame as usize;
                        self.inputs.p1_mut()[next_frame] = p1_input;
                        self.inputs.p2_mut()[next_frame] = p2_input;
                        let target_frame = self.game_state.current_frame();
                        self.game_state
                            .load_state(self.previous_states[next_frame].clone());
                        while self.game_state.current_frame() < target_frame {
                            self.game_state.update(
                                self.inputs
                                    .as_ref()
                                    .map(|item| &item[..=self.game_state.current_frame() as usize]),
                            );
                            self.previous_states[next_frame] = self.game_state.save_state();
                            frames_ran += 1;
                        }
                    } else {
                        println!("if this number ({}), is greater than or equal to {}, then the replay file was too long", next_frame, i16::MAX);
                        self.next = Some(NextState::Error);
                        break 'game_play;
                    }
                }

                self.previous_states.push(self.game_state.save_state());
                self.game_state
                    .update(self.inputs.as_ref().map(|item| item.as_slice()));
                frames_ran += 1;

                self.game_state.render_sounds(60 * speed, audio)?;
            }
        }
        let end = chrono::Utc::now() - start;
        if frames_ran > 0 {
            println!(
                "total frames {} ran in {}ms",
                frames_ran,
                end.num_milliseconds()
            );
        }

        match std::mem::replace(&mut self.next, None) {
            Some(state) => match state {
                NextState::Error => {
                    dbg!("an error occured during replays.");
                    Ok(Transition::Pop)
                }
                NextState::Back => Ok(Transition::Pop),
            },
            None => Ok(Transition::None),
        }
    }
    fn on_enter(
        &mut self,
        _: &mut Context,
        &mut AppContext { .. }: &mut AppContext,
    ) -> GameResult<()> {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context, AppContext { .. }: &mut AppContext) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        self.game_state.draw(ctx)?;

        graphics::present(ctx)?;

        Ok(())
    }
}
