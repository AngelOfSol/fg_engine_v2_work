use super::controller_select::FromControllerList;
use crate::game_match::{FromMatchSettings, MatchSettings};
use crate::player_list::PlayerList;
use crate::roster::RosterCharacter;
use crate::{
    app_state::{AppContext, AppState, Transition},
    player_list::PlayerType,
};
use fg_controller::backend::ControllerBackend;
use fg_datastructures::player_data::PlayerData;
use fg_ui::{
    delay::Delay,
    menu::{Menu, MenuAction, MenuState},
};
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
pub enum CharacterPacket {
    Update(MenuState),
    Quit,
}

impl<Target> FromControllerList for CharacterSelect<Target> {
    fn from_controllers(data: PlayerList) -> GameResult<Box<Self>> {
        Ok(Box::new(Self::new(data, None)))
    }
}

pub struct CharacterSelect<Target> {
    next: Option<NextState>,
    player_list: PlayerList,
    settings: MatchSettings,
    chosen_characters: PlayerData<Menu<RosterCharacter>>,
    delay: Delay,
    startup_delay: Delay,
    _secret: std::marker::PhantomData<Target>,
}

impl<Target> CharacterSelect<Target> {
    pub fn new(player_list: PlayerList, settings: Option<MatchSettings>) -> Self {
        let settings = settings.unwrap_or_else(MatchSettings::new);
        Self {
            player_list,
            chosen_characters: settings
                .characters
                .map(|chara| Menu::with_selected(RosterCharacter::iter().collect(), chara)),
            settings,
            next: None,
            delay: Delay::delay(20),
            startup_delay: Delay::delay(8),
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
            ref mut controllers,
            ref mut socket,
            ..
        }: &mut AppContext,
    ) -> GameResult<crate::app_state::Transition> {
        while ggez::timer::check_update_time(ctx, 60) {
            if self.startup_delay.update() {
                self.delay.update();
                for (player, state) in self
                    .player_list
                    .current_players
                    .iter()
                    .zip(self.chosen_characters.iter_mut())
                    .filter_map(|(player, state)| player.gamepad_id().map(|id| (id, state)))
                {
                    let action = state.update(&controllers.current_state(&player));
                    if action == MenuAction::Back {
                        self.next = Some(NextState::Back);

                        if let Some(ref mut socket) = socket {
                            for addr in self.player_list.network_addrs() {
                                let _ = socket.send(Packet::reliable_ordered(
                                    addr,
                                    bincode::serialize(&CharacterPacket::Quit).unwrap(),
                                    None,
                                ));
                            }
                        }
                    }

                    if !matches!(action, MenuAction::Confirm | MenuAction::None) {
                        self.delay.unready();
                    }

                    if let Some(ref mut socket) = socket {
                        for addr in self.player_list.network_addrs() {
                            let _ = socket.send(Packet::reliable_ordered(
                                addr,
                                bincode::serialize(&CharacterPacket::Update(*state.state()))
                                    .unwrap(),
                                None,
                            ));
                        }
                    }
                }

                if let (PlayerType::LocalGamepad(player), PlayerType::Dummy) = (
                    self.player_list.current_players.p1(),
                    self.player_list.current_players.p2(),
                ) {
                    if self.chosen_characters.p1().confirmed() && self.delay.is_ready() {
                        let state = self.chosen_characters.p2_mut();
                        state.update(&controllers.current_state(&player));
                    }
                }
            }
        }

        if let Some(ref mut socket) = socket {
            socket.manual_poll(std::time::Instant::now());
            while let Some(packet) = socket.recv() {
                match packet {
                    SocketEvent::Packet(packet) => {
                        let data: CharacterPacket = match bincode::deserialize(packet.payload()) {
                            Ok(payload) => payload,
                            Err(_) => break,
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

                        match data {
                            CharacterPacket::Update(state) => {
                                self.chosen_characters[player].set_state(state);
                            }
                            CharacterPacket::Quit => {
                                self.next = Some(NextState::Back);
                            }
                        }
                    }
                    SocketEvent::Timeout(_) => {}
                    SocketEvent::Connect(_) => {}
                }
            }
        }

        if self.chosen_characters.iter().all(|state| state.confirmed()) {
            self.next = Some(NextState::Next);
        }

        match std::mem::replace(&mut self.next, None) {
            Some(state) => match state {
                NextState::Next => {
                    let mut settings = self.settings.clone();
                    settings.characters =
                        self.chosen_characters.as_ref().map(|item| *item.selected());
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
                        for character in RosterCharacter::iter() {
                            let color = if character == *player.selected() {
                                if player.confirmed() {
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
