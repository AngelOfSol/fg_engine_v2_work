use inspect_design::Inspect;
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(
    Clone,
    Copy,
    Debug,
    Serialize,
    Deserialize,
    Hash,
    PartialEq,
    Eq,
    Display,
    Inspect,
    PartialOrd,
    Ord,
)]
pub enum Command {
    Attack5A,
    Attack2A,
    Attack5B,
    Attack3B,
    Attack2B,
    Attack6B,
    Attack5C,
    Attack2C,
    Attack5D,
    Attack2D,
    Air5A,
    Air8A,
    Air5B,
    Air2B,
    Air5C,
    Air2C,
    Stand,
    StandUp,
    Crouch,
    CrouchDown,
    WalkBackward,
    WalkForward,
    ForwardDash,
    DashSkid,
    BackDash,
    Jump,
    JumpForward,
    JumpBackward,
    SuperJump,
    SuperJumpForward,
    SuperJumpBackward,
    Fly1,
    Fly2,
    Fly3,
    Fly4,
    Fly6,
    Fly7,
    Fly8,
    Fly9,
    BorderEscape,
    BorderEscapeForward,
    BorderEscapeBackward,
    MeleeRestitution,
}
impl Default for Command {
    fn default() -> Self {
        Command::Stand
    }
}
