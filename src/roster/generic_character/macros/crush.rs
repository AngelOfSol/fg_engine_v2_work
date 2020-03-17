macro_rules! impl_guard_crush {
    (hitstun_air: $hitstun_air:expr, hitstun_ground: $hitstun_ground:expr) => {
        fn guard_crush(&mut self, info: &HitInfo) {
            if self.state.spirit_gauge <= 0 {
                let attack_data = info.get_attack_data();
                let flags = self.current_flags();
                let hit_direction = info.get_facing();
                let on_hit = &attack_data.on_hit;
                // guard crush time!!!!!!!!!!
                if flags.airborne {
                    self.state.current_state = (0, $hitstun_air);
                    //TODO crush velocity mutliplier
                    self.state.velocity = hit_direction.fix_collision(on_hit.air_force) * 3;
                } else {
                    self.state.current_state = (0, $hitstun_ground);
                }
                self.state.extra_data = ExtraData::Stun(attack_data.level.crush_stun());
                self.update_combo_state(&attack_data, true, false);

                self.crush_orb();
            }
        }
    };
}

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
