pub mod attacks;
pub mod commands;
pub mod data;
pub mod graphic;
pub mod inputs;
pub mod moves;
pub mod sounds;
pub mod state;

use super::{
    hit_info::{
        block, counter_hit, graze, guard_crush, hit, wrong_block, ComboEffect, HitEffect,
        HitResult, HitType, Source,
    },
    OpponentState,
};
use crate::character::state::components::GlobalGraphic;
use crate::game_match::sounds::PlayerSoundState;
use crate::graphics::animation_group::AnimationGroup;
use crate::hitbox::PositionedHitbox;
use crate::input::button::Button;
use crate::input::{read_inputs, DirectedAxis, Facing, InputState};
use crate::roster::generic_character::hit_info::Force;
use crate::roster::generic_character::AllowedCancel;
use crate::roster::generic_character::GenericCharacterBehaviour;
use crate::roster::generic_character::OpaqueStateData;
use crate::typedefs::collision;
use crate::typedefs::collision::IntoGraphical;
use crate::typedefs::graphics;
use crate::{assets::Assets, game_object::constructors::Construct};
use crate::{
    character::command::Requirement,
    game_match::sounds::{ChannelName, GlobalSound, SoundRenderer},
};
use crate::{character::components::Properties, game_object::constructors::Constructor};
use crate::{
    character::components::{AttackInfo, GroundAction},
    game_object::state::Position,
};
use crate::{
    character::state::components::StateType,
    game_match::{FlashType, PlayArea, UiElements},
};
use crate::{game_match::sounds::GlobalSoundList, game_object::state::Timer};

use crate::{game_match::sounds::SoundPath, game_object::state::ExpiresAfterAnimation};
use attacks::AttackDataMap;
use commands::{CommandId, CommandMap};
use ggez::{Context, GameResult};
use graphic::{GraphicMap, StateGraphicMap, YuyukoGraphic};
use hecs::{EntityBuilder, World};
use inputs::InputMap;
use inspect_design::Inspect;
use moves::MoveId;
use rodio::Device;
use serde::Deserialize;
use serde::Serialize;
use sounds::{SoundId, SoundList};
use state::{PlayerState, StateDataMap, StateInstant};
use std::fs::File;
use std::hash::Hash;
use std::io::BufReader;
use std::path::PathBuf;
use std::rc::Rc;
use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
};
use strum::IntoEnumIterator;
use strum::{Display, EnumIter};

const MAX_FALLING_VELOCITY: collision::Int = -8_00;

#[derive(Clone, Inspect)]
pub struct Yuyuko {
    #[tab = "States"]
    pub states: StateDataMap,
    #[tab = "Attacks"]
    pub attacks: AttackDataMap,
    #[tab = "Properties"]
    pub properties: Properties,
    #[skip]
    pub sounds: SoundList,
    #[tab = "Graphics"]
    pub graphics: GraphicMap,
    #[tab = "Inputs"]
    pub input_map: InputMap,
    #[tab = "Commands"]
    pub command_map: CommandMap,
    #[tab = "State to Graphics"]
    pub state_graphics_map: StateGraphicMap,
}

impl std::fmt::Debug for Yuyuko {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.states)
    }
}

impl Yuyuko {
    pub fn get(&self, current_state: (usize, MoveId)) -> StateInstant<'_> {
        self.states[&current_state.1].get(current_state.0)
    }

    pub fn new_with_path(
        ctx: &mut Context,
        assets: &mut Assets,
        path: PathBuf,
    ) -> GameResult<Yuyuko> {
        let data = YuyukoData::load_from_json(ctx, assets, path)?;
        Ok(Yuyuko {
            states: data.states,
            properties: data.properties,
            attacks: data.attacks,
            sounds: data.sounds,
            graphics: data.graphics,
            command_map: data.command_map,
            input_map: data.input_map,
            state_graphics_map: data.state_graphics_map,
        })
    }
}

#[derive(Deserialize)]
pub struct YuyukoData {
    states: StateDataMap,
    properties: Properties,
    attacks: AttackDataMap,
    #[serde(skip)]
    #[serde(default = "SoundList::new")]
    sounds: SoundList,
    graphics: GraphicMap,
    input_map: InputMap,
    command_map: CommandMap,
    state_graphics_map: StateGraphicMap,
}
impl YuyukoData {
    fn load_from_json(
        ctx: &mut Context,
        assets: &mut Assets,
        mut path: PathBuf,
    ) -> GameResult<YuyukoData> {
        let file = File::open(&path).unwrap();
        let buf_read = BufReader::new(file);
        let mut character = serde_json::from_reader::<_, YuyukoData>(buf_read).unwrap();
        let name = path.file_stem().unwrap().to_str().unwrap().to_owned();
        path.pop();
        path.push(&name);

        path.push("sounds");
        for sound in SoundId::iter() {
            path.push(format!("{}.mp3", sound));
            use rodio::source::Source;
            let source =
                rodio::decoder::Decoder::new(std::io::BufReader::new(std::fs::File::open(&path)?))
                    .unwrap();
            let source = rodio::buffer::SamplesBuffer::new(
                source.channels(),
                source.sample_rate(),
                source.convert_samples().collect::<Vec<_>>(),
            )
            .buffered();

            character.sounds.data.insert(sound, source);
            path.pop();
        }

        path.pop();
        path.push("graphics");
        for (name, animation_group) in character.graphics.iter_mut() {
            path.push(name.file_name());
            AnimationGroup::load(ctx, assets, animation_group, path.clone())?;
            path.pop();
        }
        Ok(character)
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize, EnumIter, Display)]
pub enum YuyukoDataId {
    Butterfly,
}
impl Default for YuyukoDataId {
    fn default() -> Self {
        Self::Butterfly
    }
}

pub struct YuyukoPlayer {
    pub data: Rc<Yuyuko>,
    pub world: World,
    pub sound_renderer: SoundRenderer<SoundPath<SoundId>>,
    pub last_combo_state: Option<(ComboEffect, usize)>,
    pub state: PlayerState,
    pub combo_text: RefCell<Option<ggez::graphics::Text>>,
}

