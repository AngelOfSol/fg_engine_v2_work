#![feature(or_patterns)]

pub mod axis;
pub mod guard;
mod input;
pub mod notation;
pub use input::*;
pub mod interpret;
pub mod motion;

pub use axis::Facing;
pub use motion::Input;
