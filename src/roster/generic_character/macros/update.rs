macro_rules! impl_update_frame_mut {
    () => {
        fn update_frame_mut(
            &mut self,
            input: &[InputState],
            opponent_position: collision::Vec2,
            play_area: &PlayArea,
            global_particles: &HashMap<GlobalParticle, Particle>,
        ) {
            if self.state.hitstop > 0 {
                self.state.hitstop -= 1;
            } else {
                self.handle_expire();
                self.handle_rebeat_data();
                self.handle_hitstun();
                self.handle_input(input);
                self.update_velocity(play_area);
                self.update_position(play_area);
                self.update_sound();
            }
            self.handle_combo_state();
            self.update_spirit();
            self.update_lockout();
            self.update_meter(opponent_position);
            self.update_particles(global_particles);
            self.update_bullets(play_area);
            self.state.sound_state.update();
            self.state.hitstop = i32::max(0, self.state.hitstop);
        }
    };
}

macro_rules! impl_handle_expire {
    () => {
        fn handle_expire(&mut self) {
            let (frame, move_id) = self.state.current_state;

            // if the next frame would be out of bounds
            self.state.current_state = if frame >= self.data.states[&move_id].duration() - 1 {
                self.state.allowed_cancels = AllowedCancel::Always;
                self.state.last_hit_using = None;
                self.state.rebeat_chain.clear();
                (0, self.data.states[&move_id].on_expire_state)
            } else {
                (frame + 1, move_id)
            };
        }
    };
}
