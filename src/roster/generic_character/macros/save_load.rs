macro_rules! impl_save {
    () => {
        fn save(&self) -> GameResult<OpaqueStateData> {
            Ok(OpaqueStateData::Yuyuko(self.state.clone()))
        }
    };
}

macro_rules! impl_load {
    () => {
        fn load(&mut self, value: OpaqueStateData) -> GameResult<()> {
            match value {
                OpaqueStateData::Yuyuko(state) => self.state = state,
                _ => panic!("tried to load a different characters data into my own data"),
            }

            Ok(())
        }
    };
}
