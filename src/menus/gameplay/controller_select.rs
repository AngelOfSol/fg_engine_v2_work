use crate::app_state::{AppContext, AppState, Transition};
use crate::input::pads_context::{Button, EventType};
use crate::player_list::PlayerType;
use crate::typedefs::player::PlayerData;
use ggez::{graphics, Context, GameResult};
use imgui::im_str;

enum NextState {
    Next,
    Back,
}

pub trait FromControllerList {
    fn from_controllers(data: PlayerData<PlayerType>) -> GameResult<Box<Self>>;
}

pub struct ControllerSelect<Target> {
    next: Option<NextState>,
    selectable: PlayerData<bool>,
    selected_gamepad: PlayerData<Option<PlayerType>>,

    _marker: std::marker::PhantomData<Target>,
}

impl<Target> ControllerSelect<Target> {
    pub fn new(selectable: PlayerData<bool>) -> Self {
        Self {
            next: None,
            selectable,
            selected_gamepad: [None, None].into(),
            _marker: std::marker::PhantomData,
        }
    }
}

impl<Target: FromControllerList + AppState + 'static> AppState for ControllerSelect<Target> {
    fn update(
        &mut self,
        _: &mut Context,
        AppContext { ref mut pads, .. }: &mut AppContext,
    ) -> GameResult<crate::app_state::Transition> {
        while let Some(event) = pads.next_event() {
            match event.event {
                EventType::ButtonPressed(button) => match button {
                    Button::DPadLeft => {
                        if &Some(event.id.into()) == self.selected_gamepad.p2() {
                            *self.selected_gamepad.p2_mut() = None;
                        } else if self.selected_gamepad.p1().is_none() && *self.selectable.p1() {
                            *self.selected_gamepad.p1_mut() = Some(event.id.into());
                        }
                    }
                    Button::DPadRight => {
                        if &Some(event.id.into()) == self.selected_gamepad.p1() {
                            *self.selected_gamepad.p1_mut() = None;
                        } else if self.selected_gamepad.p2().is_none() && *self.selectable.p2() {
                            *self.selected_gamepad.p2_mut() = Some(event.id.into());
                        }
                    }
                    Button::B => {
                        if &Some(event.id.into()) == self.selected_gamepad.p1() {
                            *self.selected_gamepad.p1_mut() = None;
                        } else if &Some(event.id.into()) == self.selected_gamepad.p2() {
                            *self.selected_gamepad.p2_mut() = None;
                        } else {
                            self.next = Some(NextState::Back);
                        }
                    }
                    Button::Start | Button::A => {
                        if self
                            .selectable
                            .iter()
                            .zip(self.selected_gamepad.iter())
                            .fold(true, |acc, (selectable, gamepad)| {
                                *selectable == gamepad.is_some() && acc
                            })
                        {
                            self.next = Some(NextState::Next);
                        }
                    }
                    _ => (),
                },
                _ => (),
            }
        }

        match std::mem::replace(&mut self.next, None) {
            Some(state) => match state {
                NextState::Next => Ok(Transition::Replace(Target::from_controllers(
                    self.selected_gamepad
                        .iter()
                        .cloned()
                        .map(|item| item.unwrap_or(PlayerType::Dummy))
                        .collect(),
                )?)),
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
        AppContext {
            ref mut imgui,
            ref pads,
            ..
        }: &mut AppContext,
    ) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        let frame = imgui.frame();

        frame
            .run(|ui| {
                imgui::Window::new(im_str!("Controllers")).build(ui, || {
                    ui.columns(3, im_str!("Controller##Gamepads"), true);
                    if let Some(gamepad) = self.selected_gamepad.p1() {
                        ui.text(&im_str!("Gamepad {}", gamepad.gamepad_id().unwrap()));
                    }
                    ui.next_column();
                    for gamepad in pads.gamepads().map(|(id, _)| id).filter(|item| {
                        Some((*item).into()) != *self.selected_gamepad.p1()
                            && Some((*item).into()) != *self.selected_gamepad.p2()
                    }) {
                        ui.text(&im_str!("Gamepad {}", gamepad));
                    }
                    ui.next_column();
                    if let Some(gamepad) = self.selected_gamepad.p2() {
                        ui.text(&im_str!("Gamepad {}", gamepad.gamepad_id().unwrap()));
                    }
                });
            })
            .render(ctx);

        graphics::present(ctx)?;

        Ok(())
    }
}
