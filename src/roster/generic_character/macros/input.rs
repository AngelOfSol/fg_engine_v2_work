macro_rules! impl_handle_input {
    (fly_start: $fly_start:pat, fly_state: $fly_state:expr, fly_end: $fly_end:expr, border_escape: $border_escape:pat, melee_restitution: $melee_restitution:pat) => {
        fn handle_input(&mut self, input: &[InputState]) {
            let (frame, move_id) = self.state.current_state;
            let cancels = self.data.states[&move_id].cancels.at_time(frame);
            let flags = self.data.states[&move_id].flags.at_time(frame);
            let state_type = self.data.states[&move_id].state_type;

            self.state.current_state = {
                let inputs = read_inputs(
                    input.iter().rev(),
                    self.state.facing,
                    state_type.buffer_window(),
                );
                if move_id == $fly_state {
                    if input.last().unwrap()[Button::A].is_pressed()
                        && input.last().unwrap()[Button::B].is_pressed()
                    {
                        (frame, move_id)
                    } else {
                        (0, $fly_end)
                    }
                } else {
                    let possible_new_move = self
                        .data
                        .command_list
                        .get_commands(&inputs)
                        .into_iter()
                        .copied()
                        .filter(|new_move_id| {
                            let is_not_self = *new_move_id != move_id;

                            let is_allowed_cancel = match self.state.allowed_cancels {
                                AllowedCancel::Hit => cancels
                                    .hit
                                    .contains(&self.data.states[&new_move_id].state_type),
                                AllowedCancel::Block => cancels
                                    .block
                                    .contains(&self.data.states[&new_move_id].state_type),
                                AllowedCancel::Always => false,
                            } || cancels
                                .always
                                .contains(&self.data.states[&new_move_id].state_type)
                                && !cancels.disallow.contains(&new_move_id);

                            let can_rebeat = !self.state.rebeat_chain.contains(&new_move_id);

                            let has_air_actions = self.state.air_actions != 0;

                            let has_required_spirit = self.state.spirit_gauge
                                >= self.data.states[&new_move_id].minimum_spirit_required;

                            let has_required_meter = self.state.meter
                                >= self.data.states[&new_move_id].minimum_meter_required;

                            let in_blockstun = state_type == MoveType::Blockstun;

                            let grounded = !flags.airborne;

                            match *new_move_id {
                                $border_escape => in_blockstun && grounded && has_required_meter,
                                $melee_restitution => {
                                    in_blockstun && grounded && has_required_meter
                                }
                                $fly_start => is_not_self && is_allowed_cancel && has_air_actions,
                                _ => {
                                    is_not_self
                                        && is_allowed_cancel
                                        && can_rebeat
                                        && has_required_spirit
                                        && has_required_meter
                                }
                            }
                        })
                        .fold(None, |acc, item| acc.or(Some(item)))
                        .map(|new_move| (0, new_move));

                    if let Some((_, new_move)) = &possible_new_move {
                        self.on_enter_move(input, *new_move);
                    }

                    possible_new_move.unwrap_or((frame, move_id))
                }
            };
        }
    };
}

macro_rules! impl_on_enter_move {
    (fly_start: $fly_start:pat, jump: $jump:pat, super_jump: $super_jump:pat, border_escape: $border_escape:pat, melee_restitution: $melee_restitution:pat) => {
        fn on_enter_move(&mut self, input: &[InputState], move_id: MoveId) {
            self.state.allowed_cancels = AllowedCancel::Always;
            self.state.last_hit_using = None;
            self.state.rebeat_chain.insert(move_id);

            match move_id {
                $border_escape => {
                    self.state.extra_data = ExtraData::JumpDirection(DirectedAxis::from_facing(
                        input.last().unwrap().axis,
                        self.state.facing,
                    ));
                    // TODO make this cost meter
                }
                $melee_restitution => {
                    // TODO make this cost meter
                }
                $jump | $super_jump => {
                    self.state.extra_data = ExtraData::JumpDirection(DirectedAxis::from_facing(
                        input.last().unwrap().axis,
                        self.state.facing,
                    ));
                }
                $fly_start => {
                    self.state.air_actions -= 1;
                    let mut dir =
                        DirectedAxis::from_facing(input.last().unwrap().axis, self.state.facing);
                    if dir.is_backward() {
                        self.state.facing = self.state.facing.invert();
                        dir = dir.invert();
                    }
                    self.state.extra_data =
                        ExtraData::FlyDirection(if dir == DirectedAxis::Neutral {
                            DirectedAxis::Forward
                        } else {
                            dir
                        });
                }
                _ => (),
            }
        }
    };
}
