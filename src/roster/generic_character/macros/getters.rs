macro_rules! impl_collision {
    () => {
        fn collision(&self) -> PositionedHitbox {
            let (frame, move_id) = &self.state.current_state;
            self.data.states[move_id]
                .hitboxes
                .at_time(*frame)
                .collision
                .with_collision_position(self.state.position)
        }
    };
}

macro_rules! impl_hitboxes {
    () => {
        fn hitboxes(&self) -> Vec<PositionedHitbox> {
            let (frame, move_id) = &self.state.current_state;
            self.data.states[move_id]
                .hitboxes
                .at_time(*frame)
                .hitbox
                .iter()
                .map(|data| {
                    data.boxes
                        .iter()
                        .map(|item| {
                            item.with_position_and_facing(self.state.position, self.state.facing)
                        })
                        .collect::<Vec<_>>()
                })
                .flatten()
                .collect()
        }
    };
}

macro_rules! impl_hurtboxes {
    () => {
        fn hurtboxes(&self) -> Vec<PositionedHitbox> {
            let (frame, move_id) = &self.state.current_state;
            self.data.states[move_id]
                .hitboxes
                .at_time(*frame)
                .hurtbox
                .iter()
                .map(|item| item.with_position_and_facing(self.state.position, self.state.facing))
                .collect()
        }
    };
}

macro_rules! impl_get_attack_data {
    () => {
        fn get_attack_data(&self) -> Option<HitInfo> {
            let (frame, move_id) = &self.state.current_state;

            self.data.states[move_id]
                .hitboxes
                .at_time(*frame)
                .hitbox
                .as_ref()
                .and_then(|item| {
                    if let Some(new_hash) = self.state.last_hit_using {
                        let mut hasher = DefaultHasher::new();
                        (move_id, item.id).hash(&mut hasher);
                        let old_hash = hasher.finish();

                        if new_hash == old_hash {
                            return None;
                        }
                    }
                    Some(item)
                })
                .map(|item| {
                    let mut hasher = DefaultHasher::new();
                    (move_id, item.id).hash(&mut hasher);
                    HitInfo::Character {
                        facing: self.state.facing,
                        info: self.data.attacks[&item.data_id].clone(),
                        hit_hash: hasher.finish(),
                    }
                })
        }
    };
}

macro_rules! impl_current_flags {
    () => {
        fn current_flags(&self) -> &Flags {
            let (frame, move_id) = self.state.current_state;
            self.data.states[&move_id].flags.at_time(frame)
        }
    };
}

macro_rules! impl_in_corner {
    () => {
        fn in_corner(&self, play_area: &PlayArea) -> bool {
            let collision = self.collision();
            i32::abs(self.state.position.x) >= play_area.width / 2 - collision.half_size.x
        }
    };
}

macro_rules! impl_position {
    () => {
        fn position(&self) -> collision::Vec2 {
            self.state.position
        }
        fn position_mut(&mut self) -> &mut collision::Vec2 {
            &mut self.state.position
        }
    };
}
macro_rules! impl_velocity {
    () => {
        fn velocity(&self) -> collision::Vec2 {
            self.state.velocity
        }
    };
}

macro_rules! impl_facing {
    () => {
        fn facing(&self) -> Facing {
            self.state.facing
        }
    };
}
