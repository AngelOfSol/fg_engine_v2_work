use super::retry_screen::RetryScreen;
use crate::app_state::{AppContext, AppState, Transition};
use crate::game_match::{FromMatchSettings, Match, MatchSettings};
use crate::input::control_scheme::PadControlScheme;
use crate::input::pads_context::{Event, EventType};
use crate::input::InputState;
use crate::player_list::PlayerList;
use crate::typedefs::player::PlayerData;
use ggez::{graphics, Context, GameResult};

type LocalMatch = Match<crate::replay::ReplayWriterFile>;

enum NextState {
    Retry,
}

pub struct LocalVersus {
    next: Option<NextState>,
    inputs: PlayerData<Vec<InputState>>,
    player_list: PlayerList,
    game_state: LocalMatch,
}
impl FromMatchSettings for LocalVersus {
    fn from_settings(
        ctx: &mut Context,
        player_list: PlayerList,
        settings: MatchSettings,
    ) -> GameResult<Box<Self>> {
        assert!(player_list
            .current_players
            .iter()
            .all(|item| item.is_local()));
        assert!(player_list.spectators.is_empty());

        Ok(Box::new(LocalVersus::new(ctx, player_list, settings)?))
    }
}

impl LocalVersus {
    pub fn new(
        ctx: &mut Context,
        player_list: PlayerList,
        settings: MatchSettings,
    ) -> GameResult<Self> {
        Ok(Self {
            next: None,
            inputs: [vec![InputState::default()], vec![InputState::default()]].into(),
            player_list,
            game_state: LocalMatch::new(
                ctx,
                settings,
                crate::replay::create_new_replay_file("local")?,
            )?,
        })
    }
}

impl AppState for LocalVersus {
    fn update(
        &mut self,
        ctx: &mut Context,
        &mut AppContext {
            ref mut pads,
            ref control_schemes,
            ref audio,
            ..
        }: &mut AppContext,
    ) -> GameResult<crate::app_state::Transition> {
        let mut events = Vec::new();
        while let Some(event) = pads.next_event() {
            events.push(event);
        }
        let events = events;

        for (input, player) in self
            .inputs
            .iter_mut()
            .zip(self.player_list.current_players.iter())
        {
            let control_scheme = &control_schemes[&player.gamepad_id().unwrap()];
            let current_frame = input.last_mut().unwrap();
            for event in events.iter() {
                let Event { id, event, .. } = event;
                if *id == control_scheme.gamepad {
                    match event {
                        EventType::ButtonPressed(button) => {
                            control_scheme.handle_press(*button, current_frame);
                        }
                        EventType::ButtonReleased(button) => {
                            control_scheme.handle_release(*button, current_frame);
                        }
                    }
                }
            }
        }
        while ggez::timer::check_update_time(ctx, 60) {
            self.game_state
                .update(self.inputs.as_ref().map(|item| item.as_slice()));
            self.game_state.render_sounds(60, audio)?;
            if self.game_state.game_over().is_some() {
                self.next = Some(NextState::Retry);
            }

            for (input, player) in self
                .inputs
                .iter_mut()
                .zip(self.player_list.current_players.iter())
            {
                let control_scheme = &control_schemes[&player.gamepad_id().unwrap()];
                let mut last_frame = input.last().unwrap().clone();
                control_scheme.update_frame(&mut last_frame);
                input.push(last_frame);
            }
        }

        match std::mem::replace(&mut self.next, None) {
            Some(state) => match state {
                NextState::Retry => Ok(Transition::Replace(Box::new(
                    RetryScreen::<LocalVersus>::new(
                        self.player_list.clone(),
                        self.game_state.settings.clone(),
                    ),
                ))),
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
        for player in self.player_list.gamepads() {
            control_schemes
                .entry(player)
                .or_insert(PadControlScheme::new(player));
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
