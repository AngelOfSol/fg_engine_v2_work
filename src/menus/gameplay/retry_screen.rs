use crate::app_state::{AppContext, AppState, Transition};
use crate::enum_helpers::NextPrev;
use crate::game_match::FromMatchSettings;
use crate::game_match::MatchSettings;
use crate::input::pads_context::{Button, EventType};
use crate::player_list::PlayerList;
use crate::typedefs::player::PlayerData;
use ggez::{graphics, Context, GameResult};
use imgui::im_str;
use laminar::{Packet, SocketEvent};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumCount, EnumIter};

#[derive(Debug, Copy, Clone)]
struct PlayerMenuState {
    selected_button: MenuButton,
    confirmed: bool,
}

impl PlayerMenuState {
    fn new() -> Self {
        Self {
            selected_button: MenuButton::Retry,
            confirmed: false,
        }
    }
    fn process_input(&mut self, input: PlayerInput) {
        match input {
            PlayerInput::None => {}
            PlayerInput::Up => {
                if !self.confirmed {
                    self.selected_button = MenuButton::prev(self.selected_button);
                }
            }
            PlayerInput::Down => {
                if !self.confirmed {
                    self.selected_button = MenuButton::next(self.selected_button);
                }
            }
            PlayerInput::Confirm => self.confirmed = true,
            PlayerInput::Back => self.confirmed = false,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, EnumIter, Display, EnumCount, Serialize, Deserialize)]
enum MenuButton {
    Retry,
    CharacterSelect,
    Quit,
}

#[derive(Debug, Copy, Clone, PartialEq, EnumIter, Display, EnumCount, Serialize, Deserialize)]
enum PlayerInput {
    None,
    Up,
    Down,
    Confirm,
    Back,
}

impl Default for PlayerInput {
    fn default() -> Self {
        Self::None
    }
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
    state: PlayerData<PlayerMenuState>,
    player_list: PlayerList,
    settings: MatchSettings,
    delay: usize,
    _secret: std::marker::PhantomData<Target>,
}

impl<Target> RetryScreen<Target> {
    pub fn new(player_list: PlayerList, settings: MatchSettings) -> Self {
        Self {
            state: [PlayerMenuState::new(), PlayerMenuState::new()].into(),
            player_list,
            settings,
            delay: 60,
            _secret: std::marker::PhantomData,
        }
    }
}

impl<Target: FromMatchSettings + AppState + 'static> AppState for RetryScreen<Target> {
    fn update(
        &mut self,
        ctx: &mut Context,
        AppContext {
            ref mut pads,
            ref mut socket,
            ..
        }: &mut AppContext,
    ) -> GameResult<crate::app_state::Transition> {
        'pads: while let Some(event) = pads.next_event() {
            let player = if let Some(player) = self
                .player_list
                .current_players
                .iter()
                .position(|item| item == &event.id.into())
            {
                player
            } else {
                continue 'pads;
            };
            let input = match event.event {
                EventType::ButtonPressed(button) => match button {
                    Button::DPadUp => PlayerInput::Up,
                    Button::DPadDown => PlayerInput::Down,
                    Button::A => PlayerInput::Confirm,
                    Button::B => PlayerInput::Back,
                    _ => continue 'pads,
                },
                _ => continue 'pads,
            };

            if self.delay == 0 {
                self.state[player].process_input(input);

                if let Some(ref mut socket) = socket {
                    for addr in self.player_list.network_addrs() {
                        let _ = socket.send(Packet::reliable_ordered(
                            addr,
                            bincode::serialize(&input).unwrap(),
                            None,
                        ));
                    }
                }
            }
        }

        while ggez::timer::check_update_time(ctx, 60) {
            self.delay = self.delay.saturating_sub(1);
        }

        if let Some(ref mut socket) = socket {
            socket.manual_poll(std::time::Instant::now());

            while let Some(packet) = socket.recv() {
                match packet {
                    SocketEvent::Packet(packet) => {
                        let payload: PlayerInput = match bincode::deserialize(packet.payload()) {
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

                        if self.delay == 0 {
                            self.state[player].process_input(payload);
                        }
                    }
                    SocketEvent::Timeout(_) => {}
                    SocketEvent::Connect(_) => {}
                }
            }
        }

        if self.state.iter().all(|item| item.confirmed) {
            if self
                .state
                .iter()
                .any(|item| item.selected_button == MenuButton::Quit)
            {
                Ok(Transition::Pop)
            } else if self
                .state
                .iter()
                .any(|item| item.selected_button == MenuButton::CharacterSelect)
            {
                // TODO TBD rework of character_select to use a different method
                Ok(Transition::Pop)
            } else if self
                .state
                .iter()
                .any(|item| item.selected_button == MenuButton::Retry)
            {
                let next =
                    Target::from_settings(ctx, self.player_list.clone(), self.settings.clone())?;
                let next = crate::menus::loading_screen::LoadingScreen::new(
                    "".to_owned(),
                    Transition::Replace(next),
                );

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
                            let color = if state == player.selected_button {
                                if player.confirmed {
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
