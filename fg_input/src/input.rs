mod axis;
mod input_coalesce;
mod input_state;
mod motion;

pub mod button;
pub mod control_scheme;
pub mod pads_context;

pub use axis::{DirectedAxis, Direction, Facing};
pub use input_state::InputState;
pub use motion::read_inputs;

use axis::Axis;

const MOTION_DIRECTION_SIZE: usize = 10;
