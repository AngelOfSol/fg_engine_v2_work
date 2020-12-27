use super::axis::{Axis, DirectedAxis, Direction, Facing};
use super::button::{Button, ButtonSet, ButtonState};
use super::input_coalesce::InputCoalesce;
use super::{InputState, MOTION_DIRECTION_SIZE};
use inspect_design::Inspect;
use pest::Parser;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Inspect)]
pub enum Input {
    Idle(DirectedAxis),
    PressButton(DirectedAxis, ButtonSet),
    QuarterCircle(Direction, ButtonSet),
    DragonPunch(Direction, ButtonSet),
    DoubleTap(DirectedAxis),
    SuperJump(DirectedAxis),
}

pub mod parse {
    use pest_derive::Parser;
    #[derive(Parser)]
    #[grammar = "input/motion.pest"]
    pub struct InputParser;
}

impl FromStr for Input {
    type Err = pest::error::Error<parse::Rule>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use parse::*;
        let input = InputParser::parse(Rule::input, s)?
            .next()
            .unwrap()
            .into_inner()
            .next()
            .unwrap();

        match input.as_rule() {
            Rule::super_jump => Ok(Self::SuperJump(
                DirectedAxis::from_str(input.into_inner().as_str()).unwrap(),
            )),
            Rule::double_tap => Ok(Self::DoubleTap(
                DirectedAxis::from_str(input.as_str()).unwrap(),
            )),
            Rule::dp => Ok(Self::DragonPunch(Direction::Forward, {
                let input = input.into_inner().next().unwrap();
                match input.as_rule() {
                    Rule::press => FromStr::from_str(input.as_str()).unwrap(),
                    _ => panic!("other button press types are not supported at this time"),
                }
            })),
            Rule::rdp => Ok(Self::DragonPunch(Direction::Backward, {
                let input = input.into_inner().next().unwrap();
                match input.as_rule() {
                    Rule::press => FromStr::from_str(input.as_str()).unwrap(),
                    _ => panic!("other button press types are not supported at this time"),
                }
            })),
            Rule::qcf => Ok(Self::QuarterCircle(Direction::Forward, {
                let input = input.into_inner().next().unwrap();
                match input.as_rule() {
                    Rule::press => FromStr::from_str(input.as_str()).unwrap(),
                    _ => panic!("other button press types are not supported at this time"),
                }
            })),
            Rule::qcb => Ok(Self::QuarterCircle(Direction::Backward, {
                let input = input.into_inner().next().unwrap();
                match input.as_rule() {
                    Rule::press => FromStr::from_str(input.as_str()).unwrap(),
                    _ => panic!("other button press types are not supported at this time"),
                }
            })),
            Rule::axis => Ok(Self::Idle(DirectedAxis::from_str(input.as_str()).unwrap())),
            Rule::button_press => {
                let mut pairs = input.into_inner();
                let axis = FromStr::from_str(pairs.next().unwrap().as_str()).unwrap();
                let buttons = FromStr::from_str(pairs.next().unwrap().as_str()).unwrap();

                Ok(Self::PressButton(axis, buttons))
            }

            _ => unreachable!(),
        }
    }
}
#[cfg(test)]
mod test {
    use super::Input;
    use crate::input::{
        button::{Button, ButtonSet},
        DirectedAxis, Direction,
    };
    use std::str::FromStr;

    #[test]
    fn test_super_jump() {
        assert_eq!(
            Input::from_str("hj7"),
            Ok(Input::SuperJump(DirectedAxis::UpBackward))
        );
        assert_eq!(
            Input::from_str("hj8"),
            Ok(Input::SuperJump(DirectedAxis::Up))
        );
        assert_eq!(
            Input::from_str("29"),
            Ok(Input::SuperJump(DirectedAxis::UpForward))
        );
    }
    #[test]
    fn double_tap() {
        assert_eq!(
            Input::from_str("66"),
            Ok(Input::DoubleTap(DirectedAxis::Forward))
        );
        assert_eq!(
            Input::from_str("44"),
            Ok(Input::DoubleTap(DirectedAxis::Backward))
        );
        assert_eq!(
            Input::from_str("22"),
            Ok(Input::DoubleTap(DirectedAxis::Down))
        );
    }
    #[test]
    fn dragon_punch() {
        assert_eq!(
            Input::from_str("623a"),
            Ok(Input::DragonPunch(
                Direction::Forward,
                ButtonSet::from(Button::A)
            ))
        );
        assert_eq!(
            Input::from_str("421a"),
            Ok(Input::DragonPunch(
                Direction::Backward,
                ButtonSet::from(Button::A)
            ))
        );
    }
    #[test]
    fn quarter_circle() {
        assert_eq!(
            Input::from_str("236a"),
            Ok(Input::QuarterCircle(
                Direction::Forward,
                ButtonSet::from(Button::A)
            ))
        );
        assert_eq!(
            Input::from_str("214a"),
            Ok(Input::QuarterCircle(
                Direction::Backward,
                ButtonSet::from(Button::A)
            ))
        );
    }
    #[test]
    fn axis() {
        assert_eq!(Input::from_str("6"), Ok(Input::Idle(DirectedAxis::Forward)));
        assert_eq!(
            Input::from_str("1"),
            Ok(Input::Idle(DirectedAxis::DownBackward))
        );
    }
    #[test]
    fn press_button() {
        assert_eq!(
            Input::from_str("6d"),
            Ok(Input::PressButton(
                DirectedAxis::Forward,
                ButtonSet::from(Button::D)
            ))
        );
        assert_eq!(
            Input::from_str("5a"),
            Ok(Input::PressButton(
                DirectedAxis::Neutral,
                ButtonSet::from(Button::A)
            ))
        );
    }
}

impl Default for Input {
    fn default() -> Self {
        Self::Idle(Default::default())
    }
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

fn read_button_set(button_list: [ButtonState; 5], check_state: ButtonState) -> Option<ButtonSet> {
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
    buffer.next().map(|item| Input::Idle(item.axis.into()))
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

fn read_button_press<'a>(
    mut buffer: impl Iterator<Item = &'a InputState> + Clone,
    forgiveness: usize,
) -> Option<Input> {
    for _ in 0..forgiveness {
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

use std::{collections::HashSet, str::FromStr};
#[derive(Clone, Debug)]
enum ReadInput {
    Optional(HashSet<Axis>, usize),
    RequiredWithin(HashSet<Axis>, usize),
    Required(HashSet<Axis>),
    Pass,
}

use maplit::hashset;

fn read_super_jump<'a>(buffer: impl Iterator<Item = &'a InputState> + Clone) -> Option<Input> {
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
