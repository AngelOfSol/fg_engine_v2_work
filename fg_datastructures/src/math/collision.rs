use super::graphics;
pub use super::graphics::IntoGraphical;

pub type Int = i32;
pub type Vec2 = nalgebra::Vector2<i32>;

pub const UNITS_PER_PIXEL: i32 = 100;

impl IntoGraphical for i32 {
    type GraphicUnit = f32;
    fn into_graphical(self) -> Self::GraphicUnit {
        (self / UNITS_PER_PIXEL) as Self::GraphicUnit
    }
}

impl IntoGraphical for Vec2 {
    type GraphicUnit = graphics::Vec2;
    fn into_graphical(self) -> Self::GraphicUnit {
        Self::GraphicUnit::new(self.x.into_graphical(), -self.y.into_graphical())
    }
}
