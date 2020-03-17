macro_rules! impl_update_particles {
    () => {
        fn update_particles(&mut self) {
            let (frame, move_id) = self.state.current_state;
            let particle_data = &self.data.particles;
            let state_particles = &self.data.states[&move_id].particles;

            for (ref mut frame, _, _) in self.state.particles.iter_mut() {
                *frame += 1;
            }
            self.state
                .particles
                .retain(|item| item.0 < particle_data[&item.2].frames.duration());
            for (particle_id, position) in state_particles
                .iter()
                .filter(|item| item.frame == frame)
                .map(|particle| (particle.particle_id, self.state.position + particle.offset))
                .collect::<Vec<_>>()
            {
                self.state.particles.push((0, position, particle_id));
            }
        }
    };
}

macro_rules! impl_spawn_particle {
    () => {
        fn spawn_particle(&mut self, particle: Particle, offset: collision::Vec2) {
            self.state.particles.push((0, offset, particle));
        }
    };
}
