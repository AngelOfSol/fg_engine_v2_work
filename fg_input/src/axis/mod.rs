pub mod directed_axis;
pub mod direction;
pub mod facing;

//pub use directed_axis::DirectedAxis;
pub use direction::Direction;
pub use facing::Facing;
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq, Display)]
pub enum Axis {
    Up,
    Down,
    Right,
    Left,
    Neutral,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

impl From<[i32; 2]> for Axis {
    fn from([x, y]: [i32; 2]) -> Self {
        match x.cmp(&0) {
            std::cmp::Ordering::Greater => match y.cmp(&0) {
                std::cmp::Ordering::Greater => Axis::UpRight,
                std::cmp::Ordering::Less => Axis::DownRight,
                std::cmp::Ordering::Equal => Axis::Right,
            },
            std::cmp::Ordering::Less => match y.cmp(&0) {
                std::cmp::Ordering::Greater => Axis::UpLeft,
                std::cmp::Ordering::Less => Axis::DownLeft,
                std::cmp::Ordering::Equal => Axis::Left,
            },
            std::cmp::Ordering::Equal => match y.cmp(&0) {
                std::cmp::Ordering::Greater => Axis::Up,
                std::cmp::Ordering::Less => Axis::Down,
                std::cmp::Ordering::Equal => Axis::Neutral,
            },
        }
    }
}
