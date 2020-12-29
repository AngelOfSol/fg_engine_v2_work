use crate::input::DirectedAxis;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ExtraData {
    JumpDirection(DirectedAxis),
    Stun(i32),
    None,
}

impl ExtraData {
    pub fn unwrap_jump_direction(self) -> DirectedAxis {
        match self {
            ExtraData::JumpDirection(dir) => dir,
            value => panic!("Expected JumpDirection, found {:?}.", value),
        }
    }
    pub fn unwrap_stun_mut(&mut self) -> &mut i32 {
        match self {
            ExtraData::Stun(ref mut time) => time,
            value => panic!("Expected HitStun, found {:?}.", value),
        }
    }
}
