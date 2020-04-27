macro_rules! impl_update_spirit {
    (fly_end: $fly_end:expr) => {
        fn update_spirit(&mut self) {
            let (ref mut frame, ref mut move_id) = &mut self.state.current_state;
            let move_data = &self.data.states[move_id];
            let flags = move_data.flags.at_time(*frame);

            if move_data.state_type == MoveType::Fly && *move_id != $fly_end {
                self.state.spirit_gauge -= 5; // TODO, move this spirit cost to an editor value
                if self.state.spirit_gauge <= 0 {
                    *move_id = $fly_end;
                    *frame = 0;
                }
            } else {
                self.state.spirit_gauge -= flags.spirit_cost;

                if flags.reset_spirit_delay {
                    self.state.spirit_delay = 0;
                }
                self.state.spirit_delay += flags.spirit_delay;
                self.state.spirit_delay -= 1;
                self.state.spirit_delay = std::cmp::max(self.state.spirit_delay, 0);

                if self.state.spirit_delay == 0 {
                    self.state.spirit_gauge += 5; // TODO: move this spirit regen to an editor value
                }
            }

            self.clamp_spirit();
        }
    };
}

macro_rules! impl_clamp_spirit {
    () => {
        fn clamp_spirit(&mut self) {
            self.state.spirit_gauge = std::cmp::max(
                std::cmp::min(
                    self.state.spirit_gauge,
                    self.data.properties.max_spirit_gauge,
                ),
                0,
            );
        }
    };
}
