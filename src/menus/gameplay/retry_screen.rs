use crate::app_state::{AppContext, AppState, Transition};
use crate::game_match::FromMatchSettings;
use crate::game_match::MatchSettings;
use crate::player_list::PlayerList;
use fg_controller::backend::ControllerBackend;
use fg_datastructures::player_data::PlayerData;
use fg_ui::menu::{self, Menu};
use ggez::{graphics, Context, GameResult};
use imgui::im_str;
use laminar::{Packet, SocketEvent};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum::{Display, EnumCount, EnumIter};

use super::CharacterSelect;

#[derive(Debug, Copy, Clone, PartialEq, EnumIter, Display, EnumCount, Serialize, Deserialize)]
enum MenuButton {
    Retry,
    CharacterSelect,
    Quit,
}

#[derive(Debug, Copy, Clone, PartialEq, EnumIter, Display, EnumCount, Serialize, Deserialize)]
enum NextState {
    Retry,
    CharacterSelect,
    Quit,
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone, Copy)]
pub enum Status {
    Confirmed,
    None,
    Quit,
}
pub struct RetryScreen<Target> {
    state: PlayerData<Menu<MenuButton>>,
    player_list: PlayerList,
    settings: MatchSettings,
    _secret: std::marker::PhantomData<Target>,
}

impl<Target> RetryScreen<Target> {
    pub fn new(player_list: PlayerList, settings: MatchSettings) -> Self {
        Self {
            state: [
                Menu::new(MenuButton::iter().collect()),
                Menu::new(MenuButton::iter().collect()),
            ]
            .into(),
            player_list,
            settings,
            _secret: std::marker::PhantomData,
        }
    }
}

impl<Target: FromMatchSettings + AppState + 'static> AppState for RetryScreen<Target> {
    fn update(
        &mut self,
        ctx: &mut Context,
        AppContext {
            ref mut controllers,
            ref mut socket,
            ..
        }: &mut AppContext,
    ) -> GameResult<crate::app_state::Transition> {
        for (player, state) in self
            .player_list
            .current_players
            .iter()
            .zip(self.state.iter_mut())
            .filter_map(|(player, state)| player.gamepad_id().map(|id| (id, state)))
        {
            state.update(&controllers.current_state(&player));

            if let Some(ref mut socket) = socket {
                for addr in self.player_list.network_addrs() {
                    let _ = socket.send(Packet::reliable_ordered(
                        addr,
                        bincode::serialize(state.state()).unwrap(),
                        None,
                    ));
                }
            }
        }

        if let Some(socket) = socket {
            socket.manual_poll(std::time::Instant::now());

            while let Some(packet) = socket.recv() {
                match packet {
                    SocketEvent::Packet(packet) => {
                        let payload: menu::MenuState = match bincode::deserialize(packet.payload())
                        {
                            Ok(payload) => payload,
                            Err(_) => continue,
                        };
                        let player = if let Some(idx) = self
                            .player_list
                            .current_players
                            .iter()
                            .position(|item| item == &packet.addr().into())
                        {
                            idx
                        } else {
                            continue;
                        };

                        self.state[player].set_state(payload);
                    }
                    SocketEvent::Timeout(_) => {}
                    SocketEvent::Connect(_) => {}
                }
            }
        }

        if self.state.iter().all(|item| item.confirmed()) {
            if self
                .state
                .iter()
                .any(|item| *item.selected() == MenuButton::Quit)
            {
                Ok(Transition::Pop)
            } else if self
                .state
                .iter()
                .any(|item| *item.selected() == MenuButton::CharacterSelect)
            {
                Ok(Transition::Replace(Box::new(
                    CharacterSelect::<Target>::new(
                        self.player_list.clone(),
                        Some(self.settings.clone()),
                    ),
                )))
            } else if self
                .state
                .iter()
                .any(|item| *item.selected() == MenuButton::Retry)
            {
                let next =
                    Target::from_settings(ctx, self.player_list.clone(), self.settings.clone())?;
                let next =
                    crate::menus::loading_screen::LoadingScreen::new(Transition::Replace(next));

                Ok(Transition::Replace(Box::new(next)))
            } else {
                Ok(Transition::None)
            }
        } else {
            Ok(Transition::None)
        }
    }
    fn on_enter(&mut self, _: &mut Context, _: &mut AppContext) -> GameResult<()> {
        Ok(())
    }
    fn draw(
        &mut self,
        ctx: &mut Context,
        AppContext { ref mut imgui, .. }: &mut AppContext,
    ) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        let frame = imgui.frame();

        frame
            .run(|ui| {
                imgui::Window::new(im_str!("Retry")).build(ui, || {
                    ui.columns(2, im_str!("col"), true);

                    for player in self.state.iter() {
                        for state in MenuButton::iter() {
                            let color = if state == *player.selected() {
                                if player.confirmed() {
                                    [0.0, 1.0, 0.0, 1.0]
                                } else {
                                    [1.0, 0.0, 0.0, 1.0]
                                }
                            } else {
                                [1.0, 1.0, 1.0, 1.0]
                            };
                            ui.text_colored(color, &im_str!("{}", state));
                        }
                        ui.next_column();
                    }
                });
            })
            .render(ctx);

        graphics::present(ctx)?;

        Ok(())
    }
}
