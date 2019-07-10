use super::{Axis, InputBuffer, MOTION_DIRECTION_SIZE, Button};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Direction {
    Forward,
    Backward,
}

impl Direction {
    fn invert(&mut self) {
        *self = match self {
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
    fn invert(&mut self) {
        match self {
            Input::Walking(dir) => dir.invert(),
            Input::Dashing(dir) => dir.invert(),
            Input::CommandNormal(_, dir, _) => dir.invert(),
            Input::QuarterCircle(dir, _) => dir.invert(),
            Input::DragonPunch(dir, _) => dir.invert(),
            _ => (),
        }
    }
}

fn read_idle(buffer: &InputBuffer) -> Option<Input> {
    match buffer.top().axis {
        Axis::Neutral => Some(Input::Idle(Standing::Standing)),
        Axis::DownLeft | Axis::Down | Axis::DownRight => Some(Input::Idle(Standing::Crouching)),
        _ => None,
    }
}

fn read_walk(buffer: &InputBuffer) -> Option<Input> {
    match buffer.top().axis {
        Axis::Left => Some(Input::Walking(Direction::Backward)),
        Axis::Right => Some(Input::Walking(Direction::Forward)),
        _ => None,
    }
}

fn read_dashing(buffer: &InputBuffer) -> Option<Input> {
    let inputs = buffer
        .iter()
        .into_direction_iter()
        .take(3)
        .collect::<Vec<_>>();
    let (time, axis) = inputs[0];
    if time > MOTION_DIRECTION_SIZE || !axis.is_horizontal() {
        return None;
    }
    let (time, should_be_neutral) = inputs[1];
    if time > MOTION_DIRECTION_SIZE || axis != Axis::Neutral {
        return None;
    }
    if inputs[2].1 == axis {
        Some(Input::Dashing(axis.get_direction().unwrap()))
    } else {
        None
    }
}

fn read_normal(buffer: &InputBuffer) -> Option<Input> {
    let top = buffer.top();
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
        Input::Normal(Standing::Crouching, Button::$button)
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

fn rest() {
    numpad_notation!(236 A);
}