use std::cell::RefCell;

impl PlayerState {
    fn new(data: &Yuyuko) -> Self {
        Self {
            velocity: collision::Vec2::zeros(),
            position: collision::Vec2::zeros(),
            current_state: (0, MoveId::RoundStart),
            stun: None,
            air_actions: data.properties.max_air_actions,
            spirit_gauge: 0,
            spirit_delay: 0,
            hitstop: 0,
            facing: Facing::Right,
            health: data.properties.health,
            allowed_cancels: AllowedCancel::Always,
            rebeat_chain: HashSet::new(),
            should_pushback: true,
            sound_state: PlayerSoundState::new(),
            meter: 0,
            lockout: 0,
            dead: false,
            smp_list: Default::default(),
            most_recent_command: (CommandId::Stand, 0),
            first_command: None,
            last_hit_using: None,
            current_combo: None,
        }
    }
}

impl YuyukoPlayer {
    pub fn new(data: Rc<Yuyuko>) -> Self {
        Self {
            state: PlayerState::new(&data),
            data,
            last_combo_state: None,
            sound_renderer: SoundRenderer::new(),
            combo_text: RefCell::new(None),
            world: World::new(),
        }
    }

    fn in_corner(&self, play_area: &PlayArea) -> bool {
        let collision = self.collision();
        i32::abs(self.state.position.x) >= play_area.width / 2 - collision.half_size.x
    }

    fn handle_rebeat_data(&mut self) {
        if !matches!(
            self.data.get(self.state.current_state).state_type,
            StateType::Attack
        ) {
            self.state.rebeat_chain.clear();
        }
    }

    fn handle_smp(&mut self, opponent: &OpponentState) {
        if !opponent.in_hitstun {
            self.state.first_command = None;
            self.state.smp_list.clear();
        }
    }
    fn handle_expire(&mut self) {
        let (frame, move_id) = self.state.current_state;

        let state_data = self.data.get(self.state.current_state);

        self.state.current_state = if frame >= state_data.duration - 1 {
            self.state.allowed_cancels = AllowedCancel::Always;
            self.state.last_hit_using = None;
            self.state.rebeat_chain.clear();

            if move_id == MoveId::HitGround && self.state.dead {
                (0, MoveId::Dead)
            } else {
                (state_data.on_expire.frame, state_data.on_expire.state_id)
            }
        } else {
            (frame + 1, move_id)
        };
    }
    fn handle_hitstun(&mut self) {
        let state_data = self.data.get(self.state.current_state);

        if let Some(ref mut stun) = self.state.stun {
            assert!(matches!(
                state_data.state_type,
                StateType::Blockstun | StateType::Hitstun
            ));
            *stun -= 1;
            if *stun == 0 {
                self.state.stun = None;

                if !state_data.flags.airborne {
                    self.state.current_state = (
                        0,
                        if state_data.flags.crouching {
                            MoveId::Crouch
                        } else {
                            MoveId::Stand
                        },
                    );
                } else {
                    self.state.current_state =
                        if matches!(state_data.state_type, StateType::Blockstun) {
                            (0, MoveId::AirIdle)
                        } else {
                            (0, MoveId::Untech)
                        };
                }
            }
        }
    }
    fn handle_input(&mut self, input: &[InputState]) {
        let (frame, move_id) = self.state.current_state;
        let state_data = self.data.get(self.state.current_state);

        self.state.current_state = {
            let inputs = read_inputs(
                input.iter().rev(),
                self.state.facing,
                state_data.state_type.buffer_window(),
            );
            // TODO move to better handling for this
            // inputs should be able to detect releases
            if move_id == MoveId::Fly {
                if input.last().unwrap()[Button::E].is_pressed() {
                    (frame, move_id)
                } else {
                    (0, MoveId::FlyEnd)
                }
            } else {
                let state = &mut self.state;
                let data = self.data.as_ref();
                let possible_new_move = inputs
                    .iter()
                    .flat_map(|input| data.input_map.get(input))
                    .flat_map(|command_ids| command_ids.iter())
                    .map(|id| (*id, &data.command_map[id]))
                    .find(|(id, command)| {
                        command.reqs.iter().all(|requirement| match requirement {
                            Requirement::HasAirActions => state.air_actions > 0,
                            Requirement::InBlockstun => {
                                state_data.state_type == StateType::Blockstun
                            }
                            Requirement::NotLockedOut => state.lockout == 0,
                            Requirement::CanCancel(new_state_type) => {
                                let is_self = command.state_id == move_id;
                                let is_allowed_cancel =
                                    match state.allowed_cancels {
                                        AllowedCancel::Hit => {
                                            state_data.cancels.hit.contains(new_state_type)
                                        }
                                        AllowedCancel::Block => {
                                            state_data.cancels.block.contains(new_state_type)
                                        }
                                        AllowedCancel::Always => false,
                                    } || state_data.cancels.always.contains(new_state_type);

                                let can_rebeat = !state.rebeat_chain.contains(&id);

                                ((!is_self && can_rebeat)
                                    || (is_self && state_data.cancels.self_gatling))
                                    && is_allowed_cancel
                            }
                            Requirement::Meter(value) => state.meter >= *value,
                            Requirement::Spirit(value) => state.spirit_gauge >= *value,
                            Requirement::Airborne => state_data.flags.airborne,
                            Requirement::Grounded => !state_data.flags.airborne,
                            Requirement::CancelFrom(previous_state) => previous_state == &move_id,
                            Requirement::NoCancelFrom(previous_state) => previous_state != &move_id,
                        })
                    });

                let ret = possible_new_move
                    .map(|(_, command)| (command.frame, command.state_id))
                    .unwrap_or((frame, move_id));

                if let Some((command_id, command)) = possible_new_move {
                    for effect in command.effects.iter() {
                        match effect {
                            crate::character::command::Effect::UseAirAction => {
                                state.air_actions = state.air_actions.saturating_sub(1)
                            }
                            crate::character::command::Effect::UseMeter(meter) => {
                                state.meter -= meter
                            }
                            crate::character::command::Effect::RefillSpirit => {
                                state.spirit_gauge = self.data.properties.max_spirit_gauge
                            }
                            crate::character::command::Effect::FlipFacing => {
                                state.facing = state.facing.invert();
                            }
                        }
                    }
                    state.allowed_cancels = AllowedCancel::Always;
                    state.last_hit_using = None;
                    state.rebeat_chain.insert(command_id);
                    state.most_recent_command = (
                        command_id,
                        state.most_recent_command.1.checked_add(1).unwrap(),
                    );
                }

                ret
            }
        };
    }

