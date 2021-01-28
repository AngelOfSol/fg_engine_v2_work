use crate::motion::Input;

use super::button::{ButtonSet, ButtonState};
use super::input_coalesce::InputCoalesce;
use super::{InputState, MOTION_DIRECTION_SIZE};
use crate::axis::{Axis, DirectedAxis, Direction, Facing};

fn read_button_set(button_list: [ButtonState; 5], check_state: ButtonState) -> Option<ButtonSet> {
    let mut buttons = None;
    for (id, _) in button_list
        .iter()
        .copied()
        .enumerate()
        .filter(|(_, state)| *state == check_state)
    {
        buttons = match buttons {
            Some(button) => Some(button | ButtonSet::from_id(id)),
            None => Some(ButtonSet::from_id(id)),
        }
    }
    buttons
}

pub fn read_inputs<'a>(
    buffer: impl Iterator<Item = &'a InputState> + Clone,
    facing: Facing,
    forgiveness: usize,
) -> Vec<Input> {
    [
        read_super_jump(buffer.clone()),
        read_dragon_punch(buffer.clone(), forgiveness),
        read_quarter_circle(buffer.clone(), forgiveness),
        read_button_press(buffer.clone(), forgiveness),
        read_double_tap(buffer.clone(), forgiveness),
        read_idle(buffer),
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

pub fn read_idle<'a>(mut buffer: impl Iterator<Item = &'a InputState> + Clone) -> Option<Input> {
    buffer.next().map(|item| Input::Idle(item.axis().into()))
}

pub fn read_double_tap<'a>(
    mut buffer: impl Iterator<Item = &'a InputState> + Clone,
    forgiveness: usize,
) -> Option<Input> {
    for _ in 0..forgiveness {
        let mut buffer = InputCoalesce::new(
            {
                let new_buffer = buffer.clone();
                buffer.next();
                new_buffer
            }
            .map(|item| item.axis().into()),
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

fn read_button_press<'a>(
    mut buffer: impl Iterator<Item = &'a InputState> + Clone,
    forgiveness: usize,
) -> Option<Input> {
    for _ in 0..forgiveness {
        if let Some(buttons) = read_recent_button_set(buffer.clone()) {
            return Some(Input::PressButton(
                buffer.next().unwrap().axis().into(),
                buttons,
            ));
        }
        buffer.next();
    }
    None
}

fn read_recent_button_set<'a>(
    mut buffer: impl Iterator<Item = &'a InputState> + Clone,
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

use std::{collections::HashSet, fmt::Debug};
#[derive(Clone, Debug)]
enum ReadInput {
    Optional(HashSet<Axis>, usize),
    RequiredWithin(HashSet<Axis>, usize),
    Required(HashSet<Axis>),
    Pass,
}

use maplit::hashset;

fn read_super_jump<'a>(buffer: impl Iterator<Item = &'a InputState> + Clone) -> Option<Input> {
    let buffer = InputCoalesce::new(buffer.map(|item| item.axis()));

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

fn read_dragon_punch<'a>(
    mut buffer: impl Iterator<Item = &'a InputState> + Clone,
    forgiveness: usize,
) -> Option<Input> {
    for _ in 0..forgiveness {
        if let Some(buttons) = read_recent_button_set(buffer.clone()) {
            let buffer = InputCoalesce::new(
                {
                    let new_buffer = buffer.clone();
                    buffer.next();
                    new_buffer
                }
                .map(|item| item.axis()),
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

fn read_quarter_circle<'a>(
    mut buffer: impl Iterator<Item = &'a InputState> + Clone,
    forgiveness: usize,
) -> Option<Input> {
    for _ in 0..forgiveness {
        if let Some(buttons) = read_recent_button_set(buffer.clone()) {
            let buffer = InputCoalesce::new(
                {
                    let new_buffer = buffer.clone();
                    buffer.next();
                    new_buffer
                }
                .map(|item| item.axis()),
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
