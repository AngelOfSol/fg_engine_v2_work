macro_rules! impl_apply_pushback {
    () => {
        fn apply_pushback(&mut self, force: collision::Int) {
            let flags = self.current_flags();
            if !flags.airborne {
                self.state.position.x += force;
            }
        }
    };
}

macro_rules! impl_get_pushback {
    () => {
        fn get_pushback(&self, play_area: &PlayArea) -> collision::Int {
            let (_, move_id) = &self.state.current_state;
            let state = &self.data.states[&move_id];

            if state.state_type.is_stun()
                && self.in_corner(play_area)
                && self.state.hitstop == 0
                && self.state.should_pushback
            {
                -self.state.velocity.x
            } else {
                0
            }
        }
    };
}
