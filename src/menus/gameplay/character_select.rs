use crate::app_state::{AppContext, AppState, Transition};
use crate::input::control_scheme::PadControlScheme;
use crate::typedefs::player::PlayerData;
use ggez::{graphics, Context, GameResult};
use gilrs::{Button, EventType, GamepadId};
use imgui::im_str;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{Display, EnumCount, EnumIter};

enum NextState {
    Next,
    Back,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SelectBy {
    Local(GamepadId),
}

#[derive(Debug, Copy, Clone, PartialEq, EnumIter, Display, EnumCount)]
pub enum Character {
    Yuyuko,
}

type NextStateCreate =
    dyn FnOnce(&mut Context, PlayerData<Character>, PlayerData<PadControlScheme>) -> Transition;

pub struct CharacterSelect {
    next: Option<NextState>,
    select_by: PlayerData<SelectBy>,
    selected_characters_id: PlayerData<usize>,
    confirmed: PlayerData<bool>,
    next_state: Box<NextStateCreate>,
}

impl CharacterSelect {
    pub fn new(select_by: PlayerData<SelectBy>, next_state: Box<NextStateCreate>) -> Self {
        Self {
            next: None,
            select_by,
            selected_characters_id: [0; 2].into(),
            confirmed: [false; 2].into(),
            next_state,
        }
    }
}

impl AppState for CharacterSelect {
    fn update(
        &mut self,
        ctx: &mut Context,
        AppContext {
            ref mut pads,
            ref control_schemes,
            ..
        }: &mut AppContext,
    ) -> GameResult<crate::app_state::Transition> {
        while let Some(event) = pads.next_event() {
            match event.event {
                EventType::ButtonPressed(button, _) => match button {
                    Button::DPadUp => {
                        for player_idx in 0..2 {
                            if SelectBy::Local(event.id) == self.select_by[player_idx]
                                && !self.confirmed[player_idx]
                            {
                                self.selected_characters_id[player_idx] = self
                                    .selected_characters_id[player_idx]
                                    .checked_sub(1)
                                    .unwrap_or(0);
                                break;
                            }
                        }
                    }
                    Button::DPadDown => {
                        for player_idx in 0..2 {
                            if SelectBy::Local(event.id) == self.select_by[player_idx]
                                && !self.confirmed[player_idx]
                            {
                                self.selected_characters_id[player_idx] =
                                    (self.selected_characters_id[player_idx] + 1)
                                        .min(Character::count() - 1);
                                break;
                            }
                        }
                    }
                    Button::East => {
                        if self.select_by.p1() == self.select_by.p2() {
                            if self.confirmed.iter().all(|item| !*item) {
                                self.next = Some(NextState::Back);
                            } else {
                                for player_idx in (0..2).rev() {
                                    if SelectBy::Local(event.id) == self.select_by[player_idx] {
                                        if self.confirmed[player_idx] {
                                            self.confirmed[player_idx] = false;
                                            break;
                                        }
                                    }
                                }
                            }
                        } else {
                            for player_idx in 0..2 {
                                if SelectBy::Local(event.id) == self.select_by[player_idx] {
                                    if self.confirmed[player_idx] {
                                        self.confirmed[player_idx] = false;
                                        break;
                                    } else {
                                        self.next = Some(NextState::Back);
                                    }
                                }
                            }
                        }
                    }
                    Button::Start | Button::South => {
                        for player_idx in 0..2 {
                            if SelectBy::Local(event.id) == self.select_by[player_idx] {
                                if !self.confirmed[player_idx] {
                                    self.confirmed[player_idx] = true;
                                    break;
                                } else if self.confirmed.iter().all(|item| *item) {
                                    self.next = Some(NextState::Next);
                                }
                            }
                        }
                    }
                    _ => (),
                },
                _ => (),
            }
        }

        match std::mem::replace(&mut self.next, None) {
            Some(state) => match state {
                NextState::Next => {
                    let next_state = std::mem::replace(
                        &mut self.next_state,
                        Box::new(|_, _, _| Transition::Pop),
                    );
                    Ok(next_state(
                        ctx,
                        self.selected_characters_id
                            .map(|idx| Character::iter().nth(idx).unwrap()),
                        self.select_by.map(|item| match item {
                            SelectBy::Local(gamepad) => control_schemes
                                .get(&gamepad)
                                .cloned()
                                .unwrap_or(PadControlScheme::new(gamepad)),
                        }),
                    ))
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
                    for (idx, character) in Character::iter().enumerate() {
                        let color = if idx == *self.selected_characters_id.p1() {
                            if *self.confirmed.p1() {
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
                    for (idx, character) in Character::iter().enumerate() {
                        let color = if idx == *self.selected_characters_id.p2() {
                            if *self.confirmed.p2() {
                                [0.0, 1.0, 0.0, 1.0]
                            } else {
                                [1.0, 0.0, 0.0, 1.0]
                            }
                        } else {
                            [1.0, 1.0, 1.0, 1.0]
                        };
                        ui.text_colored(color, &im_str!("{}", character));
                    }
                    ui.columns(1, im_str!("reset"), false);
                    ui.separator();
                    if self.confirmed.iter().all(|item| *item) {
                        ui.text(im_str!("Either player press start to start!"));
                    }
                });
            })
            .render(ctx);

        graphics::present(ctx)?;

        Ok(())
    }
}
