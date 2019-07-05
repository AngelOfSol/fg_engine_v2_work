pub mod graphics {
    pub type Float = f32;
    pub type Vec2 = nalgebra::Vector2<Float>;
    pub type Vec3 = nalgebra::Vector3<Float>;
    pub type Matrix4 = nalgebra::Matrix4<Float>;
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
            self.map(|item| item.into_graphical())
        }
    }
    // TODO: Put hitbox type in here

    pub struct Hitbox {
        center: Vec2,
        half_size: Vec2,
    }
    impl Hitbox {
        pub fn new() -> Self {
            Self {
                center: Vec2::zeros(),
                half_size: Vec2::zeros(),
            }
        }
    }

}
