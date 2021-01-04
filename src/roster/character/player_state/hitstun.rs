use crate::{
    character::state::components::StateType,
    roster::character::{
        data::Data,
        typedefs::{state::StateConsts, Character, Timed},
    },
};

use super::PlayerState;

impl<C: Character> PlayerState<C> {
    pub fn handle_hitstun(&mut self, data: &Data<C>) {
        let state_data = data.get(self);

        if let Some(ref mut stun) = self.stun {
            assert!(matches!(
                state_data.state_type,
                StateType::Blockstun | StateType::Hitstun
            ));
            *stun -= 1;
            if *stun == 0 {
                self.stun = None;

                if !state_data.flags.airborne {
                    self.current_state = Timed {
                        time: 0,
                        id: if state_data.flags.crouching {
                            C::State::CROUCH
                        } else {
                            C::State::STAND
                        },
                    };
                } else {
                    self.current_state = Timed {
                        time: 0,
                        id: if matches!(state_data.state_type, StateType::Blockstun) {
                            C::State::AIR_IDLE
                        } else {
                            C::State::UNTECH
                        },
                    };
                }
            }
        }
    }

    pub fn in_hitstun(&self, data: &Data<C>) -> bool {
        matches!(data.get(self).state_type, StateType::Hitstun)
    }
}
