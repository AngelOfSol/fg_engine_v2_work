macro_rules! impl_handle_hitstun {
    (air_idle: $air_idle:expr, stand_idle: $stand_idle:expr, crouch_idle: $crouch_idle:expr) => {
        fn handle_hitstun(&mut self) {
            let (frame, move_id) = self.state.current_state;
            let flags = self.data.states[&move_id].flags.at_time(frame);
            let state_type = self.data.states[&move_id].state_type;

            if state_type.is_stun() {
                let hitstun = self.state.extra_data.unwrap_stun_mut();
                *hitstun -= 1;
                if *hitstun == 0 {
                    if !flags.airborne {
                        self.state.current_state = (
                            0,
                            if flags.crouching {
                                $crouch_idle
                            } else {
                                $stand_idle
                            },
                        );
                    } else {
                        self.state.current_state = if state_type.is_blockstun() {
                            (0, $air_idle)
                        } else {
                            (frame, move_id)
                        };
                    }
                }
            }
        }
    };
}
