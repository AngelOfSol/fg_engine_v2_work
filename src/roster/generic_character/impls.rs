use crate::character::state::components::{Flags, MoveType};
use crate::input::Facing;
use crate::roster::generic_character::combo_state::ComboState;
use crate::typedefs::collision;

pub fn handle_refacing(
    facing: &mut Facing,
    flags: &Flags,
    position: &collision::Vec2,
    other_player: collision::Int,
) {
    if flags.allow_reface {
        *facing = if position.x > other_player && *facing == Facing::Right {
            Facing::Left
        } else if position.x < other_player && *facing == Facing::Left {
            Facing::Right
        } else {
            *facing
        }
    }
}
pub fn handle_combo_state(
    current_combo: &mut Option<ComboState>,
    last_combo_state: &mut Option<(ComboState, usize)>,
    current_state_type: MoveType,
) {
    if !current_state_type.is_stun() {
        *current_combo = None;
    }

    if current_combo.is_some() {
        *last_combo_state = Some((current_combo.clone().unwrap(), 30));
    }
    if last_combo_state.is_some() && current_combo.is_none() {
        let (_, timer) = last_combo_state.as_mut().unwrap();
        *timer -= 1;
        if *timer == 0 {
            *last_combo_state = None;
        }
    }
}
