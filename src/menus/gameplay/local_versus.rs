use super::retry_screen::RetryScreen;
use crate::app_state::{AppContext, AppState, Transition};
use crate::game_match::{FromMatchSettings, Match, MatchSettings};
use crate::player_list::PlayerList;
use fg_controller::backend::ControllerBackend;
use fg_datastructures::player_data::PlayerData;
use fg_input::InputState;
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
            ref mut controllers,
            ref control_schemes,
            ref audio,
            ..
        }: &mut AppContext,
    ) -> GameResult<crate::app_state::Transition> {
        while ggez::timer::check_update_time(ctx, 60) {
            for (input, player) in self.inputs.iter_mut().zip(
                self.player_list
                    .current_players
                    .iter()
                    .map(|player| player.gamepad_id().unwrap()),
            ) {
                let control_scheme = &control_schemes[&player];

                input.push(
                    control_scheme.map(*input.last().unwrap(), &controllers.current_state(&player)),
                );
            }

            self.game_state
                .update(self.inputs.as_ref().map(|item| item.as_slice()));
            self.game_state.render_sounds(60, audio)?;
            if self.game_state.game_over().is_some() {
                self.next = Some(NextState::Retry);
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
            control_schemes.entry(player).or_default();
        }
        Ok(())
    }
    fn draw(
        &mut self,
        ctx: &mut Context,
        AppContext { imgui, .. }: &mut AppContext,
    ) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        self.game_state.draw(ctx)?;

        imgui
            .frame()
            .run(|ui| {
                imgui::Window::new(&imgui::im_str!("Frame Rate"))
                    .no_nav()
                    .build(ui, || {
                        //

                        ui.text(imgui::im_str!("{}", self.inputs.p1().last().unwrap().axis));
                    });
            })
            .render(ctx);

        graphics::present(ctx)?;

        Ok(())
    }
}
