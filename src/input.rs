pub mod button;
pub mod control_scheme;
mod input_state;

mod input_coalesce;
#[macro_use]
mod motion;
mod axis;

pub use axis::{DirectedAxis, Direction, Facing};
pub use input_state::InputState;
pub use motion::{read_inputs, Input};

use axis::Axis;

const MOTION_DIRECTION_SIZE: usize = 10;
