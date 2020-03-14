use crate::app_state::{AppContext, AppState, Transition};
use ggez::{graphics, Context, GameResult};
use gilrs::{Button, EventType, GamepadId};
use imgui::im_str;
use laminar::{Packet, Socket, SocketEvent};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{Display, EnumCount, EnumIter};

enum NextState {
    Next,
    Back,
}

#[derive(Debug, Copy, Clone, PartialEq, EnumIter, Display, EnumCount, Serialize, Deserialize)]
pub enum Character {
    Yuyuko,
}
#[derive(Serialize, Deserialize)]
pub enum NetAction {
    ChangeConfirmation(Status),
    ChangeCharacter(Character),
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone, Copy)]
pub enum Status {
    Confirmed,
    None,
    Quit,
}

pub struct NetworkSelect {
    pub socket: Socket,
    pub target: SocketAddr,
    selected: Character,
    confirmed: Status,
}
impl NetworkSelect {
    pub fn new(socket: Socket, target: SocketAddr) -> Self {
        Self {
            socket,
            target,
            selected: Character::iter().next().unwrap(),
            confirmed: Status::None,
        }
    }
    #[allow(dead_code)]
    pub fn selected(&self) -> Character {
        self.selected
    }
}

impl LocalSelect {
    pub fn new(gamepad: GamepadId) -> Self {
        Self {
            gamepad,
            selected: 0,
            confirmed: Status::None,
        }
    }
    #[allow(dead_code)]
    pub fn selected(&self) -> Character {
        Character::iter().nth(self.selected).unwrap()
    }
}

pub struct LocalSelect {
    pub gamepad: GamepadId,
    selected: usize,
    confirmed: Status,
}

impl PlayerType for LocalSelect {
    fn handle_gamepad_input(&mut self, event: gilrs::Event) -> Option<NetAction> {
        if event.id != self.gamepad {
            return None;
        }

        match event.event {
            EventType::ButtonPressed(button, _) => match button {
                Button::DPadUp => {
                    if self.confirmed == Status::None {
                        self.selected = self.selected.checked_sub(1).unwrap_or(0);
                        return Some(NetAction::ChangeCharacter(self.selected_character()));
                    }
                }
                Button::DPadDown => {
                    if self.confirmed == Status::None {
                        self.selected = (self.selected + 1).min(Character::count() - 1);
                        return Some(NetAction::ChangeCharacter(self.selected_character()));
                    }
                }
                Button::East => match self.confirmed {
                    Status::None => {
                        self.confirmed = Status::Quit;
                        return Some(NetAction::ChangeConfirmation(self.confirmed));
                    }
                    Status::Confirmed => {
                        self.confirmed = Status::None;
                        return Some(NetAction::ChangeConfirmation(self.confirmed));
                    }
                    _ => (),
                },
                Button::Start | Button::South => {
                    if self.confirmed == Status::None {
                        self.confirmed = Status::Confirmed;
                        return Some(NetAction::ChangeConfirmation(self.confirmed));
                    }
                }
                _ => (),
            },
            _ => (),
        }
        None
    }
    fn send_net_action(&mut self, _: NetAction) {}
    fn idle(&mut self) {}

    fn confirmed(&self) -> Status {
        self.confirmed
    }
    fn selected_character(&self) -> Character {
        Character::iter().nth(self.selected).unwrap()
    }
}

impl PlayerType for NetworkSelect {
    fn handle_gamepad_input(&mut self, _: gilrs::Event) -> Option<NetAction> {
        None
    }
    fn send_net_action(&mut self, action: NetAction) {
        let _ = self.socket.send(Packet::reliable_ordered(
            self.target,
            bincode::serialize(&action).unwrap(),
            None,
        ));
    }
    fn idle(&mut self) {
        self.socket.manual_poll(std::time::Instant::now());
        while let Some(packet) = self.socket.recv() {
            match packet {
                SocketEvent::Packet(packet) => {
                    let payload: NetAction = match bincode::deserialize(packet.payload()) {
                        Ok(payload) => payload,
                        Err(_) => break,
                    };
                    match payload {
                        NetAction::ChangeCharacter(character) => {
                            self.selected = character;
                        }
                        NetAction::ChangeConfirmation(status) => {
                            self.confirmed = status;
                        }
                    }
                }
                SocketEvent::Timeout(_) => {}
                SocketEvent::Connect(_) => {}
            }
        }
    }

