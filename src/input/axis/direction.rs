use inspect_design::Inspect;
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Inspect)]
pub enum Direction {
    Forward,
    Backward,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Forward
    }
}

impl Direction {
    pub fn invert(self) -> Self {
        match self {
            Direction::Forward => Direction::Backward,
            Direction::Backward => Direction::Forward,
        }
    }
}
