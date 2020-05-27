use strum::{EnumCount, IntoEnumIterator};
pub trait NextPrev {
    fn next(value: Self) -> Self;
    fn prev(value: Self) -> Self;
}

impl<T> NextPrev for T
where
    T: EnumCount + IntoEnumIterator + PartialEq,
{
    fn prev(value: Self) -> Self {
        Self::iter()
            .nth(
                Self::iter()
                    .position(|item| item == value)
                    .unwrap()
                    .checked_sub(1)
                    .unwrap_or(Self::count() - 1),
            )
            .unwrap()
    }
    fn next(value: Self) -> Self {
        Self::iter()
            .nth(
                Self::iter()
                    .position(|item| item == value)
                    .unwrap()
                    .checked_add(1)
                    .unwrap_or(0)
                    % Self::count(),
            )
            .unwrap()
    }
}
