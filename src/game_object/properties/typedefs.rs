use inspect_design::Inspect;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, Inspect, Default)]
pub struct Speed(pub i32);