    fn update_spirit(&mut self) {
        let state_data = self.data.get(self.state.current_state);
        let (ref mut frame, ref mut move_id) = &mut self.state.current_state;

        if *move_id == MoveId::Fly {
            self.state.spirit_gauge -= 5; // TODO, move this spirit cost to an editor value
            if self.state.spirit_gauge <= 0 {
                *move_id = MoveId::FlyEnd;
                *frame = 0;
            }
        } else {
            self.state.spirit_gauge -= state_data.flags.spirit_cost;

            if state_data.flags.reset_spirit_delay {
                self.state.spirit_delay = 0;
            }
            self.state.spirit_delay += state_data.flags.spirit_delay;
            self.state.spirit_delay -= 1;
            self.state.spirit_delay = std::cmp::max(self.state.spirit_delay, 0);

            if self.state.spirit_delay == 0 {
                self.state.spirit_gauge += 5; // TODO: move this spirit regen to an editor value
            }
        }

        self.clamp_spirit();
    }
    fn clamp_spirit(&mut self) {
        self.state.spirit_gauge = std::cmp::max(
            std::cmp::min(
                self.state.spirit_gauge,
                self.data.properties.max_spirit_gauge,
            ),
            0,
        );
    }
    fn update_velocity(&mut self, play_area: &PlayArea) {
        let state_data = self.data.get(self.state.current_state);

        let base_velocity = if state_data.flags.reset_velocity {
            collision::Vec2::zeros()
        } else {
            self.state.velocity
        };

        // we only run gravity if the move doesn't want to reset velocity, because that [resetting velocity] means the move has a trajectory in mind
        let gravity = if state_data.flags.gravity && state_data.flags.airborne {
            collision::Vec2::new(0_00, -0_20)
        } else {
            collision::Vec2::zeros()
        };
        let friction = if !state_data.flags.airborne || self.in_corner(play_area) {
            collision::Vec2::new(
                -i32::min(base_velocity.x.abs(), state_data.flags.friction)
                    * i32::signum(base_velocity.x),
                0_00,
            )
        } else {
            collision::Vec2::zeros()
        };

        let accel = self.state.facing.fix_collision(state_data.flags.accel);
        self.state.velocity = base_velocity + accel + friction + gravity;
        self.state.velocity.y = self.state.velocity.y.max(MAX_FALLING_VELOCITY);
    }
    fn update_position(&mut self, play_area: &PlayArea) {
        let state_data = self.data.get(self.state.current_state);

        self.state.position += self.state.velocity;

        // handle landing
        if state_data.flags.airborne
            && self.state.position.y - state_data.hitboxes.collision.half_size.y <= -4
        {
            let mut reset_hitstun = true;
            let mut reset_velocity = true;
            self.state.current_state = if state_data.state_type == StateType::Hitstun {
                let combo = self.state.current_combo.as_mut().unwrap();
                match combo.ground_action {
                    GroundAction::Knockdown => (0, MoveId::HitGround),
                    GroundAction::GroundSlam => {
                        self.state.velocity.y *= -90;
                        self.state.velocity.y /= 100;
                        combo.ground_action = GroundAction::Knockdown;
                        reset_hitstun = false;
                        reset_velocity = false;
                        (0, MoveId::HitstunAirStart)
                    }
                    GroundAction::OnTheGround => (0, MoveId::HitGround),
                }
            } else {
                (0, MoveId::Stand)
            };
            if reset_hitstun {
                self.state.stun = None;
            }
            if reset_velocity {
                self.state.velocity = collision::Vec2::zeros();
            }
            self.state.position.y = state_data.hitboxes.collision.half_size.y;
            self.state.air_actions = self.data.properties.max_air_actions;
        }

        self.validate_position(play_area);
    }

    fn spawn_objects(&mut self) {
        for spawner in self.data.get(self.state.current_state).current_spawns() {
            let mut builder = EntityBuilder::new();
            for constructor in spawner.data.iter() {
                let _ = match constructor {
                    Constructor::Contextless(c) => c.construct_on_to(&mut builder, ()),
                    Constructor::Position(c) => {
                        c.construct_on_to(&mut builder, self.state.position)
                    }
                }
                .unwrap();
            }
            self.world.spawn(builder.build());
        }
    }
    fn update_objects(&mut self, global_graphics: &HashMap<GlobalGraphic, AnimationGroup>) {
        for (_, Timer(timer)) in self.world.query::<&mut Timer>().iter() {
            *timer += 1;
        }
        let to_destroy: Vec<_> = self
            .world
            .query::<(&Timer, &YuyukoGraphic)>()
            .with::<ExpiresAfterAnimation>()
            .iter()
            .filter(|(_, (Timer(timer), graphic))| *timer >= self.data.graphics[graphic].duration())
            .map(|(entity, _)| entity)
            .chain(
                self.world
                    .query::<(&Timer, &GlobalGraphic)>()
                    .with::<ExpiresAfterAnimation>()
                    .iter()
                    .filter(|(_, (Timer(timer), graphic))| {
                        *timer >= global_graphics[graphic].duration()
                    })
                    .map(|(entity, _)| entity),
            )
            .collect();
        for entity in to_destroy {
            self.world.despawn(entity).unwrap();
        }
    }

