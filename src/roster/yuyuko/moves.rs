use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MoveId {
    Stand,
    WalkBackward,
    WalkForward,
    #[serde(rename = "attack5a")]
    Attack5A,
    Crouch,
    ToCrouch,
    ToStand,
    ForwardDashStart,
    ForwardDash,
    ForwardDashEnd,
    BackDash,
    Jump,
    SuperJump,
    AirIdle,
    Fly,
    FlyStart,
    FlyEnd,
}

impl Default for MoveId {
    fn default() -> Self {
        MoveId::Stand
    }
}

impl MoveId {
    pub fn to_string(self) -> String {
        serde_json::to_string(&self)
            .unwrap()
            .trim_matches('\"')
            .to_owned()
    }
}
