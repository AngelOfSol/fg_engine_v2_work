macro_rules! impl_handle_combo_state {
    () => {
        fn handle_combo_state(&mut self) {
            let (_, move_id) = self.state.current_state;
            let current_state_type = self.data.states[&move_id].state_type;
            if !current_state_type.is_stun() {
                self.state.current_combo = None;
            }

            if self.state.current_combo.is_some() {
                self.last_combo_state = Some((self.state.current_combo.clone().unwrap(), 30));
            }
            if self.last_combo_state.is_some() && self.state.current_combo.is_none() {
                let (_, timer) = self.last_combo_state.as_mut().unwrap();
                *timer -= 1;
                if *timer == 0 {
                    self.last_combo_state = None;
                }
            }
        }
    };
}
