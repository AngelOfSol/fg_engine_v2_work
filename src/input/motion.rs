use super::{Axis, Button, ButtonSet, ButtonState, Facing, InputState, MOTION_DIRECTION_SIZE};
use crate::character::components::Guard;

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

    pub fn is_cardinal(self) -> bool {
        match self {
            DirectedAxis::Forward
            | DirectedAxis::Up
            | DirectedAxis::Backward
            | DirectedAxis::Down => true,
            _ => false,
        }
    }
    pub fn matches_cardinal(self, target: DirectedAxis) -> bool {
        target.is_cardinal()
            && match target {
                DirectedAxis::Forward => match self {
                    DirectedAxis::UpForward | DirectedAxis::DownForward | DirectedAxis::Forward => {
                        true
                    }
                    _ => false,
                },
                DirectedAxis::Up => match self {
                    DirectedAxis::UpForward | DirectedAxis::UpBackward | DirectedAxis::Up => true,
                    _ => false,
                },
                DirectedAxis::Backward => match self {
                    DirectedAxis::UpBackward
                    | DirectedAxis::DownBackward
                    | DirectedAxis::Backward => true,
                    _ => false,
                },
                DirectedAxis::Down => match self {
                    DirectedAxis::DownForward | DirectedAxis::DownBackward | DirectedAxis::Down => {
                        true
                    }
                    _ => false,
                },
                _ => unreachable!(),
            }
    }

    pub fn is_backward(self) -> bool {
        match self {
            DirectedAxis::Backward | DirectedAxis::UpBackward | DirectedAxis::DownBackward => true,
            _ => false,
        }
    }
    pub fn is_down(self) -> bool {
        match self {
            DirectedAxis::Down | DirectedAxis::DownBackward | DirectedAxis::DownForward => true,
            _ => false,
        }
    }

    pub fn invert(self) -> Self {
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
    pub fn from_facing(item: Axis, facing: Facing) -> Self {
        let ret = match item {
            Axis::Up => DirectedAxis::Up,
            Axis::Down => DirectedAxis::Down,
            Axis::Right => DirectedAxis::Forward,
            Axis::Left => DirectedAxis::Backward,
            Axis::Neutral => DirectedAxis::Neutral,
            Axis::UpRight => DirectedAxis::UpForward,
            Axis::UpLeft => DirectedAxis::UpBackward,
            Axis::DownRight => DirectedAxis::DownForward,
            Axis::DownLeft => DirectedAxis::DownBackward,
        };

        if facing == Facing::Left {
            ret.invert()
        } else {
            ret
        }
    }

    pub fn is_horizontal(self) -> bool {
        match self {
            DirectedAxis::Up | DirectedAxis::Neutral | DirectedAxis::Down => false,
            _ => true,
        }
    }

    pub fn is_blocking(self, guard: Guard) -> bool {
        match guard {
            Guard::Mid => true,
            Guard::High => !self.is_down(),
            Guard::Low => self.is_down(),
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Input {
    Idle(DirectedAxis),
    PressButton(DirectedAxis, ButtonSet),
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
            Input::QuarterCircle(dir, button) => Input::QuarterCircle(dir.invert(), button),
            Input::DragonPunch(dir, button) => Input::DragonPunch(dir.invert(), button),
            Input::SuperJump(dir) => Input::SuperJump(dir.invert()),
        }
    }
}

fn read_button_set(button_list: [ButtonState; 4], check_state: ButtonState) -> Option<ButtonSet> {
    let mut buttons = None;
    for (id, _) in button_list
        .iter()
        .copied()
        .enumerate()
        .filter(|(_, state)| *state == check_state)
    {
        buttons = match buttons {
            Some(button) => Some(button | Button::from_id(id)),
            None => Some(Button::from_id(id).into()),
        }
    }
    buttons
}

pub fn read_inputs(buffer: impl Iterator<Item = InputState> + Clone, facing: Facing) -> Vec<Input> {
    [
        read_super_jump_new(buffer.clone()),
        read_dragon_punch_new(buffer.clone()),
        read_quarter_circle_new(buffer.clone()),
        read_button_press_new(buffer.clone()),
        read_double_tap_new(buffer.clone()),
        read_idle_new(buffer),
    ]
    .iter()
    .flatten()
    .copied()
    .map(|item| {
        if facing == Facing::Right {
            item
        } else {
            item.invert()
        }
    })
    .collect()
}

pub fn read_idle_new(mut buffer: impl Iterator<Item = InputState> + Clone) -> Option<Input> {
    buffer.next().map(|item| Input::Idle(item.axis.into()))
}
pub fn read_double_tap_new(mut buffer: impl Iterator<Item = InputState> + Clone) -> Option<Input> {
    for _ in 0..8 {
        let mut buffer = InputCoalesce::new(
            {
                let new_buffer = buffer.clone();
                buffer.next();
                new_buffer
            }
            .map(|item| item.axis.into()),
        );

        let ret = buffer
            .next()
            .and_then(|(dash_dir, count): (DirectedAxis, _)| {
                if count <= MOTION_DIRECTION_SIZE && dash_dir.is_cardinal() {
                    Some(dash_dir)
                } else {
                    None
                }
            })
            .and_then(|dash_dir| {
                let (neutral_dir, count) = buffer.next()?;
                if count <= MOTION_DIRECTION_SIZE && !neutral_dir.matches_cardinal(dash_dir) {
                    Some(dash_dir)
                } else {
                    None
                }
            })
            .and_then(|dash_dir| {
                let (redash_dir, count) = buffer.next()?;
                if count <= MOTION_DIRECTION_SIZE * 2 && redash_dir.matches_cardinal(dash_dir) {
                    Some(Input::DoubleTap(dash_dir))
                } else {
                    None
                }
            });
        if ret.is_some() {
            return ret;
        }
    }
    None
}

fn read_button_press_new(mut buffer: impl Iterator<Item = InputState> + Clone) -> Option<Input> {
    for _ in 0..8 {
        if let Some(buttons) = read_recent_button_set(buffer.clone()) {
            return Some(Input::PressButton(
                buffer.next().unwrap().axis.into(),
                buttons,
            ));
        }
        buffer.next();
    }
    None
}

fn read_recent_button_set(
    mut buffer: impl Iterator<Item = InputState> + Clone,
) -> Option<ButtonSet> {
    buffer
        .next()
        .and_then(|base| read_button_set(base.buttons, ButtonState::JustPressed))
        .map(|mut buttons| {
            for _ in 0..2 {
                let next = if let Some(value) = buffer
                    .next()
                    .and_then(|next| read_button_set(next.buttons, ButtonState::JustPressed))
                {
                    value
                } else {
                    break;
                };
                buttons |= next;
            }
            buttons
        })
}

use std::collections::HashSet;
#[derive(Clone, Debug)]
enum ReadInput {
    Optional(HashSet<Axis>, usize),
    RequiredWithin(HashSet<Axis>, usize),
    Required(HashSet<Axis>),
    Pass,
}

use maplit::hashset;

fn read_super_jump_new(buffer: impl Iterator<Item = InputState> + Clone) -> Option<Input> {
    let buffer = InputCoalesce::new(buffer.map(|item| item.axis));

    let super_jump_right = vec![
        ReadInput::RequiredWithin(hashset!(Axis::UpRight), MOTION_DIRECTION_SIZE),
        ReadInput::Optional(hashset!(Axis::Neutral, Axis::Right), MOTION_DIRECTION_SIZE),
        ReadInput::RequiredWithin(
            hashset!(Axis::Down, Axis::DownRight, Axis::DownLeft),
            MOTION_DIRECTION_SIZE,
        ),
        ReadInput::Pass,
    ];

    if ReadInput::check(super_jump_right.iter(), buffer.clone()).unwrap_or(false) {
        return Some(Input::SuperJump(DirectedAxis::UpForward));
    }
    let super_jump = vec![
        ReadInput::RequiredWithin(hashset!(Axis::Up), MOTION_DIRECTION_SIZE),
        ReadInput::Optional(
            hashset!(Axis::Neutral, Axis::Right, Axis::Left),
            MOTION_DIRECTION_SIZE,
        ),
        ReadInput::RequiredWithin(
            hashset!(Axis::Down, Axis::DownRight, Axis::DownLeft),
            MOTION_DIRECTION_SIZE * 2,
        ),
        ReadInput::Pass,
    ];

    if ReadInput::check(super_jump.iter(), buffer.clone()).unwrap_or(false) {
        return Some(Input::SuperJump(DirectedAxis::Up));
    }

    let super_jump_left = vec![
        ReadInput::RequiredWithin(hashset!(Axis::UpLeft), MOTION_DIRECTION_SIZE),
        ReadInput::Optional(hashset!(Axis::Neutral, Axis::Left), MOTION_DIRECTION_SIZE),
        ReadInput::RequiredWithin(
            hashset!(Axis::Down, Axis::DownRight, Axis::DownLeft),
            MOTION_DIRECTION_SIZE * 2,
        ),
        ReadInput::Pass,
    ];

    if ReadInput::check(super_jump_left.iter(), buffer.clone()).unwrap_or(false) {
        return Some(Input::SuperJump(DirectedAxis::UpBackward));
    }

    None
}

fn read_dragon_punch_new(mut buffer: impl Iterator<Item = InputState> + Clone) -> Option<Input> {
    for _ in 0..8 {
        if let Some(buttons) = read_recent_button_set(buffer.clone()) {
            let buffer = InputCoalesce::new(
                {
                    let new_buffer = buffer.clone();
                    buffer.next();
                    new_buffer
                }
                .map(|item| item.axis),
            );

            let dp_right = vec![
                ReadInput::Optional(hashset!(Axis::Right), MOTION_DIRECTION_SIZE),
                ReadInput::RequiredWithin(hashset!(Axis::DownRight), MOTION_DIRECTION_SIZE * 2),
                ReadInput::RequiredWithin(hashset!(Axis::Down), MOTION_DIRECTION_SIZE),
                ReadInput::Optional(hashset!(Axis::DownRight), MOTION_DIRECTION_SIZE),
                ReadInput::Required(hashset!(Axis::Right)),
                ReadInput::Pass,
            ];

            if ReadInput::check(dp_right.iter(), buffer.clone()).unwrap_or(false) {
                return Some(Input::DragonPunch(Direction::Forward, buttons));
            }

            let dp_left = vec![
                ReadInput::Optional(hashset!(Axis::Left), MOTION_DIRECTION_SIZE),
                ReadInput::RequiredWithin(hashset!(Axis::DownLeft), MOTION_DIRECTION_SIZE * 2),
                ReadInput::RequiredWithin(hashset!(Axis::Down), MOTION_DIRECTION_SIZE),
                ReadInput::Optional(hashset!(Axis::DownLeft), MOTION_DIRECTION_SIZE),
                ReadInput::Required(hashset!(Axis::Left)),
                ReadInput::Pass,
            ];

            if ReadInput::check(dp_left.iter(), buffer.clone()).unwrap_or(false) {
                return Some(Input::DragonPunch(Direction::Backward, buttons));
            }
        }
        buffer.next();
    }
    None
}

fn read_quarter_circle_new(mut buffer: impl Iterator<Item = InputState> + Clone) -> Option<Input> {
    for _ in 0..8 {
        if let Some(buttons) = read_recent_button_set(buffer.clone()) {
            let buffer = InputCoalesce::new(
                {
                    let new_buffer = buffer.clone();
                    buffer.next();
                    new_buffer
                }
                .map(|item| item.axis),
            );

            let qcf_right = vec![
                ReadInput::Optional(hashset!(Axis::UpRight), MOTION_DIRECTION_SIZE),
                ReadInput::RequiredWithin(hashset!(Axis::Right), MOTION_DIRECTION_SIZE * 2),
                ReadInput::RequiredWithin(hashset!(Axis::DownRight), MOTION_DIRECTION_SIZE),
                ReadInput::Required(hashset!(Axis::Down)),
                ReadInput::Pass,
            ];

            if ReadInput::check(qcf_right.iter(), buffer.clone()).unwrap_or(false) {
                return Some(Input::QuarterCircle(Direction::Forward, buttons));
            }

            let qcf_left = vec![
                ReadInput::Optional(hashset!(Axis::UpLeft), MOTION_DIRECTION_SIZE),
                ReadInput::RequiredWithin(hashset!(Axis::Left), MOTION_DIRECTION_SIZE * 2),
                ReadInput::RequiredWithin(hashset!(Axis::DownLeft), MOTION_DIRECTION_SIZE),
                ReadInput::Required(hashset!(Axis::Down)),
                ReadInput::Pass,
            ];
            if ReadInput::check(qcf_left.iter(), buffer.clone()).unwrap_or(false) {
                return Some(Input::QuarterCircle(Direction::Backward, buttons));
            }
        }
        buffer.next();
    }
    None
}
#[derive(Copy, Clone, Debug, Hash)]
enum ReadInputAction {
    Consume,
    Continue,
    Fail,
    Pass,
}

impl ReadInput {
    fn run(&self, axis: Axis, duration: usize) -> ReadInputAction {
        match self {
            Self::Optional(axes, target_duration) => {
                if axes.contains(&axis) && *target_duration >= duration {
                    ReadInputAction::Consume
                } else {
                    ReadInputAction::Continue
                }
            }
            Self::RequiredWithin(axes, target_duration) => {
                if axes.contains(&axis) && *target_duration >= duration {
                    ReadInputAction::Consume
                } else {
                    ReadInputAction::Fail
                }
            }
            Self::Required(axes) => {
                if axes.contains(&axis) {
                    ReadInputAction::Consume
                } else {
                    ReadInputAction::Fail
                }
            }
            Self::Pass => ReadInputAction::Pass,
        }
    }

    fn check<'a>(
        mut machine: impl Iterator<Item = &'a ReadInput>,
        mut directions: impl Iterator<Item = (Axis, usize)>,
    ) -> Option<bool> {
        let mut current = directions.next()?;
        Some(loop {
            let state = machine.next()?;
            match state.run(current.0, current.1) {
                ReadInputAction::Consume => current = directions.next()?,
                ReadInputAction::Continue => (),
                ReadInputAction::Fail => break false,
                ReadInputAction::Pass => break true,
            }
        })
    }
}

#[derive(Clone, Debug)]
struct InputCoalesce<I, V> {
    iter: I,
    value: Option<V>,
}

impl<I, V> InputCoalesce<I, V>
where
    I: Iterator<Item = V>,
    V: PartialEq,
{
    pub fn new(iter: I) -> Self {
        Self { iter, value: None }
    }
}

use std::iter::Iterator;

impl<I, V> Iterator for InputCoalesce<I, V>
where
    I: Iterator<Item = V>,
    V: PartialEq,
{
    type Item = (I::Item, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let (mut count, value) = match self.value.take() {
            Some(value) => (1, value),
            None => (1, self.iter.next()?),
        };

        let new_value = loop {
            if let Some(new_value) = self.iter.next() {
                if new_value == value {
                    count += 1;
                } else {
                    break Some(new_value);
                }
            } else {
                break None;
            };
        };
        self.value = new_value;

        Some((value, count))
    }
}
