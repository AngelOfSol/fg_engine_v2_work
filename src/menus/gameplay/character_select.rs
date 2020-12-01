use super::controller_select::FromControllerList;
use crate::app_state::{AppContext, AppState, Transition};
use crate::enum_helpers::NextPrev;
use crate::game_match::{FromMatchSettings, MatchSettings};
use crate::input::pads_context::{Button, EventType};
use crate::player_list::PlayerList;
use crate::roster::Character;
use crate::typedefs::player::PlayerData;
use ggez::{graphics, Context, GameResult};
use imgui::im_str;
use laminar::{Packet, SocketEvent};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

enum NextState {
    Next,
    Back,
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone, Copy, Debug)]
pub enum Status {
    Confirmed,
    None,
    Quit,
}

impl<Target> FromControllerList for CharacterSelect<Target> {
    fn from_controllers(data: PlayerList) -> GameResult<Box<Self>> {
        Ok(Box::new(Self::new(data, None)))
    }
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
struct SelectState {
    selected: Character,
    confirmed: Status,
}

pub struct CharacterSelect<Target> {
    next: Option<NextState>,
    player_list: PlayerList,
    settings: MatchSettings,
    chosen_characters: PlayerData<SelectState>,
    _secret: std::marker::PhantomData<Target>,
}

impl<Target> CharacterSelect<Target> {
    pub fn new(player_list: PlayerList, settings: Option<MatchSettings>) -> Self {
        let settings = settings.unwrap_or_else(MatchSettings::new);
        Self {
            player_list,
            chosen_characters: settings.characters.map(|chara| SelectState {
                selected: chara,
                confirmed: Status::None,
            }),
            settings,
            next: None,
            _secret: std::marker::PhantomData,
        }
    }
}

impl<Target> AppState for CharacterSelect<Target>
where
    Target: FromMatchSettings + AppState + 'static,
{
    fn update(
        &mut self,
        ctx: &mut Context,
        AppContext {
            ref mut pads,
            ref mut socket,
            ..
        }: &mut AppContext,
    ) -> GameResult<crate::app_state::Transition> {
        while let Some(event) = pads.next_event() {
            'event: for (idx, _) in self
                .player_list
                .current_players
                .iter()
                .enumerate()
                .filter(|item| item.1 == &event.id.into())
                .chain(
                    self.player_list
                        .current_players
                        .iter()
                        .enumerate()
                        .filter(|item| item.1.is_dummy()),
                )
            {
                let player = &mut self.chosen_characters[idx];

                let mut dirty = false;

                if let EventType::ButtonPressed(button) = event.event {
                    match button {
                        Button::DPadUp => {
                            if player.confirmed == Status::None {
                                player.selected = Character::prev(player.selected);
                                dirty = true;
                            }
                        }
                        Button::DPadDown => {
                            if player.confirmed == Status::None {
                                player.selected = Character::next(player.selected);
                                dirty = true;
                            }
                        }
                        Button::B => match player.confirmed {
                            Status::None => {
                                player.confirmed = Status::Quit;
                                dirty = true;
                            }
                            Status::Confirmed => {
                                player.confirmed = Status::None;
                                dirty = true;
                            }
                            _ => (),
                        },
                        Button::Start | Button::A => {
                            if player.confirmed == Status::None {
                                player.confirmed = Status::Confirmed;
                                dirty = true;
                            }
                        }
                        _ => (),
                    }
                }

                if let Some(ref mut socket) = socket {
                    if dirty {
                        let data = *player;
                        for addr in self.player_list.network_addrs() {
                            let _ = socket.send(Packet::reliable_ordered(
                                addr,
                                bincode::serialize(&(idx, data)).unwrap(),
                                None,
                            ));
                        }
                        break 'event;
                    }
                }
            }
        }

        if let Some(ref mut socket) = socket {
            socket.manual_poll(std::time::Instant::now());
            while let Some(packet) = socket.recv() {
                match packet {
                    SocketEvent::Packet(packet) => {
                        let (player, state): (usize, SelectState) =
                            match bincode::deserialize(packet.payload()) {
                                Ok(payload) => payload,
                                Err(_) => break,
                            };
                        self.chosen_characters[player] = state;

                        // for addr in self
                        //     .player_list
                        //     .spectators
                        //     .iter()
                        //     .filter_map(|item| item.addr())
                        // {
                        //     let _ = socket.send(Packet::reliable_ordered(
                        //         addr,
                        //         packet.payload().iter().copied().collect(),
                        //         None,
                        //     ));
                        // }
                    }
                    SocketEvent::Timeout(_) => {}
                    SocketEvent::Connect(_) => {}
                }
            }
        }

        if self
            .chosen_characters
            .iter()
            .any(|state| state.confirmed == Status::Quit)
        {
            self.next = Some(NextState::Back);
        }

        if self
            .chosen_characters
            .iter()
            .all(|state| state.confirmed == Status::Confirmed)
        {
            self.next = Some(NextState::Next);
        }

        match std::mem::replace(&mut self.next, None) {
            Some(state) => match state {
                NextState::Next => {
                    let mut settings = self.settings.clone();
                    settings.characters = self.chosen_characters.map(|item| item.selected);
                    settings.load(ctx)?;

                    let next = Target::from_settings(ctx, self.player_list.clone(), settings)?;

                    let next =
                        crate::menus::loading_screen::LoadingScreen::new(Transition::Replace(next));

                    Ok(Transition::Replace(Box::new(next)))
                }
                NextState::Back => Ok(Transition::Pop),
            },
            None => Ok(Transition::None),
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
                imgui::Window::new(im_str!("Characters")).build(ui, || {
                    ui.columns(2, im_str!("col"), true);

                    for player in self.chosen_characters.iter() {
                        for character in Character::iter() {
                            let color = if character == player.selected {
                                if player.confirmed == Status::Confirmed {
                                    [0.0, 1.0, 0.0, 1.0]
                                } else {
                                    [1.0, 0.0, 0.0, 1.0]
                                }
                            } else {
                                [1.0, 1.0, 1.0, 1.0]
                            };
                            ui.text_colored(color, &im_str!("{}", character));
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
