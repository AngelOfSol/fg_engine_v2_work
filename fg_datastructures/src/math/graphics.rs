pub type Float = f32;
pub type Vec2 = nalgebra::Vector2<Float>;
pub type Vec3 = nalgebra::Vector3<Float>;
pub type Matrix4 = nalgebra::Matrix4<Float>;

pub trait IntoGraphical {
    type GraphicUnit;
    fn into_graphical(self) -> Self::GraphicUnit;
}
