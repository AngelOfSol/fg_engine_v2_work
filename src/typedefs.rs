pub const REPLAY_VERSION: usize = 2;

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

pub mod player {
    use std::convert::From;
    use std::iter::FromIterator;
    use std::ops::{Deref, DerefMut};

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
    pub struct PlayerData<T>([T; 2]);

    impl<T> Deref for PlayerData<T> {
        type Target = [T; 2];

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<T> DerefMut for PlayerData<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl<T> From<[T; 2]> for PlayerData<T> {
        fn from(data: [T; 2]) -> Self {
            Self(data)
        }
    }

    impl<T> From<(T, T)> for PlayerData<T> {
        fn from(data: (T, T)) -> Self {
            Self([data.0, data.1])
        }
    }

    impl<T> FromIterator<T> for PlayerData<T> {
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
            let mut iter = iter.into_iter();
            PlayerData([iter.next().unwrap(), iter.next().unwrap()])
        }
    }

    impl<T> PlayerData<T> {
        pub fn p1(&self) -> &T {
            &self.0[0]
        }
        pub fn p1_mut(&mut self) -> &mut T {
            &mut self.0[0]
        }
        pub fn p2(&self) -> &T {
            &self.0[1]
        }
        pub fn p2_mut(&mut self) -> &mut T {
            &mut self.0[1]
        }

        pub fn map<U, F: FnMut(T) -> U>(self, mut f: F) -> PlayerData<U> {
            let [l, r] = self.0;
            Self([f(l), f(r)])
        }

        pub fn as_ref(&self) -> PlayerData<&T> {
            PlayerData([&self.0[0], &self.0[1]])
        }

        #[allow(dead_code)]
        pub fn both(&self) -> (&T, &T) {
            let data = self.0.split_at(1);
            (&data.0[0], &data.1[0])
        }
        pub fn both_mut(&mut self) -> (&mut T, &mut T) {
            let data = self.0.split_at_mut(1);
            (&mut data.0[0], &mut data.1[0])
        }
    }
}
