macro_rules! impl_in_corner {
    () => {
        fn in_corner(&self, play_area: &PlayArea) -> bool {
            let collision = self.collision();
            i32::abs(self.state.position.x) >= play_area.width / 2 - collision.half_size.x
        }
    };
}
macro_rules! impl_apply_pushback {
    () => {
        fn apply_pushback(&mut self, force: collision::Int) {
            let flags = self.current_flags();
            if !flags.airborne {
                self.state.position.x += force;
            }
        }
    };
}

macro_rules! impl_get_pushback {
    () => {
        fn get_pushback(&self, play_area: &PlayArea) -> collision::Int {
            let (_, move_id) = &self.state.current_state;
            let state = &self.data.states[&move_id];

            if state.state_type.is_stun()
                && self.in_corner(play_area)
                && self.state.hitstop == 0
                && self.state.should_pushback
            {
                -self.state.velocity.x
            } else {
                0
            }
        }
    };
}

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

macro_rules! impl_prune_bullets {
    () => {
        fn prune_bullets(&mut self, play_area: &PlayArea) {
            let bullet_data = &self.data;
            self.state
                .bullets
                .retain(|item| item.alive(bullet_data, play_area));
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

macro_rules! impl_would_be_hit {
    () => {
        fn would_be_hit(
            &self,
            input: &[InputState],
            touched: bool,
            total_info: Option<HitInfo>,
        ) -> HitType {
            if !touched
                || total_info.is_none()
                || self
                    .state
                    .current_combo
                    .as_ref()
                    .map(|item| item.available_limit <= 0)
                    .unwrap_or(false)
            {
                return HitType::Whiff;
            }
            let total_info = total_info.unwrap();

            let info = match &total_info {
                HitInfo::Character { info, .. } => info,
                HitInfo::Bullet(info, _) => info,
            };

            let flags = self.current_flags();
            let state_type = self.data.states[&self.state.current_state.1].state_type;
            let axis = DirectedAxis::from_facing(input.last().unwrap().axis, self.state.facing);
            let counter_hit = flags.can_be_counter_hit && info.can_counter_hit;

            if !info.melee && flags.bullet.is_invuln() || info.melee && flags.melee.is_invuln() {
                HitType::Whiff
            } else if info.grazeable && flags.grazing {
                HitType::Graze(total_info)
            } else if info.air_unblockable && flags.airborne {
                if counter_hit {
                    HitType::CounterHit(total_info)
                } else {
                    HitType::Hit(total_info)
                }
            } else if state_type.is_blockstun() || (flags.can_block && axis.is_backward()) {
                if flags.airborne || axis.is_blocking(info.guard) {
                    HitType::Block(total_info)
                } else {
                    HitType::WrongBlock(total_info)
                }
            } else if counter_hit {
                HitType::CounterHit(total_info)
            } else {
                HitType::Hit(total_info)
            }
        }
    };
}

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

macro_rules! impl_take_hit {
    (hitstun_air: $hitstun_air:expr, hitstun_ground: $hitstun_ground:expr,
        blockstun_air: $blockstun_air:expr, blockstun_stand: $blockstun_stand:expr, blockstun_crouch: $blockstun_crouch:expr,
        wrongblock_stand: $wrongblock_stand:expr, wrongblock_crouch: $wrongblock_crouch:expr) => {
        fn take_hit(&mut self, info: &HitType) {
            let flags = self.current_flags();

            match info {
                HitType::Hit(info) => {
                    let hit_direction = info.get_facing();
                    let attack_data = info.get_attack_data();

                    let on_hit = &attack_data.on_hit;
                    if flags.airborne || attack_data.launcher {
                        self.state.current_state = (0, $hitstun_air);
                        self.state.velocity = hit_direction.fix_collision(on_hit.air_force);
                    } else {
                        self.state.current_state = (0, $hitstun_ground);
                        self.state.velocity = hit_direction
                            .fix_collision(collision::Vec2::new(on_hit.ground_pushback, 0_00));
                    }
                    self.state.extra_data = ExtraData::Stun(attack_data.level.hitstun());
                    self.state.hitstop = on_hit.defender_stop;
                    self.state.should_pushback = info.should_pushback();

                    self.update_combo_state(&attack_data, false, false);
                    let current_combo = self.state.current_combo.as_ref().unwrap();

                    self.state.health -= current_combo.last_hit_damage;
                    self.state
                        .sound_state
                        .play_sound(ChannelName::Hit, GlobalSound::Block.into());
                }
                HitType::CounterHit(info) => {
                    let hit_direction = info.get_facing();
                    let attack_data = info.get_attack_data();

                    let on_hit = &attack_data.on_hit;
                    if flags.airborne || attack_data.launcher {
                        self.state.current_state = (0, $hitstun_air);
                        self.state.velocity = hit_direction.fix_collision(on_hit.air_force);
                    } else {
                        self.state.current_state = (0, $hitstun_ground);
                        self.state.velocity = hit_direction
                            .fix_collision(collision::Vec2::new(on_hit.ground_pushback, 0_00));
                    }
                    self.state.extra_data = ExtraData::Stun(attack_data.level.counter_hitstun());
                    self.state.hitstop = on_hit.defender_stop;
                    self.state.should_pushback = info.should_pushback();

                    self.update_combo_state(&attack_data, false, true);
                    let current_combo = self.state.current_combo.as_ref().unwrap();
                    self.state.health -= current_combo.last_hit_damage;
                }
                HitType::Block(info) => {
                    let hit_direction = info.get_facing();
                    let attack_data = info.get_attack_data();

                    let on_block = &attack_data.on_block;
                    if flags.airborne {
                        self.state.current_state = (0, $blockstun_air);
                        self.state.velocity = hit_direction.fix_collision(on_block.air_force);
                    } else {
                        self.state.current_state = (
                            0,
                            if flags.crouching {
                                $blockstun_crouch
                            } else {
                                $blockstun_stand
                            },
                        );
                        self.state.velocity = hit_direction
                            .fix_collision(collision::Vec2::new(on_block.ground_pushback, 0_00));
                    }

                    self.state.spirit_gauge -= attack_data.spirit_cost;
                    self.state.spirit_gauge = i32::max(0, self.state.spirit_gauge);
                    if attack_data.reset_spirit_delay {
                        self.state.spirit_delay = 0;
                    }
                    self.state.spirit_delay += attack_data.spirit_delay;

                    self.state.extra_data = ExtraData::Stun(attack_data.level.blockstun());
                    self.state.hitstop = on_block.defender_stop;
                    self.state.should_pushback = info.should_pushback();
                    self.state.health -= attack_data.chip_damage;

                    self.state
                        .sound_state
                        .play_sound(ChannelName::Hit, GlobalSound::Block.into());

                    if self.state.spirit_gauge <= 0 {
                        self.guard_crush(info);
                    }
                }
                HitType::WrongBlock(info) => {
                    let hit_direction = info.get_facing();
                    let attack_data = info.get_attack_data();

                    let on_block = &attack_data.on_block;
                    self.state.current_state = (
                        0,
                        if flags.crouching {
                            $wrongblock_crouch
                        } else {
                            $wrongblock_stand
                        },
                    );
                    self.state.velocity = hit_direction
                        .fix_collision(collision::Vec2::new(on_block.ground_pushback, 0_00));

                    self.state.spirit_delay = attack_data.level.wrongblock_delay();
                    self.state.spirit_gauge -= attack_data.level.wrongblock_cost();
                    self.state.spirit_gauge = i32::max(0, self.state.spirit_gauge);

                    self.state.extra_data = ExtraData::Stun(attack_data.level.wrongblockstun());
                    self.state.hitstop = on_block.defender_stop;
                    self.state.should_pushback = info.should_pushback();
                    self.state.health -= attack_data.chip_damage;

                    if self.state.spirit_gauge <= 0 {
                        self.guard_crush(info);
                    }
                }
                HitType::Whiff | HitType::Graze(_) => {}
            }
        }
    };
}

macro_rules! impl_deal_hit {
    (on_hit_particle: $on_hit_particle:expr) => {
        fn deal_hit(&mut self, info: &HitType) {
            let boxes = self.hitboxes();

            match info {
                HitType::Hit(info) | HitType::CounterHit(info) => {
                    if let Some(last_hit) = info.get_hit_by_data() {
                        self.state.last_hit_using = Some(last_hit);
                    }
                    let info = info.get_attack_data();
                    let on_hit = &info.on_hit;

                    self.state.hitstop = on_hit.attacker_stop;
                    self.state.allowed_cancels = AllowedCancel::Hit;

                    if !boxes.is_empty() {
                        // TODO improve hit effect particle spawning determination
                        let spawn_point = boxes
                            .iter()
                            .fold(collision::Vec2::zeros(), |acc, item| acc + item.center)
                            / boxes.len() as i32;
                        self.spawn_particle($on_hit_particle, spawn_point);
                    }
                }
                HitType::Block(info) | HitType::WrongBlock(info) => {
                    if let Some(last_hit) = info.get_hit_by_data() {
                        self.state.last_hit_using = Some(last_hit);
                    }
                    let info = info.get_attack_data();
                    let on_block = &info.on_block;

                    self.state.allowed_cancels = AllowedCancel::Block;
                    self.state.hitstop = on_block.attacker_stop;
                }
                HitType::Whiff | HitType::Graze(_) => {}
            }
        }
    };
}

macro_rules! impl_handle_fly {
    (fly_start: $fly_start:expr) => {
        fn handle_fly(move_id: MoveId, extra_data: &mut ExtraData) -> collision::Vec2 {
            if move_id == $fly_start {
                let fly_dir = extra_data.unwrap_fly_direction();
                *extra_data = ExtraData::FlyDirection(fly_dir);
                let speed = match fly_dir {
                    DirectedAxis::Forward => collision::Vec2::new(1_00, 0_00),
                    DirectedAxis::UpForward => collision::Vec2::new(0_71, 0_71),
                    DirectedAxis::DownForward => collision::Vec2::new(0_71, -0_71),
                    DirectedAxis::Backward => collision::Vec2::new(-1_00, 0_00),
                    DirectedAxis::UpBackward => collision::Vec2::new(-0_71, 0_71),
                    DirectedAxis::DownBackward => collision::Vec2::new(-0_71, -0_71),
                    DirectedAxis::Up => collision::Vec2::new(0_00, 1_00),
                    DirectedAxis::Down => collision::Vec2::new(0_00, -1_00),
                    _ => unreachable!(),
                };
                3 * speed / 4
            } else {
                collision::Vec2::zeros()
            }
        }
    };
}
macro_rules! impl_handle_jump {
    (jump: $jump:pat, super_jump: $super_jump:pat, border_escape: $border_escape:pat) => {
        fn handle_jump(
            flags: &Flags,
            data: &Properties,
            move_id: MoveId,
            extra_data: &mut ExtraData,
        ) -> collision::Vec2 {
            if flags.jump_start {
                let axis = extra_data.unwrap_jump_direction();
                *extra_data = ExtraData::None;
                match move_id {
                    $jump => {
                        if !axis.is_horizontal() {
                            data.neutral_jump_accel
                        } else {
                            data.directed_jump_accel
                                .component_mul(&collision::Vec2::new(
                                    axis.direction_multiplier(true),
                                    1,
                                ))
                        }
                    }
                    $super_jump | $border_escape => {
                        if !axis.is_horizontal() {
                            data.neutral_super_jump_accel
                        } else {
                            data.directed_super_jump_accel
                                .component_mul(&collision::Vec2::new(
                                    axis.direction_multiplier(true),
                                    1,
                                ))
                        }
                    }
                    _ => panic!("jump_start not allowed on non jump moves"),
                }
            } else {
                collision::Vec2::zeros()
            }
        }
    };
}

macro_rules! impl_handle_combo_state {
    () => {
        fn handle_combo_state(&mut self) {
            let (_, move_id) = self.state.current_state;
            let current_state_type = self.data.states[&move_id].state_type;
            if !current_state_type.is_stun() {
                self.state.current_combo = None;
            }
        }
    };
}

macro_rules! impl_handle_rebeat_data {
    () => {
        fn handle_rebeat_data(&mut self) {
            let (_, move_id) = self.state.current_state;

            if !self.data.states[&move_id].state_type.is_attack() {
                self.state.rebeat_chain.clear();
            }
        }
    };
}

// TODO: change these bools into one 3 element enum
macro_rules! impl_update_combo_state {
    () => {
        fn update_combo_state(&mut self, info: &AttackInfo, guard_crush: bool, counter_hit: bool) {
            self.state.current_combo = Some(match &self.state.current_combo {
                Some(state) => {
                    // 20 is minimum proration
                    let proration = i32::max(info.proration * state.proration / 100, 20);
                    let last_hit_damage = info.hit_damage * state.proration / 100;
                    ComboState {
                        hits: state.hits + 1,
                        total_damage: state.total_damage + last_hit_damage,
                        last_hit_damage,
                        proration,
                        ground_action: info.ground_action,
                        available_limit: state.available_limit - info.limit_cost,
                    }
                }
                None => {
                    let initial_hit_damage = if guard_crush { 0 } else { info.hit_damage };
                    ComboState {
                        hits: 1,
                        total_damage: initial_hit_damage,
                        last_hit_damage: initial_hit_damage,
                        proration: info.proration,
                        ground_action: info.ground_action,
                        available_limit: if counter_hit {
                            info.counter_hit_limit
                        } else {
                            info.starter_limit
                        },
                    }
                }
            });
        }
    };
}

macro_rules! impl_handle_expire {
    () => {
        fn handle_expire(&mut self) {
            let (frame, move_id) = self.state.current_state;

            // if the next frame would be out of bounds
            self.state.current_state = if frame >= self.data.states[&move_id].duration() - 1 {
                self.state.allowed_cancels = AllowedCancel::Always;
                self.state.last_hit_using = None;
                self.state.rebeat_chain.clear();
                (0, self.data.states[&move_id].on_expire_state)
            } else {
                (frame + 1, move_id)
            };
        }
    };
}

macro_rules! impl_handle_hitstun {
    (air_idle: $air_idle:expr, stand_idle: $stand_idle:expr, crouch_idle: $crouch_idle:expr) => {
        fn handle_hitstun(&mut self) {
            let (frame, move_id) = self.state.current_state;
            let flags = self.data.states[&move_id].flags.at_time(frame);
            let state_type = self.data.states[&move_id].state_type;

            if state_type.is_stun() {
                let hitstun = self.state.extra_data.unwrap_stun_mut();
                *hitstun -= 1;
                if *hitstun == 0 {
                    if !flags.airborne {
                        self.state.current_state = (
                            0,
                            if flags.crouching {
                                $crouch_idle
                            } else {
                                $stand_idle
                            },
                        );
                    } else {
                        self.state.current_state = if state_type.is_blockstun() {
                            (0, $air_idle)
                        } else {
                            (frame, move_id)
                        };
                    }
                }
            }
        }
    };
}

macro_rules! impl_handle_input {
    (fly_start: $fly_start:pat, fly_state: $fly_state:expr, fly_end: $fly_end:expr, border_escape: $border_escape:pat, melee_restitution: $melee_restitution:pat) => {
        fn handle_input(&mut self, input: &[InputState]) {
            let (frame, move_id) = self.state.current_state;
            let cancels = self.data.states[&move_id].cancels.at_time(frame);
            let flags = self.data.states[&move_id].flags.at_time(frame);
            let state_type = self.data.states[&move_id].state_type;

            self.state.current_state = {
                let inputs = read_inputs(input.iter().rev(), self.state.facing);
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

                            let in_blockstun = state_type == MoveType::Blockstun;

                            let grounded = !flags.airborne;

                            match *new_move_id {
                                $border_escape => in_blockstun && grounded,
                                $melee_restitution => in_blockstun && grounded,
                                $fly_start => is_not_self && is_allowed_cancel && has_air_actions,
                                _ => {
                                    is_not_self
                                        && is_allowed_cancel
                                        && can_rebeat
                                        && has_required_spirit
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
                    self.crush_orb();
                }
                $melee_restitution => {
                    self.crush_orb();
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

macro_rules! impl_update_bullets {
    () => {
        fn update_bullets(&mut self, play_area: &PlayArea) {
            // first update all active bullets
            for bullet in self.state.bullets.iter_mut() {
                bullet.update(&self.data);
            }

            self.prune_bullets(play_area);

            // then spawn bullets
            let (frame, move_id) = self.state.current_state;
            for spawn in self.data.states[&move_id]
                .bullets
                .iter()
                .filter(|item| item.get_spawn_frame() == frame)
            {
                self.state
                    .bullets
                    .push(spawn.instantiate(self.state.position, self.state.facing));
            }
        }
    };
}

macro_rules! impl_update_spirit {
    (fly_end: $fly_end:expr) => {
        fn update_spirit(&mut self) {
            let (ref mut frame, ref mut move_id) = &mut self.state.current_state;
            let move_data = &self.data.states[move_id];
            let flags = move_data.flags.at_time(*frame);

            if move_data.state_type == MoveType::Fly {
                self.state.spirit_gauge -= 10; // TODO, move this spirit cost to an editor value
                if self.state.spirit_gauge == 0 {
                    *move_id = $fly_end;
                    *frame = 0;
                }
            } else {
                self.state.spirit_gauge -= flags.spirit_cost;

                if flags.reset_spirit_delay {
                    self.state.spirit_delay = 0;
                }
                self.state.spirit_delay += flags.spirit_delay;
                self.state.spirit_delay -= 1;
                self.state.spirit_delay = std::cmp::max(self.state.spirit_delay, 0);

                if self.state.spirit_delay == 0 {
                    self.state.spirit_gauge += 5; // TODO: move this spirit regen to an editor value
                }
            }

            if self.state.crushed_orbs > 0 {
                self.state.uncrush_timer -= 1;
                if self.state.uncrush_timer <= 0 {
                    self.state.crushed_orbs -= 1;
                    self.state.uncrush_timer = match self.state.crushed_orbs {
                        0 => 0,
                        1 => 13,
                        2 => 8,
                        3 => 5,
                        4 => 3,
                        _ => unreachable!(),
                    } * 60;
                }
            }

            self.clamp_spirit();
        }
    };
}

macro_rules! impl_clamp_spirit {
    () => {
        fn clamp_spirit(&mut self) {
            self.state.spirit_gauge = std::cmp::max(
                std::cmp::min(
                    self.state.spirit_gauge,
                    self.data.properties.max_spirit_gauge - self.state.crushed_orbs * 100,
                ),
                0,
            );
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

macro_rules! impl_update_frame_mut {
    () => {
        fn update_frame_mut(&mut self, input: &[InputState], play_area: &PlayArea) {
            if self.state.hitstop > 0 {
                self.state.hitstop -= 1;
            } else {
                self.handle_expire();
                self.handle_rebeat_data();
                self.handle_hitstun();
                self.handle_input(input);
                self.update_velocity(play_area);
                self.update_position(play_area);
            }
            self.handle_combo_state();
            self.update_spirit();
            self.update_particles();
            self.update_bullets(play_area);
            self.state.sound_state.update();
            self.state.hitstop = i32::max(0, self.state.hitstop);
        }
    };
}

macro_rules! impl_draw_ui {
    () => {
        fn draw_ui(&self, ctx: &mut Context, bottom_line: graphics::Matrix4) -> GameResult<()> {
            ggez::graphics::set_transform(ctx, bottom_line);
            ggez::graphics::apply_transformations(ctx)?;
            ggez::graphics::set_blend_mode(ctx, ggez::graphics::BlendMode::Alpha)?;

            let spirit_current = ggez::graphics::Rect::new(
                0.0,
                0.0,
                100.0 * self.state.spirit_gauge as f32
                    / self.data.properties.max_spirit_gauge as f32,
                20.0,
            );
            let spirit_backdrop = ggez::graphics::Rect::new(0.0, 0.0, 100.0, 20.0);
            let spirit_max = ggez::graphics::Rect::new(-5.0, -5.0, 110.0, 30.0);

            let rect = ggez::graphics::Mesh::new_rectangle(
                ctx,
                ggez::graphics::DrawMode::Fill(ggez::graphics::FillOptions::default()),
                spirit_max,
                ggez::graphics::Color::new(0.0, 0.0, 0.0, 1.0),
            )?;

            ggez::graphics::draw(ctx, &rect, ggez::graphics::DrawParam::default())?;

            let rect = ggez::graphics::Mesh::new_rectangle(
                ctx,
                ggez::graphics::DrawMode::Fill(ggez::graphics::FillOptions::default()),
                spirit_backdrop,
                ggez::graphics::Color::new(1.0, 1.0, 1.0, 1.0),
            )?;

            ggez::graphics::draw(ctx, &rect, ggez::graphics::DrawParam::default())?;

            let rect = ggez::graphics::Mesh::new_rectangle(
                ctx,
                ggez::graphics::DrawMode::Fill(ggez::graphics::FillOptions::default()),
                spirit_current,
                ggez::graphics::Color::new(0.0, 0.0, 1.0, 1.0),
            )?;

            ggez::graphics::draw(ctx, &rect, ggez::graphics::DrawParam::default())?;

            // draw HP bar

            ggez::graphics::set_transform(
                ctx,
                graphics::Matrix4::new_translation(&graphics::Vec3::new(0.0, -400.0, 0.0))
                    * bottom_line,
            );
            ggez::graphics::apply_transformations(ctx)?;

            let hp_length = 300.0;
            let hp_current = ggez::graphics::Rect::new(
                0.0,
                0.0,
                hp_length * self.state.health as f32 / self.data.properties.health as f32,
                20.0,
            );
            let hp_backdrop = ggez::graphics::Rect::new(0.0, 0.0, hp_length, 20.0);
            let hp_max = ggez::graphics::Rect::new(-5.0, -5.0, hp_length + 10.0, 30.0);

            let rect = ggez::graphics::Mesh::new_rectangle(
                ctx,
                ggez::graphics::DrawMode::Fill(ggez::graphics::FillOptions::default()),
                hp_max,
                ggez::graphics::Color::new(0.0, 0.0, 0.0, 1.0),
            )?;

            ggez::graphics::draw(ctx, &rect, ggez::graphics::DrawParam::default())?;

            let rect = ggez::graphics::Mesh::new_rectangle(
                ctx,
                ggez::graphics::DrawMode::Fill(ggez::graphics::FillOptions::default()),
                hp_backdrop,
                ggez::graphics::Color::new(1.0, 1.0, 1.0, 1.0),
            )?;

            ggez::graphics::draw(ctx, &rect, ggez::graphics::DrawParam::default())?;

            let rect = ggez::graphics::Mesh::new_rectangle(
                ctx,
                ggez::graphics::DrawMode::Fill(ggez::graphics::FillOptions::default()),
                hp_current,
                ggez::graphics::Color::new(0.0, 1.0, 0.0, 1.0),
            )?;

            ggez::graphics::draw(ctx, &rect, ggez::graphics::DrawParam::default())?;

            Ok(())
        }
    };
}

macro_rules! impl_draw {
    () => {
        fn draw(&self, ctx: &mut Context, world: graphics::Matrix4) -> GameResult<()> {
            let (frame, move_id) = self.state.current_state;

            let collision = &self.data.states[&move_id].hitboxes.at_time(frame).collision;
            let position = world
                * graphics::Matrix4::new_translation(&graphics::up_dimension(
                    self.state.position.into_graphical(),
                ));

            self.data.states[&move_id].draw_at_time(
                ctx,
                &self.data.assets,
                frame,
                position
                    * graphics::Matrix4::new_translation(&graphics::up_dimension(
                        self.state
                            .facing
                            .fix_graphics(-collision.center.into_graphical()),
                    ))
                    * graphics::Matrix4::new_nonuniform_scaling(&graphics::up_dimension(
                        self.state.facing.graphics_multiplier(),
                    )),
            )?;

            Ok(())
        }
    };
}

macro_rules! impl_draw_particles {
    () => {
        fn draw_particles(&self, ctx: &mut Context, world: graphics::Matrix4) -> GameResult<()> {
            for (frame, position, id) in &self.state.particles {
                self.data.particles[&id].draw_at_time(
                    ctx,
                    &self.data.assets,
                    *frame,
                    world
                        * graphics::Matrix4::new_translation(&graphics::up_dimension(
                            position.into_graphical(),
                        )),
                )?;
            }

            Ok(())
        }
    };
}

macro_rules! impl_draw_bullets {
    () => {
        fn draw_bullets(&self, ctx: &mut Context, world: graphics::Matrix4) -> GameResult<()> {
            for bullet in &self.state.bullets {
                bullet.draw(ctx, &self.data, &self.data.assets, world)?;
            }

            Ok(())
        }
    };
}

macro_rules! impl_draw_shadow {
    () => {
        fn draw_shadow(&self, ctx: &mut Context, world: graphics::Matrix4) -> GameResult<()> {
            let (frame, move_id) = self.state.current_state;

            let collision = &self.data.states[&move_id].hitboxes.at_time(frame).collision;
            let position = world
                * graphics::Matrix4::new_translation(&graphics::up_dimension(
                    self.state.position.into_graphical(),
                ));

            self.data.states[&move_id].draw_shadow_at_time(
                ctx,
                &self.data.assets,
                frame,
                position
                    * graphics::Matrix4::new_translation(&graphics::up_dimension(
                        self.state
                            .facing
                            .fix_graphics(-collision.center.into_graphical()),
                    ))
                    * graphics::Matrix4::new_nonuniform_scaling(&graphics::up_dimension(
                        self.state.facing.graphics_multiplier(),
                    )),
            )?;
            Ok(())
        }
    };
}

macro_rules! impl_getters {
    () => {
        fn position(&self) -> collision::Vec2 {
            self.state.position
        }
        fn position_mut(&mut self) -> &mut collision::Vec2 {
            &mut self.state.position
        }

        fn velocity(&self) -> collision::Vec2 {
            self.state.velocity
        }
        fn facing(&self) -> Facing {
            self.state.facing
        }
    };
}
