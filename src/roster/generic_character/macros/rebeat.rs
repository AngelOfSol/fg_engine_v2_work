macro_rules! impl_handle_rebeat_data {
    () => {
        fn handle_rebeat_data(&mut self) {
            let (_, move_id) = self.state.current_state;

            if !self.data.states[&move_id].state_type.is_attack() {
                self.state.rebeat_chain.clear();
            }
        }
    };
}