    fn update_sound(&mut self) {
        for sound in self.data.get(self.state.current_state).current_sounds() {
            self.state.sound_state.play_sound(sound.channel, sound.name);
        }
    }

    fn handle_combo_state(&mut self) {
        let state_data = self.data.get(self.state.current_state);
        if !matches!(
            state_data.state_type,
            StateType::Hitstun | StateType::Blockstun
        ) {
            self.state.current_combo = None;
        }

        if self.state.current_combo.is_some() {
            self.last_combo_state = Some((self.state.current_combo.clone().unwrap(), 30));
        }
        if self.last_combo_state.is_some() && self.state.current_combo.is_none() {
            let (_, timer) = self.last_combo_state.as_mut().unwrap();
            *timer -= 1;
            if *timer == 0 {
                self.last_combo_state = None;
            }
        }
    }

    fn update_meter(&mut self, opponent_position: collision::Vec2) {
        let state_data = self.data.get(self.state.current_state);

        self.state.meter -= state_data.flags.meter_cost;

        if self.state.meter < 50_00 {
            self.state.meter += 5;
        } else if self.state.meter < 100_00 {
            self.state.meter += 2;
        } else if self.state.meter > 150_00 {
            self.state.meter -= 5;
        } else if self.state.meter > 100_00 {
            self.state.meter -= 2;
            // clamp to 100 to make sure we don't wobble around 100
            self.state.meter = self.state.meter.max(100_00);
        }

        let dir = (opponent_position - self.state.position).x.signum();
        let facing_opponent = dir == self.state.facing.collision_multiplier().x;
        if matches!(state_data.state_type, StateType::Movement) && facing_opponent {
            // only apply bonus/penalty if we're facing the opponent
            // fly is the exception to this because it reorients our facing direction
            // TODO stop having fly reorient facing direction

            let speed = self.state.velocity.x.abs();
            let bonus_meter = 50;
            // apply bonus/penalty based on speed
            if dir == self.state.velocity.x.signum() {
                self.state.meter += bonus_meter.min(bonus_meter * speed / 10_00);
            } else if -dir == self.state.velocity.x.signum() {
                self.state.meter -= bonus_meter.min(bonus_meter * speed / 10_00);
            }
        }

        self.state.meter = 0.max(200_00.min(self.state.meter))
    }

    fn update_lockout(&mut self) {
        self.state.lockout -= 1;
        self.state.lockout = 0.max(self.state.lockout);
    }
}

impl GenericCharacterBehaviour for YuyukoPlayer {
    fn apply_pushback(&mut self, force: collision::Int) {
        if !self.data.get(self.state.current_state).flags.airborne {
            self.state.position.x += force;
        }
    }
    fn validate_position(&mut self, play_area: &PlayArea) {
        let state_data = self.data.get(self.state.current_state);
        // handle stage sides
        if i32::abs(self.state.position.x)
            > play_area.width / 2 - state_data.hitboxes.collision.half_size.x
        {
            self.state.position.x = i32::signum(self.state.position.x)
                * (play_area.width / 2 - state_data.hitboxes.collision.half_size.x);
        }

        // if not airborne, make sure the character is locked to the ground properly
        if !state_data.flags.airborne {
            self.state.position.y = state_data.hitboxes.collision.half_size.y;
        }
    }
    fn prune_bullets(&mut self, _play_area: &PlayArea) {}

