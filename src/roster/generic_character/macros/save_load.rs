macro_rules! impl_save {
    () => {
        fn save(&self) -> GameResult<Vec<u8>> {
            bincode::serialize(&self.state).map_err(|_| {
                ggez::GameError::EventLoopError("Saving a player's state had an error.".to_owned())
            })
        }
    };
}

macro_rules! impl_load {
    () => {
        fn load(&mut self, value: &[u8]) -> GameResult<()> {
            self.state = bincode::deserialize(value).map_err(|_| {
                ggez::GameError::EventLoopError("Loading a player's state had an error.".to_owned())
            })?;
            Ok(())
        }
    };
}
