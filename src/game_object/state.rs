use crate::typedefs::collision;
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Position {
    value: collision::Vec2,
}
