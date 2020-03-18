macro_rules! impl_crush_orb {
    () => {
        fn crush_orb(&mut self) {
            self.state.crushed_orbs += 1;
            self.state.crushed_orbs = i32::min(5, self.state.crushed_orbs);
            // move this to own file/type/function
            self.state.uncrush_timer = match self.state.crushed_orbs {
                1 => 13,
                2 => 8,
                3 => 5,
                4 => 3,
                5 => 1,
                _ => unreachable!(),
            } * 60;
            // TODO move "100" to crushed_orb_value or to max_spirit_gauge / 5
            self.state.spirit_gauge =
                self.data.properties.max_spirit_gauge - self.state.crushed_orbs * 100;
        }
    };
}
