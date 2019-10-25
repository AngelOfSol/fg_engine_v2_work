use super::moves::MoveId;
use crate::command_list::CommandList;
use crate::input::{Button, ButtonSet, DirectedAxis, Input};
use crate::{make_command_list, numpad, read_axis};

pub fn generate_command_list() -> CommandList<MoveId> {
    make_command_list! {
        numpad!(press A B) => MoveId::FlyStart,

        numpad!(6 A B), numpad!(66) => MoveId::ForwardDashStart,
        numpad!(4 A B), numpad!(44) => MoveId::BackDash,

        //numpad!(1 A B), numpad!(2 A B), numpad!(3 A B)  => MoveId::BorderEscapeJump,

        numpad!(5 A), numpad!(4 A), numpad!(6 A) => MoveId::Attack5A,
        numpad!(5 A), numpad!(4 A), numpad!(6 A) => MoveId::Air5A,

        numpad!(6 B) => MoveId::Attack6B,

        numpad!(5 C), numpad!(4 C), numpad!(6 C) => MoveId::Attack5C,

        numpad!(27), numpad!(28), numpad!(29) => MoveId::SuperJump,
        numpad!(7), numpad!(8), numpad!(9) => MoveId::Jump,
        numpad!(7), numpad!(8), numpad!(9) => MoveId::SuperJump,

        numpad!(6) => MoveId::WalkForward,
        numpad!(4) => MoveId::WalkBackward,

        numpad!(1), numpad!(2), numpad!(3) => MoveId::Crouch,
        numpad!(1), numpad!(2), numpad!(3) => MoveId::ToCrouch,

        numpad!(5) => MoveId::Stand,
        numpad!(5) => MoveId::ToStand,
        numpad!(5) => MoveId::ForwardDashEnd
    }
}
