//mod axis;
mod input_coalesce;
pub mod input_state;
mod motion;

pub mod button;

//pub use axis::{DirectedAxis, Direction, Facing};
pub use input_state::InputState;
pub use motion::read_inputs;

//pub use axis::Axis;

const MOTION_DIRECTION_SIZE: usize = 10;
