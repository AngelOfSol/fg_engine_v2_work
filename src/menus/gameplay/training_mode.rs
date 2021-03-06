use crate::app_state::{AppContext, AppState, Transition};
use crate::game_match::{FromMatchSettings, Match, MatchSettings};
use crate::player_list::PlayerList;
use fg_controller::backend::ControllerBackend;
use fg_datastructures::player_data::PlayerData;
use fg_input::InputState;
use ggez::{graphics, Context, GameResult};
use inspect_design::traits::*;

type TrainingMatch = Match<crate::replay::ReplayWriterFile>;

enum NextState {
    Back,
}

pub struct TrainingMode {
    next: Option<NextState>,
    inputs: PlayerData<Vec<InputState>>,
    player_list: PlayerList,
    game_state: TrainingMatch,
    dirty: bool,
    // inspect_state: <crate::roster::yuyuko::Yuyuko as Inspect>::State,
    fps: u32,
}

impl FromMatchSettings for TrainingMode {
    fn from_settings(
        ctx: &mut Context,
        player_list: PlayerList,
        settings: MatchSettings,
    ) -> GameResult<Box<Self>> {
        Ok(Box::new(TrainingMode::new(ctx, player_list, settings)?))
    }
}

impl TrainingMode {
    pub fn new(
        ctx: &mut Context,
        player_list: PlayerList,
        settings: MatchSettings,
    ) -> GameResult<Self> {
        Ok(Self {
            next: None,
            inputs: [vec![InputState::default()], vec![InputState::default()]].into(),
            player_list,
            game_state: TrainingMatch::new(
                ctx,
                settings,
                crate::replay::create_new_replay_file("training")?,
            )?,
            dirty: true,
            // inspect_state: Default::default(),
            fps: 60,
        })
    }
}

impl AppState for TrainingMode {
    fn update(
        &mut self,
        ctx: &mut Context,
        &mut AppContext {
            ref audio,
            ref mut controllers,
            ref control_schemes,
            ..
        }: &mut AppContext,
    ) -> GameResult<crate::app_state::Transition> {
        let mut count = 0;
        while ggez::timer::check_update_time(ctx, self.fps) {
            for (input, player) in self
                .inputs
                .iter_mut()
                .zip(
                    self.player_list
                        .current_players
                        .iter()
                        .map(|player| player.gamepad_id().unwrap()),
                )
                .take(1)
            {
                let control_scheme = &control_schemes[&player];

                input.push(
                    control_scheme.map(*input.last().unwrap(), &controllers.current_state(&player)),
                );
            }

            count += 1;
            self.game_state
                .update(self.inputs.as_ref().map(|item| item.as_slice()));
            self.game_state.render_sounds(60, audio)?;

            if self.game_state.game_over().is_some() {
                self.next = Some(NextState::Back);
            }

            self.dirty = true;
        }
        if count > 1 {
            dbg!(count);
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
        if self.dirty {
            graphics::clear(ctx, graphics::BLACK);
            self.game_state.draw(ctx)?;

            self.dirty = false;

            // let inspect_state = &mut self.inspect_state;
            let fps = &mut self.fps;
            match self.game_state.players.p1_mut() {
                crate::roster::CharacterBehavior::YuyukoPlayer(value) => {
                    imgui
                        .frame()
                        .run(|ui| {
                            imgui::Window::new(&imgui::im_str!("Editor"))
                                .no_nav()
                                .build(ui, || {
                                    //  value.data.inspect_mut("yuyu", inspect_state, ui);
                                });
                            imgui::Window::new(&imgui::im_str!("Frame Rate"))
                                .no_nav()
                                .build(ui, || {
                                    fps.inspect_mut("fps", &mut (), ui);
                                    value.state.current_state.inspect(
                                        "state",
                                        &mut Default::default(),
                                        ui,
                                    );
                                });
                        })
                        .render(ctx);
                }
            }

            graphics::present(ctx)?;
        }

        Ok(())
    }
}
