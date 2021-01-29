mod input_coalesce;
pub mod input_state;
mod motion;

pub mod button;

pub use input_state::InputState;
pub use motion::read_inputs;

const MOTION_DIRECTION_SIZE: usize = 10;