    fn would_be_hit(
        &self,
        input: &[InputState],
        attack_info: &AttackInfo,
        source: &Source,
        combo_effect: Option<&ComboEffect>,
        old_effect: Option<HitEffect>,
    ) -> HitResult {
        let state_data = self.data.get(self.state.current_state);
        let axis = DirectedAxis::from_facing(input.last().unwrap().axis, self.state.facing);
        match old_effect {
            Some(effect) => match effect {
                HitEffect::Hit(effect) => {
                    if effect.combo.available_limit > 0 {
                        effect.append_hit(attack_info, source).into()
                    } else {
                        effect.into()
                    }
                }
                HitEffect::GuardCrush(effect) => {
                    if effect.combo.available_limit > 0 {
                        effect.append_hit(attack_info).into()
                    } else {
                        effect.into()
                    }
                }
                HitEffect::CounterHit(effect) => {
                    if effect.combo.available_limit > 0 {
                        effect.append_hit(attack_info).into()
                    } else {
                        effect.into()
                    }
                }
                HitEffect::Graze(effect) => {
                    if attack_info.magic && state_data.flags.bullet.is_invuln()
                        || attack_info.melee && state_data.flags.melee.is_invuln()
                        || attack_info.air && state_data.flags.air.is_invuln()
                        || attack_info.foot && state_data.flags.foot.is_invuln()
                    {
                        effect.into()
                    } else if attack_info.grazeable {
                        effect.append_graze(attack_info).into()
                    } else if (state_data.flags.can_block
                        && (axis.is_blocking(false)
                            || axis.is_blocking(self.state.facing == source.facing)))
                        && !(attack_info.air_unblockable && state_data.flags.airborne)
                    {
                        if state_data.flags.airborne || axis.is_guarding(attack_info.guard) {
                            if block::Effect::would_crush(
                                Some(effect.defender.take_spirit_gauge),
                                attack_info,
                                self.state.spirit_gauge,
                            ) {
                                effect
                                    .append_guard_crush(
                                        attack_info,
                                        source,
                                        state_data.flags.airborne,
                                    )
                                    .into()
                            } else {
                                effect
                                    .append_block(attack_info, source, state_data.flags.airborne)
                                    .into()
                            }
                        } else if wrong_block::Effect::would_crush(
                            Some(effect.defender.take_spirit_gauge),
                            attack_info,
                            self.state.spirit_gauge,
                        ) {
                            effect
                                .append_guard_crush(attack_info, source, state_data.flags.airborne)
                                .into()
                        } else {
                            effect.append_wrongblock(attack_info, source).into()
                        }
                    } else if state_data.flags.can_be_counter_hit && attack_info.can_counter_hit {
                        effect
                            .append_counterhit(attack_info, source, state_data.flags.airborne)
                            .into()
                    } else {
                        effect
                            .append_hit(attack_info, source, state_data.flags.airborne)
                            .into()
                    }
                }
                HitEffect::Block(effect) => {
                    if !(attack_info.air_unblockable && state_data.flags.airborne) {
                        if state_data.flags.airborne || axis.is_guarding(attack_info.guard) {
                            if block::Effect::would_crush(
                                Some(effect.defender.take_spirit_gauge),
                                attack_info,
                                self.state.spirit_gauge,
                            ) {
                                effect
                                    .append_guard_crush(
                                        attack_info,
                                        source,
                                        state_data.flags.airborne,
                                    )
                                    .into()
                            } else {
                                effect
                                    .append_block(attack_info, source, state_data.flags.airborne)
                                    .into()
                            }
                        } else if wrong_block::Effect::would_crush(
                            Some(effect.defender.take_spirit_gauge),
                            attack_info,
                            self.state.spirit_gauge,
                        ) {
                            effect
                                .append_guard_crush(attack_info, source, state_data.flags.airborne)
                                .into()
                        } else {
                            effect.append_wrongblock(attack_info, source).into()
                        }
                    } else {
                        effect
                            .append_hit(attack_info, source, state_data.flags.airborne)
                            .into()
                    }
                }
                HitEffect::WrongBlock(effect) => {
                    if axis.is_guarding(attack_info.guard) {
                        if block::Effect::would_crush(
                            Some(effect.defender.take_spirit_gauge),
                            attack_info,
                            self.state.spirit_gauge,
                        ) {
                            effect
                                .append_guard_crush(attack_info, source, state_data.flags.airborne)
                                .into()
                        } else {
                            effect.append_block(attack_info).into()
                        }
                    } else if wrong_block::Effect::would_crush(
                        Some(effect.defender.take_spirit_gauge),
                        attack_info,
                        self.state.spirit_gauge,
                    ) {
                        effect
                            .append_guard_crush(attack_info, source, state_data.flags.airborne)
                            .into()
                    } else {
                        effect.append_wrongblock(attack_info, source).into()
                    }
                }
            },
            None => {
                if attack_info.magic && state_data.flags.bullet.is_invuln()
                    || attack_info.melee && state_data.flags.melee.is_invuln()
                    || attack_info.air && state_data.flags.air.is_invuln()
                    || attack_info.foot && state_data.flags.foot.is_invuln()
                    || self
                        .state
                        .current_combo
                        .as_ref()
                        .map(|item| item.available_limit <= 0)
                        .unwrap_or(false)
                {
                    HitResult::None
                } else if attack_info.grazeable && state_data.flags.grazing {
                    graze::Effect::build(attack_info).into()
                } else if (matches!(state_data.state_type, StateType::Blockstun)
                    || (state_data.flags.can_block
                    // this is crossup protection
                    // if the attack is facing the same direction you're facing
                    // then the attack should be able to be blocked by holding both back
                    // and forward.
                    && (axis.is_blocking(false) || axis.is_blocking(self.state.facing == source.facing))))
                    && !(attack_info.air_unblockable && state_data.flags.airborne)
                {
                    if state_data.flags.airborne || axis.is_guarding(attack_info.guard) {
                        if block::Effect::would_crush(None, attack_info, self.state.spirit_gauge) {
                            guard_crush::Effect::build(
                                attack_info,
                                source,
                                state_data.flags.airborne,
                            )
                            .into()
                        } else {
                            block::Effect::build(attack_info, source, state_data.flags.airborne)
                                .into()
                        }
                    } else if wrong_block::Effect::would_crush(
                        None,
                        attack_info,
                        self.state.spirit_gauge,
                    ) {
                        guard_crush::Effect::build(attack_info, source, state_data.flags.airborne)
                            .into()
                    } else {
                        wrong_block::Effect::build(attack_info, source).into()
                    }
                } else if state_data.flags.can_be_counter_hit && attack_info.can_counter_hit {
                    counter_hit::Effect::build(attack_info, source, state_data.flags.airborne)
                        .into()
                } else {
                    combo_effect
                        .map(|combo| {
                            if combo.available_limit > 0 {
                                hit::Effect::build(
                                    attack_info,
                                    source,
                                    state_data.flags.airborne,
                                    combo.clone(),
                                )
                                .into()
                            } else {
                                HitResult::None
                            }
                        })
                        .unwrap_or_else(|| {
                            hit::Effect::build_starter(
                                attack_info,
                                source,
                                state_data.flags.airborne,
                            )
                            .into()
                        })
                }
            }
        }
    }

    fn in_hitstun(&self) -> bool {
        self.data.get(self.state.current_state).state_type == StateType::Hitstun
    }

