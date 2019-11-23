pub mod graphics {
    pub type Float = f32;
    pub type Vec2 = nalgebra::Vector2<Float>;
    pub type Vec3 = nalgebra::Vector3<Float>;
    pub type Matrix4 = nalgebra::Matrix4<Float>;

    pub fn up_dimension(vec2: Vec2) -> Vec3 {
        Vec3::new(vec2.x, vec2.y, 0.0)
    }
}

pub mod collision {
    pub type Int = i32;
    pub type Vec2 = nalgebra::Vector2<Int>;

    const UNITS_PER_PIXEL: i32 = 100;

    pub trait IntoGraphical {
        type Graphical;
        fn into_graphical(self) -> Self::Graphical;
    }

    impl IntoGraphical for i32 {
        type Graphical = super::graphics::Float;
        fn into_graphical(self) -> Self::Graphical {
            (self / UNITS_PER_PIXEL) as Self::Graphical
        }
    }
    impl IntoGraphical for Vec2 {
        type Graphical = super::graphics::Vec2;
        fn into_graphical(self) -> Self::Graphical {
            Self::Graphical::new(self.x.into_graphical(), -self.y.into_graphical())
        }
    }
}

use serde::de::DeserializeOwned;
use serde::Serialize;
use std::hash::Hash;

pub trait HashId: Eq + Hash + Default {}

pub trait StateId: Eq + Hash + Default + Serialize + DeserializeOwned {}

pub trait FgSerializable: Default + Serialize + DeserializeOwned {}

impl<T> HashId for T where T: Eq + Hash + Default {}

impl<T> StateId for T where T: Eq + Hash + Default + Serialize + DeserializeOwned {}

impl<T> FgSerializable for T where T: Default + Serialize + DeserializeOwned {}
