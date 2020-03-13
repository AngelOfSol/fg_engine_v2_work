use crate::app_state::{AppContext, AppState, Transition};
use crate::game_match::Match;
use crate::input::control_scheme::PadControlScheme;
use crate::input::InputState;
use crate::typedefs::player::PlayerData;
use ggez::{graphics, Context, GameResult};
use gilrs::{Event, EventType};

enum NextState {
    Back,
}

pub struct TrainingMode {
    next: Option<NextState>,
    inputs: PlayerData<Vec<InputState>>,
    controls: PlayerData<PadControlScheme>,
    game_state: Match,
}

impl TrainingMode {
    pub fn new(ctx: &mut Context, controls: PlayerData<PadControlScheme>) -> GameResult<Self> {
        Ok(Self {
            next: None,
            inputs: [vec![InputState::default()], vec![InputState::default()]].into(),
            controls,
            game_state: Match::new(ctx)?,
        })
    }
}

impl AppState for TrainingMode {
    fn update(
        &mut self,
        ctx: &mut Context,
        &mut AppContext { ref mut pads, .. }: &mut AppContext,
    ) -> GameResult<crate::app_state::Transition> {
        let mut events = Vec::new();
        while let Some(event) = pads.next_event() {
            events.push(event);
        }
        let events = events;

        // only iterates over the first player
        for (input, control_scheme) in self.inputs.iter_mut().zip(self.controls.iter()).take(1) {
            let mut current_frame = input.last().unwrap().clone();
            for event in events.iter() {
                let Event { id, event, .. } = event;
                if *id == control_scheme.gamepad {
                    match event {
                        EventType::ButtonPressed(button, _) => {
                            current_frame = control_scheme.handle_press(*button, current_frame);
                        }
                        EventType::ButtonReleased(button, _) => {
                            current_frame = control_scheme.handle_release(*button, current_frame);
                        }
                        _ => (),
                    }
                }
            }
            *input.last_mut().unwrap() = current_frame;
        }
        while ggez::timer::check_update_time(ctx, 60) {
            self.game_state
                .update(self.inputs.as_ref().map(|item| item.as_slice()))?;
            for (input, control_scheme) in self.inputs.iter_mut().zip(self.controls.iter()) {
                input.push(input.last().unwrap().clone());
                *input.last_mut().unwrap() = control_scheme.update_frame(*input.last().unwrap());
            }
        }

        match std::mem::replace(&mut self.next, None) {
            Some(state) => match state {
                NextState::Back => Ok(Transition::Pop),
            },
            None => Ok(Transition::None),
        }
    }
    fn on_enter(&mut self, _: &mut Context, _: &mut AppContext) -> GameResult<()> {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context, AppContext { .. }: &mut AppContext) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        self.game_state.draw(ctx)?;

        graphics::present(ctx)?;

        Ok(())
    }
}
