mod local_history;
mod networked_history;

pub use local_history::LocalHistory;
pub use networked_history::{NetworkedHistory, PredictionResult};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InputRange {
    pub first: usize,
    pub last: usize,
}
