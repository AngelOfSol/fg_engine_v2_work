macro_rules! impl_render_sound {
    () => {
        fn render_sound(
            &mut self,
            audio_device: &Device,
            sound_list: &SoundList<GlobalSound>,
            fps: u32,
        ) -> () {
            self.sound_renderer.render_frame(
                &audio_device,
                &self.data.sounds.data,
                &sound_list.data,
                &self.state.sound_state,
                fps,
            );
        }
    };
}

macro_rules! impl_update_sound {
    () => {
        fn update_sound(&mut self) {
            let (frame, move_id) = self.state.current_state;
            let sounds = &self.data.states[&move_id].sounds;

            for sound in sounds.iter().filter(|item| item.frame == frame) {
                self.state
                    .sound_state
                    .play_sound(sound.channel, sound.name.into());
            }
        }
    };
}
