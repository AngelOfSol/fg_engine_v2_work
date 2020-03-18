macro_rules! impl_handle_combo_state {
    () => {
        fn handle_combo_state(&mut self) {
            let (_, move_id) = self.state.current_state;
            let current_state_type = self.data.states[&move_id].state_type;
            if !current_state_type.is_stun() {
                self.state.current_combo = None;
            }
        }
    };
}
