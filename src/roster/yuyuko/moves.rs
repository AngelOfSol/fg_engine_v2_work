use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum YuyukoMove {
    Stand,
    WalkBackward,
    WalkForward,
    #[serde(rename = "attack5a")]
    Attack5A,
    Crouch,
    ToCrouch,
    ToStand,
    StartForwardDash,
    ForwardDash,
    Jump,
    JumpForward,
    JumpBackward,
    SuperJump,
    SuperJumpForward,
    SuperJumpBackward,
    AirIdle,
}

impl Default for YuyukoMove {
    fn default() -> Self {
        YuyukoMove::Stand
    }
}

impl YuyukoMove {
    pub fn to_string(self) -> String {
        serde_json::to_string(&self)
            .unwrap()
            .trim_matches('\"')
            .to_owned()
    }
}
