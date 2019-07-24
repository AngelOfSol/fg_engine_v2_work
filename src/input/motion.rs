use super::ringbuffer::DirectionIter;
use super::{Axis, Button, ButtonState, Facing, InputBuffer, MOTION_DIRECTION_SIZE};

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
    pub fn direction_multiplier(self, facing: bool) -> i32 {
        let facing = if facing { 1 } else { -1 };
        let self_value = match self {
            DirectedAxis::Forward | DirectedAxis::UpForward | DirectedAxis::DownForward => 1,
            DirectedAxis::Backward | DirectedAxis::UpBackward | DirectedAxis::DownBackward => -1,
            _ => 0,
        };
        facing * self_value
    }
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
    Double(Button, Button),
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Input {
    Idle(DirectedAxis),
    PressButton(DirectedAxis, ButtonSet),
    ReleaseButton(ButtonSet),
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
            Input::PressButton(dir, button) => Input::PressButton(dir.invert(), button),
            Input::ReleaseButton(button) => Input::ReleaseButton(button),
            Input::QuarterCircle(dir, button) => Input::QuarterCircle(dir.invert(), button),
            Input::DragonPunch(dir, button) => Input::DragonPunch(dir.invert(), button),
            Input::SuperJump(dir) => Input::SuperJump(dir.invert()),
        }
    }
}

pub fn read_inputs(buffer: &InputBuffer, facing: Facing) -> Vec<Input> {
    [
        read_super_jump(buffer),
        read_super_jump_macro(buffer),
        read_dragon_punch(buffer),
        read_quarter_circle(buffer),
        read_button_press(buffer),
        read_button_press_neutral(buffer),
        read_button_release(buffer),
        read_double_tap(buffer),
        read_idle(buffer),
    ]
    .iter()
    .filter(|item| item.is_some())
    .map(|item| item.unwrap())
    .map(|item| {
        if facing == Facing::Right {
            item
        } else {
            item.invert()
        }
    })
    .collect()
}

fn read_button_set(button_list: [ButtonState; 4], check_state: ButtonState) -> Option<ButtonSet> {
    let mut buttons = None;
    for (id, state) in button_list.iter().enumerate() {
        if *state == check_state {
            buttons = match buttons {
                Some(button) => return Some(ButtonSet::Double(button, Button::from_usize(id))),
                None => Some(Button::from_usize(id)),
            }
        }
    }
    if let Some(button) = buttons {
        return Some(ButtonSet::Single(button));
    }
    None
}

fn read_idle(buffer: &InputBuffer) -> Option<Input> {
    Some(Input::Idle(buffer.top().axis.into()))
}

fn read_double_tap(buffer: &InputBuffer) -> Option<Input> {
    let inputs = buffer.iter().into_direction_iter().collect::<Vec<_>>();
    let (time, axis) = inputs[0];
    if time > MOTION_DIRECTION_SIZE * 2 {
        return None;
    }
    let (time, should_be_neutral) = inputs[1];
    if time > MOTION_DIRECTION_SIZE * 2 || should_be_neutral != Axis::Neutral {
        return None;
    }
    if inputs[2].1 == axis {
        Some(Input::DoubleTap(axis.into()))
    } else {
        None
    }
}

enum SuperJumpReaderState {
    Start,
    JumpRead(usize, Axis),
}

fn read_super_jump(buffer: &InputBuffer) -> Option<Input> {
    let mut ret = None;
    let mut state = SuperJumpReaderState::Start;
    for (duration, axis) in buffer.iter().into_direction_iter() {
        state = match state {
            SuperJumpReaderState::Start => {
                if duration <= 2 && axis.is_up() {
                    SuperJumpReaderState::JumpRead(0, axis)
                } else {
                    break;
                }
            }
            SuperJumpReaderState::JumpRead(total_duration, jump_axis) => {
                if axis.is_down() {
                    ret = if duration <= MOTION_DIRECTION_SIZE * 3 {
                        Some(Input::SuperJump(jump_axis.into()))
                    } else {
                        None
                    };
                    break;
                } else if total_duration + duration < MOTION_DIRECTION_SIZE * 3 {
                    SuperJumpReaderState::JumpRead(total_duration + duration, jump_axis)
                } else {
                    break;
                }
            }
        }
    }
    ret
}

fn read_super_jump_macro(buffer: &InputBuffer) -> Option<Input> {
    if buffer.top().buttons[0].is_pressed()
        && buffer.top().buttons[1].is_pressed()
        && buffer.top().axis.is_up()
    {
        Some(Input::SuperJump(buffer.top().axis.into()))
    } else {
        None
    }
}

fn read_button_press(buffer: &InputBuffer) -> Option<Input> {
    for input_state in buffer.iter().take(1) {
        if let Some(buttons) = read_button_set(input_state.buttons, ButtonState::JustPressed) {
            return Some(Input::PressButton(input_state.axis.into(), buttons));
        }
    }
    None
}

fn read_button_press_neutral(buffer: &InputBuffer) -> Option<Input> {
    for input_state in buffer.iter().take(1) {
        if let Some(buttons) = read_button_set(input_state.buttons, ButtonState::JustPressed) {
            return Some(Input::PressButton(DirectedAxis::Neutral, buttons));
        }
    }
    None
}

fn read_button_release(buffer: &InputBuffer) -> Option<Input> {
    for input_state in buffer.iter().take(1) {
        if let Some(buttons) = read_button_set(input_state.buttons, ButtonState::JustReleased) {
            return Some(Input::ReleaseButton(buttons));
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
