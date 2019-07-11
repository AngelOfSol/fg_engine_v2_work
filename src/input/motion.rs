use super::ringbuffer::DirectionIter;
use super::{Axis, Button, ButtonState, InputBuffer, MOTION_DIRECTION_SIZE};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Direction {
    Forward,
    Backward,
}

impl Direction {
    fn invert(self) -> Self {
        match self {
            Direction::Forward => Direction::Backward,
            Direction::Backward => Direction::Forward,
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Standing {
    Standing,
    Crouching,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Input {
    Idle(Standing),
    Walking(Direction),
    Dashing(Direction),
    Normal(Standing, Button),
    CommandNormal(Standing, Direction, Button),
    QuarterCircle(Direction, Button),
    DragonPunch(Direction, Button),
}

impl Input {
    fn invert(mut self) -> Self {
        match self {
            Input::Walking(dir) => Input::Walking(dir.invert()),
            Input::Dashing(dir) => Input::Dashing(dir.invert()),
            Input::CommandNormal(standing, dir, button) => Input::CommandNormal(standing, dir.invert(), button),
            Input::QuarterCircle(dir, button) => Input::QuarterCircle(dir.invert(), button),
            Input::DragonPunch(dir, button) => Input::DragonPunch(dir.invert(), button),
            rest => rest,
        }
    }
}

pub fn read_inputs(buffer: &InputBuffer, facing_right: bool) -> Vec<Input> {
    [
        read_dragon_punch(buffer),
        read_quarter_circle(buffer),
        read_dash_macro(buffer),
        read_command_normal(buffer),
        read_normal(buffer),
        read_dashing(buffer),
        read_walk(buffer),
        read_idle(buffer),
    ]
    .iter()
    .filter(|item| item.is_some())
    .map(|item| item.unwrap())
    .map(|item| if facing_right { item } else { item.invert() })
    .collect()
}

fn read_idle(buffer: &InputBuffer) -> Option<Input> {
    match buffer.top().axis {
        Axis::DownLeft | Axis::Down | Axis::DownRight => Some(Input::Idle(Standing::Crouching)),
        _ => Some(Input::Idle(Standing::Standing)),
    }
}

fn read_walk(buffer: &InputBuffer) -> Option<Input> {
    match buffer.top().axis {
        Axis::Left => Some(Input::Walking(Direction::Backward)),
        Axis::Right => Some(Input::Walking(Direction::Forward)),
        _ => None,
    }
}

fn read_dash_macro(buffer: &InputBuffer) -> Option<Input> {
    let top = buffer.top();
    if top.axis.is_horizontal()
        && top[Button::A] == ButtonState::JustPressed
        && top[Button::B] == ButtonState::JustPressed
    {
        Some(Input::Dashing(top.axis.get_direction().unwrap()))
    } else {
        None
    }
}

fn read_dashing(buffer: &InputBuffer) -> Option<Input> {
    let inputs = buffer.iter().into_direction_iter().collect::<Vec<_>>();
    let (time, axis) = inputs[0];
    if time > MOTION_DIRECTION_SIZE || !axis.is_horizontal() {
        return None;
    }
    let (time, should_be_neutral) = inputs[1];
    if time > MOTION_DIRECTION_SIZE || should_be_neutral != Axis::Neutral {
        return None;
    }
    if inputs[2].1 == axis {
        Some(Input::Dashing(axis.get_direction().unwrap()))
    } else {
        None
    }
}

fn read_normal(buffer: &InputBuffer) -> Option<Input> {
    let axis = buffer.top().axis;
    for state in buffer.iter().take(8) {
        for (id, state) in state.buttons.iter().enumerate() {
            if *state == ButtonState::JustPressed {
                return Some(Input::Normal(axis.get_standing(), Button::from_usize(id)));
            }
        }
    }
    None
}

fn read_command_normal(buffer: &InputBuffer) -> Option<Input> {
    let axis = buffer.top().axis;
    if axis.is_command() {
        for state in buffer.iter().take(8) {
            for (id, state) in state.buttons.iter().enumerate() {
                if *state == ButtonState::JustPressed {
                    return Some(Input::CommandNormal(
                        axis.get_standing(),
                        axis.get_direction().unwrap(),
                        Button::from_usize(id),
                    ));
                }
            }
        }
    }
    None
}

fn read_quarter_circle_motion(iter: DirectionIter<'_>) -> Option<Direction> {
    let inputs = iter.take(3).collect::<Vec<_>>();
    let (time, axis) = inputs[0];
    if time > MOTION_DIRECTION_SIZE || axis != Axis::Down {
        return None;
    }
    let (time, axis) = inputs[1];
    if time > MOTION_DIRECTION_SIZE || !(axis == Axis::DownLeft || axis == Axis::DownRight) {
        return None;
    }
    let (_, axis) = inputs[2];
    if axis.is_horizontal() {
        axis.get_direction()
    } else {
        None
    }
}
fn read_quarter_circle(buffer: &InputBuffer) -> Option<Input> {
    let mut iter = buffer.iter();
    let mut count = 0;
    while let Some(state) = iter.next() {
        if count >= 8 {
            break;
        }
        count += 1;
        for (id, state) in state.buttons.iter().enumerate() {
            if *state == ButtonState::JustPressed {
                if let Some(direction) =
                    read_quarter_circle_motion(iter.clone().into_direction_iter())
                {
                    return Some(Input::QuarterCircle(direction, Button::from_usize(id)));
                }
            }
        }
    }
    None
}

fn read_dragon_punch_motion(iter: DirectionIter<'_>) -> Option<Direction> {
    let inputs = iter.take(3).collect::<Vec<_>>();
    let (time, axis) = inputs[0];
    if time > MOTION_DIRECTION_SIZE || !axis.is_horizontal() {
        return None;
    }
    let (time, down_input) = inputs[1];
    if time > MOTION_DIRECTION_SIZE || down_input != Axis::Down {
        return None;
    }
    let (_, end_axis) = inputs[2];
    if end_axis.get_direction() == axis.get_direction() {
        axis.get_direction()
    } else {
        None
    }
}
fn read_dragon_punch(buffer: &InputBuffer) -> Option<Input> {
    let mut iter = buffer.iter();
    let mut count = 0;
    while let Some(state) = iter.next() {
        if count >= 8 {
            break;
        }
        count += 1;
        for (id, state) in state.buttons.iter().enumerate() {
            if *state == ButtonState::JustPressed {
                if let Some(direction) =
                    read_dragon_punch_motion(iter.clone().into_direction_iter())
                {
                    return Some(Input::DragonPunch(direction, Button::from_usize(id)));
                }
            }
        }
    }
    None
}

#[macro_export]
macro_rules! numpad_notation {
    (236$button:ident) => {
        Input::QuarterCircle(Direction::Forward, Button::$button)
    };
    (214$button:ident) => {
        Input::QuarterCircle(Direction::Backward, Button::$button)
    };
    (623$button:ident) => {
        Input::DragonPunch(Direction::Forward, Button::$button)
    };
    (421$button:ident) => {
        Input::DragonPunch(Direction::Backward, Button::$button)
    };

    (6$button:ident) => {
        Input::CommandNormal(Standing::Standing, Direction::Forward, Button::$button)
    };
    (3$button:ident) => {
        Input::CommandNormal(Standing::Crouching, Direction::Forward, Button::$button)
    };
    (5$button:ident) => {
        Input::Normal(Standing::Standing, Button::$button)
    };
    (2$button:ident) => {
        Input::Normal(Standing::Crouching, Button::$button)
    };
    (66) => {
        Input::Dashing(Direction::Forward)
    };
    (44) => {
        Input::Dashing(Direction::Backward)
    };
    (5) => {
        Input::Idle(Standing::Standing)
    };
    (2) => {
        Input::Idle(Standing::Crouching)
    };
    (6) => {
        Input::Walking(Direction::Forward)
    };
    (4) => {
        Input::Walking(Direction::Backward)
    };
}