    fn take_hit(&mut self, info: &HitEffect, play_area: &PlayArea) {
        let airborne = match info {
            HitEffect::Hit(hit::Effect {
                defender:
                    hit::DefenderEffect {
                        take_damage,
                        is_lethal,
                        set_force,
                        modify_meter,
                        set_stun,
                        set_stop,
                        set_should_pushback,
                        ..
                    },
                ..
            })
            | HitEffect::CounterHit(counter_hit::Effect {
                defender:
                    counter_hit::DefenderEffect {
                        take_damage,
                        is_lethal,
                        set_force,
                        modify_meter,
                        set_stun,
                        set_stop,
                        set_should_pushback,
                        ..
                    },
                ..
            })
            | HitEffect::GuardCrush(guard_crush::Effect {
                defender:
                    guard_crush::DefenderEffect {
                        take_damage,
                        is_lethal,
                        set_force,
                        modify_meter,
                        set_stun,
                        set_stop,
                        set_should_pushback,
                        ..
                    },
                ..
            })
            | HitEffect::Block(block::Effect {
                defender:
                    block::DefenderEffect {
                        take_damage,
                        is_lethal,
                        set_force,
                        modify_meter,
                        set_stun,
                        set_stop,
                        set_should_pushback,
                        ..
                    },
                ..
            })
            | HitEffect::WrongBlock(wrong_block::Effect {
                defender:
                    wrong_block::DefenderEffect {
                        take_damage,
                        is_lethal,
                        set_force,
                        modify_meter,
                        set_stun,
                        set_stop,
                        set_should_pushback,
                        ..
                    },
                ..
            }) => {
                self.state.health -= take_damage;

                if self.state.health <= 0 && *is_lethal {
                    self.state.dead = true;
                }

                let airborne = match set_force {
                    Force::Airborne(_) => true,
                    Force::Grounded(_) => false,
                } || self.state.dead;

                self.state.should_pushback = *set_should_pushback;
                self.state.meter += modify_meter;

                self.state.hitstop = *set_stop;

                self.state.stun = Some(*set_stun);
                self.state.velocity = match set_force {
                    Force::Airborne(value) | Force::Grounded(value) => *value,
                };

                airborne
            }

            HitEffect::Graze(effect) => {
                let effect = &effect.defender;

                self.state.health -= effect.take_damage;

                self.state.hitstop = effect.set_stop;
                self.data.get(self.state.current_state).flags.airborne
            }
        };

        match info {
            HitEffect::Hit(hit::Effect { combo, .. })
            | HitEffect::GuardCrush(guard_crush::Effect { combo, .. })
            | HitEffect::CounterHit(counter_hit::Effect { combo, .. }) => {
                self.state.current_combo = Some(combo.clone());
            }
            _ => {}
        }

        match info {
            HitEffect::Hit(hit::Effect {
                defender:
                    hit::DefenderEffect {
                        take_spirit_gauge,
                        reset_spirit_delay,
                        add_spirit_delay,
                        ..
                    },
                ..
            })
            | HitEffect::CounterHit(counter_hit::Effect {
                defender:
                    counter_hit::DefenderEffect {
                        take_spirit_gauge,
                        reset_spirit_delay,
                        add_spirit_delay,
                        ..
                    },
                ..
            })
            | HitEffect::Graze(graze::Effect {
                defender:
                    graze::DefenderEffect {
                        take_spirit_gauge,
                        reset_spirit_delay,
                        add_spirit_delay,
                        ..
                    },
                ..
            })
            | HitEffect::Block(block::Effect {
                defender:
                    block::DefenderEffect {
                        take_spirit_gauge,
                        reset_spirit_delay,
                        add_spirit_delay,
                        ..
                    },
                ..
            })
            | HitEffect::WrongBlock(wrong_block::Effect {
                defender:
                    wrong_block::DefenderEffect {
                        take_spirit_gauge,
                        reset_spirit_delay,
                        add_spirit_delay,
                        ..
                    },
                ..
            }) => {
                self.state.spirit_gauge -= *take_spirit_gauge;
                self.state.spirit_delay = if *reset_spirit_delay {
                    0
                } else {
                    self.state.spirit_delay
                } + *add_spirit_delay;
            }
            HitEffect::GuardCrush(_) => {
                self.state.spirit_gauge = self.data.properties.max_spirit_gauge;
            }
        }

        match info {
            HitEffect::GuardCrush(_) => {
                if airborne {
                    self.state.current_state = (0, MoveId::HitstunAirStart);
                } else {
                    self.state.current_state = (0, MoveId::GuardCrush);
                }
            }
            HitEffect::Hit(_) | HitEffect::CounterHit(_) => {
                if airborne {
                    self.state.current_state = (0, MoveId::HitstunAirStart);
                } else {
                    self.state.current_state = (0, MoveId::HitstunStandStart);
                }
            }
            HitEffect::Block(_) => {
                if airborne {
                    self.state.current_state = (0, MoveId::BlockstunAirStart);
                } else if self.data.get(self.state.current_state).flags.crouching {
                    self.state.current_state = (0, MoveId::BlockstunCrouchStart);
                } else {
                    self.state.current_state = (0, MoveId::BlockstunStandStart);
                }
            }
            HitEffect::WrongBlock(_) => {
                if self.data.get(self.state.current_state).flags.crouching {
                    self.state.current_state = (0, MoveId::WrongblockCrouchStart);
                } else {
                    self.state.current_state = (0, MoveId::WrongblockStandStart);
                }
            }
            HitEffect::Graze(_) => {}
        }

        self.validate_position(play_area);
    }

    fn deal_hit(&mut self, info: &HitType) {
        match info {
            HitType::Hit => {
                self.state
                    .sound_state
                    .play_sound(ChannelName::Hit, GlobalSound::Hit.into());
            }
            HitType::GuardCrush => {
                self.state
                    .sound_state
                    .play_sound(ChannelName::Hit, GlobalSound::GuardCrush.into());
            }
            HitType::CounterHit => {
                self.state
                    .sound_state
                    .play_sound(ChannelName::Hit, GlobalSound::CounterHit.into());
            }
            HitType::Block => {
                self.state
                    .sound_state
                    .play_sound(ChannelName::Hit, GlobalSound::Block.into());
            }
            HitType::WrongBlock => {
                self.state
                    .sound_state
                    .play_sound(ChannelName::Hit, GlobalSound::WrongBlock.into());
            }
            HitType::Graze => {}
        }

        let hitbox = self
            .data
            .get(self.state.current_state)
            .hitboxes
            .hitbox
            .as_ref()
            .unwrap();
        let attack_info = &self.data.attacks[&hitbox.data_id];

        self.state.meter += match info {
            HitType::Hit => attack_info.on_hit.attacker_meter,
            HitType::GuardCrush => attack_info.on_guard_crush.attacker_meter,
            HitType::CounterHit => attack_info.on_counter_hit.attacker_meter,
            HitType::Graze => attack_info.on_graze.attacker_meter,
            HitType::Block => attack_info.on_block.attacker_meter,
            HitType::WrongBlock => attack_info.on_wrongblock.attacker_meter,
        };
        self.state.hitstop = match info {
            HitType::Hit => attack_info.on_hit.attacker_stop,
            HitType::GuardCrush => attack_info.on_guard_crush.attacker_stop,
            HitType::CounterHit => attack_info.on_counter_hit.attacker_stop,
            HitType::Graze => 0,
            HitType::Block => attack_info.on_block.attacker_stop,
            HitType::WrongBlock => attack_info.on_wrongblock.attacker_stop,
        };

        self.state.last_hit_using = Some((hitbox.data_id, hitbox.id));

        let associated_command = self.state.most_recent_command;

        if let Some(first_command) = self.state.first_command {
            if associated_command != first_command
                && associated_command.1 > first_command.1
                && !self.state.smp_list.contains_key(&associated_command.0)
            {
                self.state
                    .smp_list
                    .insert(associated_command.0, associated_command.1);
            }
        } else {
            self.state.first_command = Some(associated_command);
        }

        match info {
            HitType::Hit | HitType::GuardCrush | HitType::CounterHit => {
                self.state.allowed_cancels = AllowedCancel::Hit;
            }
            HitType::Graze => {}
            HitType::Block | HitType::WrongBlock => {
                self.state.allowed_cancels = AllowedCancel::Block;
            }
        }
    }

