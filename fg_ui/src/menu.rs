use crate::delay::Delay;
use fg_controller::backend::{Axis, Button, ControllerState};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MenuAction {
    Confirm,
    Select,
    Deselect,
    Back,
    None,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize,
)]
pub struct MenuState {
    selected: usize,
    confirmed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Menu<T> {
    items: Vec<T>,

    state: MenuState,

    back: Delay,

    confirm: Delay,

    axis: Delay,
}

const BUTTON_DELAY: usize = 20;

impl<T> Menu<T> {
    pub fn new(items: Vec<T>) -> Self {
        assert!(!items.is_empty());
        Self {
            items,
            state: MenuState::default(),
            back: Delay::delay(30),
            confirm: Delay::delay(BUTTON_DELAY),
            axis: Delay::delay(BUTTON_DELAY),
        }
    }
    pub fn with_selected(items: Vec<T>, item: T) -> Self
    where
        T: Eq,
    {
        assert!(!items.is_empty());
        Self {
            state: MenuState {
                selected: items.iter().position(|x| x == &item).unwrap(),
                confirmed: false,
            },
            items,
            back: Delay::delay(30),
            confirm: Delay::new(BUTTON_DELAY),
            axis: Delay::new(BUTTON_DELAY),
        }
    }

    pub fn state(&self) -> &MenuState {
        &self.state
    }

    pub fn set_state(&mut self, state: MenuState) {
        self.state = state;
    }

    pub fn update(&mut self, input: &ControllerState) -> MenuAction {
        if self.back.update() && (input[Button::B] || input[Button::Back]) {
            self.back.unready();
            if self.state.confirmed {
                self.state.confirmed = false;
                return MenuAction::Deselect;
            } else {
                return MenuAction::Back;
            }
        }

        if self.confirm.update() && (input[Button::A] || input[Button::Start]) {
            self.confirm.unready();
            if self.state.confirmed {
                return MenuAction::Confirm;
            } else {
                self.state.confirmed = true;
                return MenuAction::Select;
            }
        }
        let can_move = self.axis.update() && !self.state.confirmed;
        if input.axis().y() == Axis::Up {
            if can_move {
                self.axis.unready();
                self.state.selected -= 1;
                self.state.selected += self.items.len();
                self.state.selected %= self.items.len();
            }
        } else if input.axis().y() == Axis::Down {
            if can_move {
                self.axis.unready();
                self.state.selected += 1;
                self.state.selected %= self.items.len();
            }
        } else {
            self.axis.ready();
        }

        MenuAction::None
    }

    pub fn selected(&self) -> &T {
        &self.items[self.state.selected]
    }
    pub fn selected_mut(&mut self) -> &mut T {
        &mut self.items[self.state.selected]
    }

    pub fn items(&self) -> impl Iterator<Item = &T> {
        self.items.iter()
    }
    pub fn items_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.items.iter_mut()
    }

    pub fn confirmed(&self) -> bool {
        self.state.confirmed
    }

    pub fn modify_items<F>(&mut self, f: F)
    where
        F: FnOnce(&mut Vec<T>),
    {
        f(&mut self.items);
        self.state.selected = (self.items.len() - 1).min(self.state.selected);
    }
}
