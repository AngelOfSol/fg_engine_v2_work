use super::moves::MoveId;
use crate::command_list::CommandList;
use crate::input::button::Button;
#[allow(unused_imports)]
use crate::input::{DirectedAxis, Direction, Input};
use crate::{make_command_list, numpad, read_axis};

pub fn generate_command_list() -> CommandList<MoveId> {
    make_command_list! {
        numpad!(1 C D), numpad!(2 C D), numpad!(3 C D), numpad!(4 C D), numpad!(5 C D), numpad!(6 C D), numpad!(7 C D), numpad!(8 C D), numpad!(9 C D) => MoveId::ChainShift,


        numpad!(1 A B), numpad!(2 A B), numpad!(3 A B), numpad!(4 A B), numpad!(5 A B), numpad!(6 A B), numpad!(7 A B), numpad!(8 A B), numpad!(9 A B) => MoveId::FlyStart,

        numpad!(6 A B), numpad!(66) => MoveId::ForwardDashStart,
        numpad!(4 A B), numpad!(44) => MoveId::BackDash,

        numpad!(1 A B), numpad!(2 A B), numpad!(3 A B) => MoveId::BorderEscapeJump,
        numpad!(4 A B) => MoveId::MeleeRestitution,

        numpad!(5 A), numpad!(4 A), numpad!(6 A), numpad!(7 A), numpad!(8 A), numpad!(9 A) => MoveId::Attack5A,
        numpad!(1 A), numpad!(2 A), numpad!(3 A) => MoveId::Attack2A,


        numpad!(5 B), numpad!(4 B), numpad!(7 B), numpad!(8 B) => MoveId::Attack5B,
        numpad!(1 B), numpad!(2 B) => MoveId::Attack2B,
        numpad!(3 B) => MoveId::Attack3B,
        numpad!(6 B) => MoveId::Attack6B,


        numpad!(5 A), numpad!(4 A), numpad!(6 A), numpad!(1 A), numpad!(2 A), numpad!(3 A) => MoveId::Air5A,
        numpad!(7 A), numpad!(8 A), numpad!(9 A) => MoveId::Air8A,
        numpad!(5 B), numpad!(4 B), numpad!(6 B), numpad!(7 B), numpad!(8 B), numpad!(9 B) => MoveId::Air5B,
        numpad!(1 B), numpad!(2 B), numpad!(3 B) => MoveId::Air2B,


        numpad!(5 C), numpad!(4 C), numpad!(6 C) => MoveId::Attack5C,
        numpad!(2 C), numpad!(1 C), numpad!(3 C) => MoveId::Attack2C,

        numpad!(1 C), numpad!(2 C), numpad!(3 C) => MoveId::Air2C,
        numpad!(5 C), numpad!(4 C), numpad!(6 C), numpad!(7 C), numpad!(8 C), numpad!(9 C) => MoveId::Air5C,

        numpad!(27), numpad!(28), numpad!(29) => MoveId::SuperJump,
        numpad!(1 A B), numpad!(2 A B), numpad!(3 A B) => MoveId::SuperJump,
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
