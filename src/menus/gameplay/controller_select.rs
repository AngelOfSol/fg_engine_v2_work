use std::collections::HashMap;

use crate::app_state::{AppContext, AppState, Transition};
use crate::player_list::{PlayerList, PlayerType};
use fg_controller::backend::{Axis, Button, ControllerBackend};
use fg_datastructures::player_data::PlayerData;
use fg_ui::delay::Delay;
use ggez::{graphics, Context, GameResult};
use imgui::im_str;
use sdl_controller_backend::ControllerId;

enum NextState {
    Next,
    Back,
}

pub trait FromControllerList {
    fn from_controllers(data: PlayerList) -> GameResult<Box<Self>>;
}

pub struct ControllerSelect<Target> {
    next: Option<NextState>,
    selectable: PlayerData<bool>,
    selected_gamepad: PlayerData<Option<PlayerType>>,

    reset: HashMap<ControllerId, Delay>,
    _marker: std::marker::PhantomData<Target>,
}

impl<Target> ControllerSelect<Target> {
    pub fn new(selectable: PlayerData<bool>) -> Self {
        Self {
            next: None,
            selectable,
            selected_gamepad: [None, None].into(),
            reset: Default::default(),
            _marker: std::marker::PhantomData,
        }
    }
}

impl<Target: FromControllerList + AppState + 'static> AppState for ControllerSelect<Target> {
    fn update(
        &mut self,
        ctx: &mut Context,
        AppContext {
            ref mut controllers,
            ..
        }: &mut AppContext,
    ) -> GameResult<crate::app_state::Transition> {
        while ggez::timer::check_update_time(ctx, 60) {
            for (id, state) in controllers
                .controllers()
                .map(|id| (id, controllers.current_state(&id)))
            {
                let reset = self.reset.entry(id).or_insert_with(|| Delay::new(20));
                reset.update();

                if state.axis().x() == Axis::Left && reset.is_ready() {
                    if &Some(id.into()) == self.selected_gamepad.p2() {
                        *self.selected_gamepad.p2_mut() = None;
                    } else if self.selected_gamepad.p1().is_none() && *self.selectable.p1() {
                        *self.selected_gamepad.p1_mut() = Some(id.into());
                    }
                } else if state.axis().x() == Axis::Right && reset.is_ready() {
                    if &Some(id.into()) == self.selected_gamepad.p1() {
                        *self.selected_gamepad.p1_mut() = None;
                    } else if self.selected_gamepad.p2().is_none() && *self.selectable.p2() {
                        *self.selected_gamepad.p2_mut() = Some(id.into());
                    }
                } else if state.axis().x() == Axis::Neutral {
                    reset.ready();
                }

                if state[Button::B] {
                    self.next = Some(NextState::Back);
                }

                #[allow(clippy::clippy::blocks_in_if_conditions)]
                if (state[Button::A] || state[Button::Start])
                    && self
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
        }

        match std::mem::replace(&mut self.next, None) {
            Some(state) => match state {
                NextState::Next => Ok(Transition::Replace(Target::from_controllers(
                    PlayerList::new(
                        self.selected_gamepad
                            .iter()
                            .cloned()
                            .map(|item| item.unwrap_or(PlayerType::Dummy))
                            .collect(),
                    ),
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
            ref controllers,
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
                        ui.text(&im_str!("Gamepad {:?}", gamepad.gamepad_id().unwrap()));
                    }
                    ui.next_column();
                    for gamepad in controllers.controllers().filter(|item| {
                        Some((*item).into()) != *self.selected_gamepad.p1()
                            && Some((*item).into()) != *self.selected_gamepad.p2()
                    }) {
                        ui.text(&im_str!("Gamepad {:?}", gamepad));
                    }
                    ui.next_column();
                    if let Some(gamepad) = self.selected_gamepad.p2() {
                        ui.text(&im_str!("Gamepad {:?}", gamepad.gamepad_id().unwrap()));
                    }
                });
            })
            .render(ctx);

        graphics::present(ctx)?;

        Ok(())
    }
}