    fn confirmed(&self) -> Status {
        self.confirmed
    }
    fn selected_character(&self) -> Character {
        self.selected
    }
}

pub trait PlayerType {
    fn handle_gamepad_input(&mut self, event: gilrs::Event) -> Option<NetAction>;
    fn send_net_action(&mut self, action: NetAction);
    fn idle(&mut self);
    fn confirmed(&self) -> Status;
    fn selected_character(&self) -> Character;
}

// TODO parameterize this on SelectBy/Local to allow things to be handled easier
// use traits to
pub struct CharacterSelect<P1, P2, Target> {
    next: Option<NextState>,
    p1: Option<P1>,
    p2: Option<P2>,
    _secret: std::marker::PhantomData<Target>,
}

impl<P1, P2, Target> CharacterSelect<P1, P2, Target>
where
    P1: PlayerType,
    P2: PlayerType,
    Target: FromCharacters<P1, P2>,
{
    pub fn new(p1: P1, p2: P2) -> Self {
        Self {
            next: None,
            p1: Some(p1),
            p2: Some(p2),
            _secret: std::marker::PhantomData,
        }
    }
}

pub trait FromCharacters<P1, P2> {
    fn from_characters(ctx: &mut Context, p1: P1, p2: P2) -> GameResult<Box<Self>>;
}

impl<P1, P2, Target: 'static> AppState for CharacterSelect<P1, P2, Target>
where
    P1: PlayerType,
    P2: PlayerType,
    Target: FromCharacters<P1, P2> + AppState + 'static,
{
    fn update(
        &mut self,
        ctx: &mut Context,
        AppContext { ref mut pads, .. }: &mut AppContext,
    ) -> GameResult<crate::app_state::Transition> {
        while let Some(event) = pads.next_event() {
            if let Some(player) = &mut self.p1 {
                let ret = player.handle_gamepad_input(event);
                if ret.is_some() && self.p2.is_some() {
                    self.p2.as_mut().unwrap().send_net_action(ret.unwrap());
                    break;
                }
            }
            if let Some(player) = &mut self.p2 {
                let ret = player.handle_gamepad_input(event);
                if ret.is_some() && self.p1.is_some() {
                    self.p1.as_mut().unwrap().send_net_action(ret.unwrap());
                    break;
                }
            }
        }

        if let Some(player) = &mut self.p1 {
            player.idle();
            if player.confirmed() == Status::Quit {
                self.next = Some(NextState::Back);
            }
        }
        if let Some(player) = &mut self.p2 {
            player.idle();
            if player.confirmed() == Status::Quit {
                self.next = Some(NextState::Back);
            }
        }

        if self
            .p1
            .as_ref()
            .map(|player| player.confirmed() == Status::Confirmed)
            .unwrap_or(false)
            && self
                .p2
                .as_ref()
                .map(|player| player.confirmed() == Status::Confirmed)
                .unwrap_or(false)
        {
            self.next = Some(NextState::Next);
        }

        match std::mem::replace(&mut self.next, None) {
            Some(state) => match state {
                NextState::Next => {
                    let test = Target::from_characters(
                        ctx,
                        self.p1.take().unwrap(),
                        self.p2.take().unwrap(),
                    )?;
                    Ok(Transition::Replace(test))
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
                imgui::Window::new(im_str!("Controllers")).build(ui, || {
                    ui.columns(2, im_str!("col"), true);
                    if let Some(player) = &self.p1 {
                        for character in Character::iter() {
                            let color = if character == player.selected_character() {
                                if player.confirmed() == Status::Confirmed {
                                    [0.0, 1.0, 0.0, 1.0]
                                } else {
                                    [1.0, 0.0, 0.0, 1.0]
                                }
                            } else {
                                [1.0, 1.0, 1.0, 1.0]
                            };
                            ui.text_colored(color, &im_str!("{}", character));
                        }
                    }
                    ui.next_column();
                    if let Some(player) = &self.p2 {
                        for character in Character::iter() {
                            let color = if character == player.selected_character() {
                                if player.confirmed() == Status::Confirmed {
                                    [0.0, 1.0, 0.0, 1.0]
                                } else {
                                    [1.0, 0.0, 0.0, 1.0]
                                }
                            } else {
                                [1.0, 1.0, 1.0, 1.0]
                            };
                            ui.text_colored(color, &im_str!("{}", character));
                        }
                    }
                });
            })
            .render(ctx);

        graphics::present(ctx)?;

        Ok(())
    }
}
