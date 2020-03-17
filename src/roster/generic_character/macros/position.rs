macro_rules! impl_update_velocity {
    (fly_start: $fly_start:expr, fly_state: $fly_state:expr) => {
        fn update_velocity(&mut self, play_area: &PlayArea) {
            let (frame, move_id) = self.state.current_state;
            let flags = self.data.states[&move_id].flags.at_time(frame);

            let base_velocity = if flags.reset_velocity {
                collision::Vec2::zeros()
            } else {
                self.state.velocity
            };

            // we only run gravity if the move doesn't want to reset velocity, because that [resetting velocity] means the move has a trajectory in mind
            let gravity = if !flags.reset_velocity
                && flags.airborne
                && move_id != $fly_start
                && move_id != $fly_state
            {
                collision::Vec2::new(0_00, -0_20)
            } else {
                collision::Vec2::zeros()
            };
            let friction = if !flags.airborne || self.in_corner(play_area) {
                collision::Vec2::new(
                    -i32::min(base_velocity.x.abs(), flags.friction) * i32::signum(base_velocity.x),
                    0_00,
                )
            } else {
                collision::Vec2::zeros()
            };

            let accel = self.state.facing.fix_collision(flags.accel)
                + self
                    .state
                    .facing
                    .fix_collision(Self::handle_fly(move_id, &mut self.state.extra_data))
                + self.state.facing.fix_collision(Self::handle_jump(
                    flags,
                    &self.data.properties,
                    move_id,
                    &mut self.state.extra_data,
                ));
            self.state.velocity = base_velocity + accel + friction + gravity;
        }
    };
}

macro_rules! impl_update_position {
    (knockdown_start: $knockdown_start:expr, hitstun_air: $hitstun_air:expr, stand_idle: $stand_idle:expr) => {
        fn update_position(&mut self, play_area: &PlayArea) {
            let (frame, move_id) = self.state.current_state;
            let state = &self.data.states[&move_id];
            let flags = state.flags.at_time(frame);
            let hitboxes = state.hitboxes.at_time(frame);
            let collision = &hitboxes.collision;

            self.state.position += self.state.velocity;

            // handle landing
            if flags.airborne && self.state.position.y - collision.half_size.y <= -4 {
                let mut reset_hitstun = true;
                let mut reset_velocity = true;
                self.state.current_state = if state.state_type == MoveType::Hitstun {
                    match self.state.current_combo.as_ref().unwrap().ground_action {
                        GroundAction::Knockdown => (0, $knockdown_start),
                        GroundAction::GroundSlam => {
                            self.state.velocity.y *= -1;
                            self.state.current_combo.as_mut().unwrap().ground_action =
                                GroundAction::Knockdown;
                            reset_hitstun = false;
                            reset_velocity = false;
                            (0, $hitstun_air)
                        }
                        GroundAction::OnTheGround => (0, $knockdown_start),
                    }
                } else {
                    (0, $stand_idle)
                };
                if reset_hitstun {
                    self.state.extra_data = ExtraData::None;
                }
                if reset_velocity {
                    self.state.velocity = collision::Vec2::zeros();
                }
                self.state.position.y = hitboxes.collision.half_size.y;
                self.state.air_actions = self.data.properties.max_air_actions;
            }

            // handle stage sides
            if i32::abs(self.state.position.x) > play_area.width / 2 - collision.half_size.x {
                self.state.position.x = i32::signum(self.state.position.x)
                    * (play_area.width / 2 - collision.half_size.x);
            }

            // if not airborne, make sure the character is locked to the ground properly
            if !flags.airborne {
                self.state.position.y = hitboxes.collision.half_size.y;
            }
        }
    };
}

macro_rules! impl_handle_refacing {
    () => {
        fn handle_refacing(&mut self, other_player: collision::Int) {
            let flags = self.current_flags();
            if flags.allow_reface {
                self.state.facing = if self.state.position.x > other_player
                    && self.state.facing == Facing::Right
                {
                    Facing::Left
                } else if self.state.position.x < other_player && self.state.facing == Facing::Left
                {
                    Facing::Right
                } else {
                    self.state.facing
                }
            }
        }
    };
}
