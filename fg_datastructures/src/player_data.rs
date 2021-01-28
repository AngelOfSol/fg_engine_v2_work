use serde::{Deserialize, Serialize};
use std::convert::From;
use std::iter::FromIterator;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
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
        PlayerData([f(l), f(r)])
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

    pub fn swap(self) -> PlayerData<T> {
        let [left, right] = self.0;
        Self([right, left])
    }
}

impl<T, E: std::fmt::Debug> PlayerData<Result<T, E>> {
    pub fn transpose(self) -> Result<PlayerData<T>, E> {
        let PlayerData([lhs, rhs]) = self;
        Ok([lhs?, rhs?].into())
    }
}
