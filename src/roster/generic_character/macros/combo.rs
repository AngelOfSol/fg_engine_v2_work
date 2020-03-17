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

// TODO: change these bools into one 3 element enum
macro_rules! impl_update_combo_state {
    () => {
        fn update_combo_state(&mut self, info: &AttackInfo, guard_crush: bool, counter_hit: bool) {
            self.state.current_combo = Some(match &self.state.current_combo {
                Some(state) => {
                    // 20 is minimum proration
                    let proration = i32::max(info.proration * state.proration / 100, 20);
                    let last_hit_damage = info.hit_damage * state.proration / 100;
                    ComboState {
                        hits: state.hits + 1,
                        total_damage: state.total_damage + last_hit_damage,
                        last_hit_damage,
                        proration,
                        ground_action: info.ground_action,
                        available_limit: state.available_limit - info.limit_cost,
                    }
                }
                None => {
                    let initial_hit_damage = if guard_crush { 0 } else { info.hit_damage };
                    ComboState {
                        hits: 1,
                        total_damage: initial_hit_damage,
                        last_hit_damage: initial_hit_damage,
                        proration: info.proration,
                        ground_action: info.ground_action,
                        available_limit: if counter_hit {
                            info.counter_hit_limit
                        } else {
                            info.starter_limit
                        },
                    }
                }
            });
        }
    };
}
