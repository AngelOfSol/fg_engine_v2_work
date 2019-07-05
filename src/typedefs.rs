pub mod graphics {
    type Float = f32;
    pub type Vec2 = nalgebra::Vector2<Float>;
    pub type Vec3 = nalgebra::Vector3<Float>;
    pub type Matrix4 = nalgebra::Matrix4<Float>;
}

pub mod collision {
    type Int = i32;
    pub type Vec2 = nalgebra::Vector2<Int>;
    // TODO: Put hitbox type in here
}