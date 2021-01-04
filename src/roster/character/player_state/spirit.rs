use crate::roster::character::{
    data::Data,
    typedefs::{state::StateConsts, Character, Timed},
};

use super::PlayerState;

impl<C: Character> PlayerState<C> {
    pub fn update_spirit(&mut self, data: &Data<C>) {
        let state_data = data.get(self);
        let Timed {
            ref mut time,
            ref mut id,
        } = &mut self.current_state;

        if *id == C::State::FLY {
            self.spirit_gauge -= 5; // TODO, move this spirit cost to an editor value
            if self.spirit_gauge <= 0 {
                *id = C::State::FLY_END;
                *time = 0;
            }
        } else {
            self.spirit_gauge -= state_data.flags.spirit_cost;

            if state_data.flags.reset_spirit_delay {
                self.spirit_delay = 0;
            }
            self.spirit_delay += state_data.flags.spirit_delay;
            self.spirit_delay -= 1;
            self.spirit_delay = std::cmp::max(self.spirit_delay, 0);

            if self.spirit_delay == 0 {
                self.spirit_gauge += 5; // TODO: move this spirit regen to an editor value
            }
        }

        self.clamp_spirit(data);
    }

    fn clamp_spirit(&mut self, data: &Data<C>) {
        self.spirit_gauge = std::cmp::max(
            std::cmp::min(self.spirit_gauge, data.properties.max_spirit_gauge),
            0,
        );
    }
}
