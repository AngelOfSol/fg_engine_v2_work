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
                        .play_sound(ChannelName::Hit, GlobalSound::Hit.into());
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

                    self.state
                        .sound_state
                        .play_sound(ChannelName::Hit, GlobalSound::CounterHit.into());
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

                    if self.state.spirit_gauge <= 0 {
                        self.guard_crush(info);
                        self.state
                            .sound_state
                            .play_sound(ChannelName::Hit, GlobalSound::GuardCrush.into());
                    } else {
                        self.state
                            .sound_state
                            .play_sound(ChannelName::Hit, GlobalSound::Block.into());
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
                        self.state
                            .sound_state
                            .play_sound(ChannelName::Hit, GlobalSound::GuardCrush.into());
                    } else {
                        self.state
                            .sound_state
                            .play_sound(ChannelName::Hit, GlobalSound::WrongBlock.into());
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
