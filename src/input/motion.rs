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
pub enum DirectedAxis {
    Up,
    Down,
    Forward,
    Backward,
    Neutral,
    UpForward,
    UpBackward,
    DownForward,
    DownBackward,
}

impl DirectedAxis {
    fn invert(self) -> Self {
        match self {
            DirectedAxis::Forward => DirectedAxis::Backward,
            DirectedAxis::DownForward => DirectedAxis::DownBackward,
            DirectedAxis::UpForward => DirectedAxis::UpBackward,
            DirectedAxis::Backward => DirectedAxis::Forward,
            DirectedAxis::DownBackward => DirectedAxis::DownForward,
            DirectedAxis::UpBackward => DirectedAxis::UpForward,
            value => value,
        }
    }
}
impl From<Axis> for DirectedAxis {
    fn from(item: Axis) -> Self {
        match item {
            Axis::Up => DirectedAxis::Up,
            Axis::Down => DirectedAxis::Down,
            Axis::Right => DirectedAxis::Forward,
            Axis::Left => DirectedAxis::Backward,
            Axis::Neutral => DirectedAxis::Neutral,
            Axis::UpRight => DirectedAxis::UpForward,
            Axis::UpLeft => DirectedAxis::UpBackward,
            Axis::DownRight => DirectedAxis::DownForward,
            Axis::DownLeft => DirectedAxis::DownBackward,
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ButtonSet {
    Single(Button),
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Input {
    Idle(DirectedAxis),
    Holding(DirectedAxis, ButtonSet),
    Button(DirectedAxis, ButtonSet),
    QuarterCircle(Direction, ButtonSet),
    DragonPunch(Direction, ButtonSet),
    DoubleTap(DirectedAxis),
    SuperJump(DirectedAxis),
}

impl Input {
    fn invert(self) -> Self {
        match self {
            Input::Idle(dir) => Input::Idle(dir.invert()),
            Input::DoubleTap(dir) => Input::DoubleTap(dir.invert()),
            Input::Button(dir, button) => Input::Button(dir.invert(), button),
            Input::Holding(dir, button) => Input::Holding(dir.invert(), button),
            Input::QuarterCircle(dir, button) => Input::QuarterCircle(dir.invert(), button),
            Input::DragonPunch(dir, button) => Input::DragonPunch(dir.invert(), button),
            Input::SuperJump(dir) => Input::SuperJump(dir.invert()),
        }
    }
}

pub fn read_inputs(buffer: &InputBuffer, facing_right: bool) -> Vec<Input> {
    [
        read_super_jump(buffer),
        read_dragon_punch(buffer),
        read_quarter_circle(buffer),
        read_button(buffer),
        read_holding(buffer),
        read_double_tap(buffer),
        read_idle(buffer),
    ]
    .iter()
    .filter(|item| item.is_some())
    .map(|item| item.unwrap())
    .map(|item| if facing_right { item } else { item.invert() })
    .collect()
}

fn read_idle(buffer: &InputBuffer) -> Option<Input> {
    Some(Input::Idle(buffer.top().axis.into()))
}

fn read_double_tap(buffer: &InputBuffer) -> Option<Input> {
    let inputs = buffer.iter().into_direction_iter().collect::<Vec<_>>();
    let (time, axis) = inputs[0];
    if time > MOTION_DIRECTION_SIZE {
        return None;
    }
    let (time, should_be_neutral) = inputs[1];
    if time > MOTION_DIRECTION_SIZE || should_be_neutral != Axis::Neutral {
        return None;
    }
    if inputs[2].1 == axis {
        Some(Input::DoubleTap(axis.into()))
    } else {
        None
    }
}

fn read_super_jump(buffer: &InputBuffer) -> Option<Input> {
    let inputs = buffer.iter().into_direction_iter().collect::<Vec<_>>();

    if inputs.len() < 5 {
        return None;
    }

    let (time, axis) = inputs[0];
    if time > 1 && !axis.is_up() {
        return None;
    }
    let time = inputs
        .iter()
        .skip(1)
        .take(3)
        .fold(Some(0), |acc, (time, axis)| match acc {
            Some(acc_time) => {
                if axis.is_down() {
                    None
                } else {
                    Some(*time + acc_time)
                }
            }
            None => None,
        });
    let (last_time, last_axis) = inputs[3];
    if time.is_none()
        || (time.unwrap() <= MOTION_DIRECTION_SIZE * 3
            && last_axis.is_down()
            && last_time <= MOTION_DIRECTION_SIZE)
    {
        Some(Input::SuperJump(axis.into()))
    } else {
        None
    }
}

fn read_button(buffer: &InputBuffer) -> Option<Input> {
    for input_state in buffer.iter().take(8) {
        for (id, state) in input_state.buttons.iter().enumerate() {
            if *state == ButtonState::JustPressed {
                return Some(Input::Button(
                    input_state.axis.into(),
                    ButtonSet::Single(Button::from_usize(id)),
                ));
            }
        }
    }
    None
}

fn read_holding(buffer: &InputBuffer) -> Option<Input> {
    for (id, state) in buffer.top().buttons.iter().enumerate() {
        if *state == ButtonState::Pressed {
            return Some(Input::Holding(
                buffer.top().axis.into(),
                ButtonSet::Single(Button::from_usize(id)),
            ));
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
                    return Some(Input::QuarterCircle(
                        direction,
                        ButtonSet::Single(Button::from_usize(id)),
                    ));
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
                    return Some(Input::DragonPunch(
                        direction,
                        ButtonSet::Single(Button::from_usize(id)),
                    ));
                }
            }
        }
    }
    None
}