    fn get_current_combo(&self) -> Option<&ComboEffect> {
        self.state.current_combo.as_ref()
    }

    fn get_attack_data(&self) -> Option<Cow<'_, AttackInfo>> {
        let smp = self
            .state
            .smp_list
            .get(&self.state.most_recent_command.0)
            .map(|old_time| self.state.most_recent_command.1 > *old_time)
            .unwrap_or(false);

        self.data
            .get(self.state.current_state)
            .hitboxes
            .hitbox
            .as_ref()
            .and_then(|hitbox| {
                if Some((hitbox.data_id, hitbox.id)) != self.state.last_hit_using {
                    if smp {
                        let mut owned = self.data.attacks[&hitbox.data_id].clone();

                        owned.on_hit.limit_cost *= 2;

                        Some(Cow::Owned(owned))
                    } else {
                        Some(Cow::Borrowed(&self.data.attacks[&hitbox.data_id]))
                    }
                } else {
                    None
                }
            })
    }

    fn handle_refacing(&mut self, other_player: collision::Int) {
        if self.data.get(self.state.current_state).flags.allow_reface {
            self.state.facing = if self.state.position.x > other_player
                && self.state.facing == Facing::Right
            {
                Facing::Left
            } else if self.state.position.x < other_player && self.state.facing == Facing::Left {
                Facing::Right
            } else {
                self.state.facing
            }
        }
    }
    fn update_frame_mut(
        &mut self,
        input: &[InputState],
        opponent: OpponentState,
        play_area: &PlayArea,
        global_graphics: &HashMap<GlobalGraphic, AnimationGroup>,
    ) {
        if self.state.hitstop > 0 {
            self.state.hitstop -= 1;
        } else {
            self.handle_expire();
            self.handle_rebeat_data();
            self.handle_hitstun();
            self.handle_input(input);
            self.update_velocity(play_area);
            self.update_position(play_area);
            self.update_sound();
        }
        self.handle_combo_state();
        self.update_spirit();
        self.handle_smp(&opponent);
        self.update_lockout();
        self.update_meter(opponent.position);
        self.update_objects(global_graphics);
        self.spawn_objects();
        self.state.sound_state.update();
        self.state.hitstop = i32::max(0, self.state.hitstop);
    }

    fn draw_ui(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        ui: &UiElements,
        bottom_line: graphics::Matrix4,
        flipped: bool,
        wins: usize,
        first_to: usize,
        last_combo_state: &Option<(ComboEffect, usize)>,
    ) -> GameResult<()> {
        use std::ops::DerefMut;
        crate::roster::generic_character::impls::draw_ui(
            ctx,
            assets,
            ui,
            bottom_line,
            flipped,
            wins,
            first_to,
            last_combo_state,
            self.combo_text.borrow_mut().deref_mut(),
            self.state.health,
            self.state.spirit_gauge,
            self.state.meter,
            self.state.lockout,
            &self.data.properties,
        )
    }

    fn get_last_combo_state(&self) -> &Option<(ComboEffect, usize)> {
        &self.last_combo_state
    }

    fn draw(&self, ctx: &mut Context, assets: &Assets, world: graphics::Matrix4) -> GameResult<()> {
        let (frame, move_id) = self.state.current_state;

        let collision = &self.data.states[&move_id].hitboxes[frame].collision;

        let graphic = self.data.state_graphics_map[&move_id];

        self.data.graphics[&graphic].draw_at_time(
            ctx,
            assets,
            frame,
            crate::roster::impls::get_transform(
                world,
                collision.center,
                self.state.position,
                self.state.facing,
            ),
        )
    }

    fn draw_objects(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        world: graphics::Matrix4,
        global_graphics: &HashMap<GlobalGraphic, AnimationGroup>,
    ) -> GameResult<()> {
        for (_, (position, graphic, Timer(frame))) in self
            .world
            .query::<(&Position, &YuyukoGraphic, &Timer)>()
            .iter()
        {
            self.data.graphics[graphic].draw_at_time(
                ctx,
                assets,
                *frame % self.data.graphics[graphic].duration(),
                world
                    * graphics::Matrix4::new_translation(&graphics::up_dimension(
                        position.value.into_graphical(),
                    )),
            )?;
        }
        for (_, (position, graphic, Timer(frame))) in self
            .world
            .query::<(&Position, &GlobalGraphic, &Timer)>()
            .iter()
        {
            global_graphics[graphic].draw_at_time(
                ctx,
                assets,
                *frame % global_graphics[graphic].duration(),
                world
                    * graphics::Matrix4::new_translation(&graphics::up_dimension(
                        position.value.into_graphical(),
                    )),
            )?;
        }

        Ok(())
    }

    fn draw_shadow(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        world: graphics::Matrix4,
    ) -> GameResult<()> {
        let (frame, move_id) = self.state.current_state;

        let collision = &self.data.states[&move_id].hitboxes[frame].collision;

        let graphic = self.data.state_graphics_map[&move_id];

        self.data.graphics[&graphic].draw_shadow_at_time(
            ctx,
            assets,
            frame,
            crate::roster::impls::get_transform(
                world,
                collision.center,
                self.state.position,
                self.state.facing,
            ),
        )
    }
    fn get_pushback(&self, play_area: &PlayArea) -> collision::Int {
        let (_, move_id) = &self.state.current_state;
        let state = &self.data.states[&move_id];

        if matches!(state.state_type, StateType::Hitstun | StateType::Blockstun)
            && self.in_corner(play_area)
            && self.state.hitstop == 0
            && self.state.should_pushback
        {
            -self.state.velocity.x
        } else {
            0
        }
    }
    fn collision(&self) -> PositionedHitbox {
        let (frame, move_id) = &self.state.current_state;
        self.data.states[move_id].hitboxes[*frame]
            .collision
            .with_collision_position(self.state.position)
    }
    fn hitboxes(&self) -> Vec<PositionedHitbox> {
        let (frame, move_id) = &self.state.current_state;
        self.data.states[move_id].hitboxes[*frame]
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
    fn hurtboxes(&self) -> Vec<PositionedHitbox> {
        let (frame, move_id) = &self.state.current_state;
        self.data.states[move_id].hitboxes[*frame]
            .hurtbox
            .iter()
            .map(|item| item.with_position_and_facing(self.state.position, self.state.facing))
            .collect()
    }

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
    fn render_sound(&mut self, audio_device: &Device, sound_list: &GlobalSoundList, fps: u32) {
        self.sound_renderer.render_frame(
            &audio_device,
            &self.data.sounds.data,
            &sound_list.data,
            &self.state.sound_state,
            fps,
        );
    }

    fn save(&self) -> GameResult<OpaqueStateData> {
        Ok(OpaqueStateData::Yuyuko(self.state.clone()))
    }
    fn load(&mut self, value: OpaqueStateData) -> GameResult<()> {
        match value {
            OpaqueStateData::Yuyuko(state) => self.state = state,
            _ => panic!("tried to load a different characters data into my own data"),
        }

        Ok(())
    }

    fn update_cutscene(&mut self, play_area: &PlayArea) {
        if self.in_cutscene() {
            self.handle_expire();
        }
        self.validate_position(play_area);
        self.state.sound_state.update();
    }

    fn in_cutscene(&self) -> bool {
        let (current_frame, move_id) = self.state.current_state;
        self.data.states[&move_id].flags[current_frame + 1].cutscene
    }

    fn get_flash(&self) -> Option<FlashType> {
        self.data.get(self.state.current_state).flags.flash
    }

    fn get_lockout(&self) -> (i32, bool) {
        let flags = self.data.get(self.state.current_state).flags;
        (flags.lockout_timer, flags.reset_lockout_timer)
    }

    fn modify_lockout(&mut self, timer: i32, reset: bool) {
        self.state.lockout = timer + if reset { 0 } else { self.state.lockout };
    }

    fn is_locked_out(&self) -> bool {
        self.state.lockout > 0
    }

    fn update_no_input(
        &mut self,
        play_area: &PlayArea,
        global_graphics: &HashMap<GlobalGraphic, AnimationGroup>,
    ) {
        if self.state.hitstop > 0 {
            self.state.hitstop -= 1;
        } else {
            self.handle_expire();
            self.handle_hitstun();
            self.update_velocity(play_area);
            self.update_position(play_area);
            self.update_sound();
        }
        self.handle_combo_state();
        self.update_spirit();
        self.update_lockout();
        self.update_objects(global_graphics);
        self.spawn_objects();
        self.state.sound_state.update();
        self.state.hitstop = i32::max(0, self.state.hitstop);
    }

    fn is_dead(&self) -> bool {
        self.state.dead
    }

    fn draw_order_priority(&self) -> i32 {
        if matches!(
            self.data.states[&self.state.current_state.1].state_type,
            StateType::Hitstun | StateType::Blockstun
        ) {
            -1
        } else {
            0
        }
    }

    fn reset_to_position_roundstart(
        &mut self,
        play_area: &PlayArea,
        position: collision::Int,
        facing: Facing,
    ) {
        self.state = PlayerState {
            position: collision::Vec2::new(position, 0),
            velocity: collision::Vec2::zeros(),
            current_state: (0, MoveId::Stand),
            stun: None,
            air_actions: self.data.properties.max_air_actions,
            spirit_gauge: 0,
            spirit_delay: 0,
            hitstop: 0,
            facing,
            health: self.data.properties.health,
            allowed_cancels: AllowedCancel::Always,
            rebeat_chain: HashSet::new(),
            should_pushback: true,
            sound_state: PlayerSoundState::new(),
            meter: 0,
            lockout: 0,
            dead: false,
            smp_list: Default::default(),
            most_recent_command: (CommandId::Stand, 0),
            first_command: None,
            last_hit_using: None,
            current_combo: None,
        };
        self.validate_position(play_area);
    }
    fn reset_to_position_gamestart(
        &mut self,
        play_area: &PlayArea,
        position: collision::Int,
        facing: Facing,
    ) {
        self.state = PlayerState {
            position: collision::Vec2::new(position, 0),
            velocity: collision::Vec2::zeros(),
            current_state: (0, MoveId::RoundStart),
            stun: None,
            air_actions: self.data.properties.max_air_actions,
            spirit_gauge: 0,
            spirit_delay: 0,
            hitstop: 0,
            facing,
            health: self.data.properties.health,
            allowed_cancels: AllowedCancel::Always,
            rebeat_chain: HashSet::new(),
            should_pushback: true,
            sound_state: PlayerSoundState::new(),
            meter: 0,
            lockout: 0,
            dead: false,
            smp_list: Default::default(),
            most_recent_command: (CommandId::Stand, 0),
            first_command: None,
            last_hit_using: None,
            current_combo: None,
        };

        self.validate_position(play_area);
    }

    fn health(&self) -> i32 {
        self.state.health
    }
}
