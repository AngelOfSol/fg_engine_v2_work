use std::{
    fmt::{Debug, Display, Formatter},
    str::FromStr,
};

use crate::{
    axis::{DirectedAxis, Direction},
    button::ButtonSet,
    notation,
};
use inspect_design::traits::{Inspect, InspectMut};
use nom::Finish;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Input {
    DragonPunch(Direction, ButtonSet),
    QuarterCircle(Direction, ButtonSet),
    PressButton(ButtonSet, DirectedAxis),
    SuperJump(DirectedAxis),
    DoubleTap(DirectedAxis),
    Idle(DirectedAxis),
}

impl Inspect for Input {
    const FLATTEN: bool = true;
    type State = Option<String>;
    fn inspect(&self, label: &str, _: &mut Self::State, ui: &imgui::Ui<'_>) {
        ui.text(imgui::im_str!("{}: {}", label, self))
    }
}

impl InspectMut for Input {
    fn inspect_mut(&mut self, label: &str, state: &mut Self::State, ui: &imgui::Ui<'_>) {
        let text = state.get_or_insert_with(|| format!("{}", self));

        let color_token = if text.parse::<Input>().is_err() {
            ui.push_style_color(imgui::StyleColor::Text, [1.0, 0.0, 0.0, 1.0])
        } else {
            ui.push_style_color(
                imgui::StyleColor::Text,
                ui.style_color(imgui::StyleColor::Text),
            )
        };

        text.inspect_mut(label, &mut (), ui);

        if let Ok(input) = text.parse() {
            *self = input;
        }

        color_token.pop(ui);
    }
}
impl Serialize for Input {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = format!("{}", self);
        value.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Input {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        value
            .parse()
            .map_err(|err| serde::de::Error::custom(format!("{:?}", err)))
    }
}

impl Debug for Input {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Input {{ {} }}", self)
    }
}

impl Display for Input {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Idle(inner) => write!(f, "{}", inner),
            Self::DoubleTap(inner) => write!(f, "{0}{0}", inner),
            Self::SuperJump(inner) => write!(f, "hj{}", inner),
            Self::DragonPunch(dir, button) => {
                write!(
                    f,
                    "{}{}",
                    match dir {
                        Direction::Forward => "623",
                        Direction::Backward => "421",
                    },
                    button
                )
            }
            Self::QuarterCircle(dir, button) => {
                write!(
                    f,
                    "{}{}",
                    match dir {
                        Direction::Forward => "236",
                        Direction::Backward => "214",
                    },
                    button
                )
            }
            Self::PressButton(button, dir) => write!(f, "{}{}", dir, button),
        }
    }
}

impl FromStr for Input {
    type Err = nom::error::ErrorKind;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        notation::parse(s)
            .finish()
            .map_err(|err| err.code)
            .map(|(_, item)| item)
    }
}

#[cfg(test)]
mod test {
    use super::Input;
    use crate::{
        axis::{DirectedAxis, Direction},
        button::button_set,
    };
    use std::str::FromStr;

    #[test]
    fn test_ordering() {
        let list: Vec<_> = vec![
            "623ab", "623b", "623a", "214ab", "214b", "214a", "6ab", "6b", "5b", "6a", "5a", "29",
            "66", "6", "5",
        ]
        .into_iter()
        .map(|value| Input::from_str(value))
        .flatten()
        .collect();

        let mut sorted = list.clone();
        sorted.sort();

        assert_eq!(sorted, list);
    }

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

        let value = Input::SuperJump(DirectedAxis::UpForward);
        // round trip
        assert_eq!(
            value,
            serde_json::to_string(&value)
                .and_then(|value| serde_json::from_str(&value))
                .unwrap()
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
        let value = Input::DoubleTap(DirectedAxis::Down);
        // round trip
        assert_eq!(
            value,
            serde_json::to_string(&value)
                .and_then(|value| serde_json::from_str(&value))
                .unwrap()
        );
    }
    #[test]
    fn dragon_punch() {
        assert_eq!(
            Input::from_str("623a"),
            Ok(Input::DragonPunch(Direction::Forward, button_set::A))
        );
        assert_eq!(
            Input::from_str("421a"),
            Ok(Input::DragonPunch(Direction::Backward, button_set::A))
        );
        let value = Input::DragonPunch(Direction::Backward, button_set::A);
        // round trip
        assert_eq!(
            value,
            serde_json::to_string(&value)
                .and_then(|value| serde_json::from_str(&value))
                .unwrap()
        );
    }
    #[test]
    fn quarter_circle() {
        assert_eq!(
            Input::from_str("236a"),
            Ok(Input::QuarterCircle(Direction::Forward, button_set::A))
        );
        assert_eq!(
            Input::from_str("214a"),
            Ok(Input::QuarterCircle(Direction::Backward, button_set::A))
        );
        let value = Input::QuarterCircle(Direction::Backward, button_set::A);
        // round trip
        assert_eq!(
            value,
            serde_json::to_string(&value)
                .and_then(|value| serde_json::from_str(&value))
                .unwrap()
        );
    }
    #[test]
    fn axis() {
        assert_eq!(Input::from_str("6"), Ok(Input::Idle(DirectedAxis::Forward)));
        assert_eq!(
            Input::from_str("1"),
            Ok(Input::Idle(DirectedAxis::DownBackward))
        );
        let value = Input::Idle(DirectedAxis::DownBackward);
        // round trip
        assert_eq!(
            value,
            serde_json::to_string(&value)
                .and_then(|value| serde_json::from_str(&value))
                .unwrap()
        );
    }
    #[test]
    fn press_button() {
        assert_eq!(
            Input::from_str("6d"),
            Ok(Input::PressButton(button_set::D, DirectedAxis::Forward,))
        );
        assert_eq!(
            Input::from_str("5a"),
            Ok(Input::PressButton(button_set::A, DirectedAxis::Neutral,))
        );

        let value = Input::PressButton(button_set::A, DirectedAxis::Neutral);
        // round trip
        assert_eq!(
            value,
            serde_json::to_string(&value)
                .and_then(|value| serde_json::from_str(&value))
                .unwrap()
        );
    }
}

impl Default for Input {
    fn default() -> Self {
        Self::Idle(Default::default())
    }
}

impl Input {
    pub(crate) fn invert(self) -> Self {
        match self {
            Input::Idle(dir) => Input::Idle(dir.invert()),
            Input::DoubleTap(dir) => Input::DoubleTap(dir.invert()),
            Input::PressButton(button, dir) => Input::PressButton(button, dir.invert()),
            Input::QuarterCircle(dir, button) => Input::QuarterCircle(dir.invert(), button),
            Input::DragonPunch(dir, button) => Input::DragonPunch(dir.invert(), button),
            Input::SuperJump(dir) => Input::SuperJump(dir.invert()),
        }
    }
}
