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
                    .and_then(|i| i.checked_sub(1))
                    .unwrap_or(Self::COUNT - 1),
            )
            .unwrap()
    }
    fn next(value: Self) -> Self {
        Self::iter()
            .nth(
                Self::iter()
                    .position(|item| item == value)
                    .and_then(|i| i.checked_add(1))
                    .unwrap_or(0)
                    % Self::COUNT,
            )
            .unwrap()
    }
}
