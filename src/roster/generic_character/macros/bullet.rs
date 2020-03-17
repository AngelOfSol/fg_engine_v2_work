macro_rules! impl_prune_bullets {
    () => {
        fn prune_bullets(&mut self, play_area: &PlayArea) {
            let bullet_data = &self.data;
            self.state
                .bullets
                .retain(|item| item.alive(bullet_data, play_area));
        }
    };
}

macro_rules! impl_update_bullets {
    () => {
        fn update_bullets(&mut self, play_area: &PlayArea) {
            // first update all active bullets
            for bullet in self.state.bullets.iter_mut() {
                bullet.update(&self.data);
            }

            self.prune_bullets(play_area);

            // then spawn bullets
            let (frame, move_id) = self.state.current_state;
            for spawn in self.data.states[&move_id]
                .bullets
                .iter()
                .filter(|item| item.get_spawn_frame() == frame)
            {
                self.state
                    .bullets
                    .push(spawn.instantiate(self.state.position, self.state.facing));
            }
        }
    };
}
