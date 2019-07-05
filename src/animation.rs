#[allow(clippy::module_inception)]
mod animation;
mod sprite;

pub use animation::{Animation, AnimationUi};
pub use sprite::{load_image, Sprite};
