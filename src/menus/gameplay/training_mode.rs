use super::{FromCharacters, LocalSelect};
use crate::app_state::{AppContext, AppState, Transition};
use crate::game_match::Match;
use crate::input::control_scheme::PadControlScheme;
use crate::input::InputState;
use crate::typedefs::player::PlayerData;

use ggez::{graphics, Context, GameResult};
use gilrs::{Event, EventType, GamepadId};

enum NextState {
    Back,
}

pub struct TrainingMode {
    next: Option<NextState>,
    inputs: PlayerData<Vec<InputState>>,
    players: PlayerData<GamepadId>,
    game_state: Match,
}

impl FromCharacters<LocalSelect, LocalSelect> for TrainingMode {
    fn from_characters(
        ctx: &mut Context,
        p1: LocalSelect,
        p2: LocalSelect,
    ) -> GameResult<Box<Self>> {
        Ok(Box::new(TrainingMode::new(
            ctx,
            [p1.gamepad, p2.gamepad].into(),
        )?))
    }
}

impl TrainingMode {
    pub fn new(ctx: &mut Context, players: PlayerData<GamepadId>) -> GameResult<Self> {
        Ok(Self {
            next: None,
            inputs: [vec![InputState::default()], vec![InputState::default()]].into(),
            players,
            game_state: Match::new(ctx)?,
        })
    }
}

impl AppState for TrainingMode {
    fn update(
        &mut self,
        ctx: &mut Context,
        &mut AppContext {
            ref mut pads,
            ref control_schemes,
            ..
        }: &mut AppContext,
    ) -> GameResult<crate::app_state::Transition> {
        let mut events = Vec::new();
        while let Some(event) = pads.next_event() {
            events.push(event);
        }
        let events = events;

        // only iterates over the first player
        for (input, player) in self.inputs.iter_mut().zip(self.players.iter()).take(1) {
            let current_frame = input.last_mut().unwrap();
            let control_scheme = &control_schemes[player];
            for event in events.iter() {
                let Event { id, event, .. } = event;
                if *id == control_scheme.gamepad {
                    match event {
                        EventType::ButtonPressed(button, _) => {
                            control_scheme.handle_press(*button, current_frame);
                        }
                        EventType::ButtonReleased(button, _) => {
                            control_scheme.handle_release(*button, current_frame);
                        }
                        _ => (),
                    }
                }
            }
        }
        while ggez::timer::check_update_time(ctx, 60) {
            self.game_state
                .update(self.inputs.as_ref().map(|item| item.as_slice()));
            for (input, player) in self.inputs.iter_mut().zip(self.players.iter()) {
                let control_scheme = &control_schemes[player];
                let mut last_frame = input.last().unwrap().clone();
                control_scheme.update_frame(&mut last_frame);
                input.push(last_frame);
            }
        }

        match std::mem::replace(&mut self.next, None) {
            Some(state) => match state {
                NextState::Back => Ok(Transition::Pop),
            },
            None => Ok(Transition::None),
        }
    }
    fn on_enter(
        &mut self,
        _: &mut Context,
        &mut AppContext {
            ref mut control_schemes,
            ..
        }: &mut AppContext,
    ) -> GameResult<()> {
        for player in self.players.iter() {
            control_schemes
                .entry(*player)
                .or_insert(PadControlScheme::new(*player));
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context, AppContext { .. }: &mut AppContext) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        self.game_state.draw(ctx)?;

        graphics::present(ctx)?;

        Ok(())
    }
}
