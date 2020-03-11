use crate::app_state::{AppContext, AppState, Transition};
use crate::game_match::Match;
use crate::input::control_scheme::PadControlScheme;
use crate::input::InputBuffer;
use crate::typedefs::player::PlayerData;
use ggez::{graphics, Context, GameResult};
use gilrs::{Button, Event, EventType, GamepadId};
use imgui::im_str;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{Display, EnumCount, EnumIter};

enum NextState {
    Back,
}

pub struct TrainingMode {
    next: Option<NextState>,
    inputs: PlayerData<InputBuffer>,
    controls: PlayerData<PadControlScheme>,
    game_state: Match,
}

impl TrainingMode {
    pub fn new(ctx: &mut Context, controls: PlayerData<PadControlScheme>) -> GameResult<Self> {
        Ok(Self {
            next: None,
            inputs: [InputBuffer::new(), InputBuffer::new()].into(),
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
            let mut current_frame = input.top().clone();
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
            *input.top_mut() = current_frame;
            //input.push(current_frame);
        }
        while ggez::timer::check_update_time(ctx, 60) {
            self.game_state.update(&self.inputs)?;
            for (input, control_scheme) in self.inputs.iter_mut().zip(self.controls.iter()) {
                input.push(input.top().clone());
                *input.top_mut() = control_scheme.update_frame(*input.top());
            }
        }

        match std::mem::replace(&mut self.next, None) {
            Some(state) => match state {
                NextState::Back => Ok(Transition::Pop),
            },
            None => Ok(Transition::None),
        }
    }
    fn on_enter(&mut self, ctx: &mut Context, _: &mut AppContext) -> GameResult<()> {
        //crate::graphics::prepare_screen_for_game(ctx)
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context, AppContext { .. }: &mut AppContext) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        self.game_state.draw(ctx)?;

        graphics::present(ctx)?;

        Ok(())
    }
}
