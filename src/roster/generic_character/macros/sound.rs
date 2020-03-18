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
                &self.data.sounds,
                &sound_list.data,
                &self.state.sound_state,
                fps,
            );
        }
    };
}
