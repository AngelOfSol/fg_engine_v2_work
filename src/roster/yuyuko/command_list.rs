use super::moves::YuyukoMove;
use crate::command_list::CommandList;
use crate::input::{Button, ButtonSet, DirectedAxis, Input};

use crate::{make_command_list, numpad, read_axis};
pub fn generate_command_list() ->
    CommandList<YuyukoMove> {
    make_command_list! {
        numpad!(5 A), numpad!(4 A), numpad!(6 A) => YuyukoMove::Attack5A,

        numpad!(66) => YuyukoMove::StartForwardDash,

        numpad!(29) => YuyukoMove::SuperJumpForward,
        numpad!(28) => YuyukoMove::SuperJump,
        numpad!(27) => YuyukoMove::SuperJumpBackward,

        numpad!(9) => YuyukoMove::JumpForward,
        numpad!(8) => YuyukoMove::Jump,
        numpad!(7) => YuyukoMove::JumpBackward,
        numpad!(9) => YuyukoMove::SuperJumpForward,
        numpad!(8) => YuyukoMove::SuperJump,
        numpad!(7) => YuyukoMove::SuperJumpBackward,

        numpad!(6) => YuyukoMove::WalkForward,
        numpad!(4) => YuyukoMove::WalkBackward,

        numpad!(1) => YuyukoMove::Crouch,
        numpad!(2) => YuyukoMove::Crouch,
        numpad!(3) => YuyukoMove::Crouch,
        numpad!(1) => YuyukoMove::ToCrouch,
        numpad!(2) => YuyukoMove::ToCrouch,
        numpad!(3) => YuyukoMove::ToCrouch,

        numpad!(5) => YuyukoMove::Stand,
        numpad!(5) => YuyukoMove::ToStand
    }
}