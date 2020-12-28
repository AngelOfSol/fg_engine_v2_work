pub mod attacks;
pub mod moves;

use crate::game_object::state::Timer;
use crate::graphics::animation_group::AnimationGroup;
use crate::hitbox::PositionedHitbox;
use crate::input::button::Button;
use crate::input::{read_inputs, DirectedAxis, Facing, InputState};
use crate::roster::generic_character::combo_state::{AllowedCancel, ComboState};
use crate::roster::generic_character::extra_data::ExtraData;
use crate::roster::generic_character::hit_info::{
    EffectData, Force, HitAction, HitEffect, HitEffectType, HitResult, HitSource,
};
use crate::roster::generic_character::GenericCharacterBehaviour;
use crate::roster::generic_character::OpaqueStateData;
use crate::typedefs::collision;
use crate::typedefs::graphics;
use crate::{assets::Assets, game_object::constructors::Construct};
use crate::{
    character::command::Requirement,
    game_match::sounds::{ChannelName, GlobalSound, SoundList, SoundRenderer},
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
use crate::{character::state::State, typedefs::collision::IntoGraphical};
use crate::{
    character::{
        command::Command,
        state::components::{Flags, GlobalGraphic},
    },
    input::Input,
};
use crate::{game_match::sounds::SoundPath, game_object::state::ExpiresAfterAnimation};
use attacks::AttackId;
use ggez::{Context, GameResult};
use hecs::{EntityBuilder, World};
use inspect_design::Inspect;
use moves::{CommandId, MoveId};
use rodio::Device;
use serde::Deserialize;
use serde::Serialize;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::BufReader;
use std::path::PathBuf;
use std::rc::Rc;
use strum::IntoEnumIterator;
use strum::{Display, EnumIter};

#[derive(Clone, Inspect)]
pub struct Yuyuko {
    #[tab = "States"]
    pub states: StateList,
    #[tab = "Attacks"]
    pub attacks: AttackList,
    #[tab = "Properties"]
    pub properties: Properties,
    #[skip]
    pub sounds: SoundList<YuyukoSound>,
    #[tab = "Graphics"]
    pub graphics: HashMap<YuyukoGraphic, AnimationGroup>,
    #[tab = "Inputs"]
    pub input_map: HashMap<Input, Vec<CommandId>>,
    #[tab = "Commands"]
    pub command_map: HashMap<CommandId, Command<MoveId>>,
}

impl std::fmt::Debug for Yuyuko {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.states)
    }
}

type StateList = HashMap<MoveId, State<MoveId, AttackId, YuyukoSound>>;
pub type AttackList = HashMap<AttackId, AttackInfo>;

impl Yuyuko {
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
        })
    }
}

#[derive(Deserialize)]
pub struct YuyukoData {
    states: StateList,
    properties: Properties,
    attacks: AttackList,
    #[serde(skip)]
    #[serde(default = "SoundList::new")]
    sounds: SoundList<YuyukoSound>,
    graphics: HashMap<YuyukoGraphic, AnimationGroup>,
    input_map: HashMap<Input, Vec<CommandId>>,
    command_map: HashMap<CommandId, Command<MoveId>>,
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
        path.push("states");
        for (name, state) in character.states.iter_mut() {
            State::load(ctx, assets, state, &name.file_name(), path.clone())?;
        }
        path.pop();

        path.push("sounds");
        for sound in YuyukoSound::iter() {
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
#[derive(
    Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize, EnumIter, Display, Inspect,
)]
pub enum YuyukoSound {
    Grunt,
}

impl Into<SoundPath<YuyukoSound>> for YuyukoSound {
    fn into(self) -> SoundPath<YuyukoSound> {
        SoundPath::Local(self)
    }
}

impl Default for YuyukoSound {
    fn default() -> Self {
        Self::Grunt
    }
}

#[derive(
    Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize, EnumIter, Display, Inspect,
)]
#[serde(rename_all = "snake_case")]
pub enum YuyukoGraphic {
    SuperJumpParticle,
    HitEffect,
    Butterfly1,
    Butterfly2,
    Butterfly3,
    Butterfly4,
}

impl Default for YuyukoGraphic {
    fn default() -> Self {
        Self::Butterfly1
    }
}

impl YuyukoGraphic {
    pub fn file_name(self) -> String {
        serde_json::to_string(&self)
            .unwrap()
            .trim_matches('\"')
            .to_owned()
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
    pub sound_renderer: SoundRenderer<SoundPath<YuyukoSound>>,
    pub last_combo_state: Option<(ComboState, usize)>,
    pub state: YuyukoState,
    pub combo_text: RefCell<Option<ggez::graphics::Text>>,
}

use std::cell::RefCell;

pub type YuyukoState = PlayerState<MoveId, YuyukoSound, CommandId>;

impl YuyukoState {
    fn new(data: &Yuyuko) -> Self {
        Self {
            velocity: collision::Vec2::zeros(),
            position: collision::Vec2::zeros(),
            current_state: (0, MoveId::RoundStart),
            extra_data: ExtraData::None,
            air_actions: data.properties.max_air_actions,
            spirit_gauge: 0,
            spirit_delay: 0,
            hitstop: 0,
            facing: Facing::Right,
            last_hit_using: None,
            health: data.properties.health,
            current_combo: None,
            allowed_cancels: AllowedCancel::Always,
            rebeat_chain: HashSet::new(),
            should_pushback: true,
            sound_state: PlayerSoundState::new(),
            meter: 0,
            lockout: 0,
            dead: false,
        }
    }
}

use crate::game_match::sounds::PlayerSoundState;

use super::PlayerState;

impl YuyukoPlayer {
    pub fn new(data: Rc<Yuyuko>) -> Self {
        Self {
            state: YuyukoState::new(&data),
            data,
            last_combo_state: None,
            sound_renderer: SoundRenderer::new(),
            combo_text: RefCell::new(None),
            world: World::new(),
        }
    }
    fn handle_fly(move_id: MoveId, extra_data: &mut ExtraData) -> collision::Vec2 {
        crate::roster::impls::handle_fly(move_id, MoveId::FlyStart, extra_data)
    }
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
                MoveId::Jump => {
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
                MoveId::SuperJump | MoveId::BorderEscapeJump => {
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
    fn in_corner(&self, play_area: &PlayArea) -> bool {
        let collision = self.collision();
        i32::abs(self.state.position.x) >= play_area.width / 2 - collision.half_size.x
    }
    fn current_flags(&self) -> &Flags {
        let (frame, move_id) = self.state.current_state;
        &self.data.states[&move_id].flags[frame]
    }
    fn handle_rebeat_data(&mut self) {
        let (_, move_id) = self.state.current_state;

        if !matches!(self.data.states[&move_id].state_type, StateType::Attack) {
            self.state.rebeat_chain.clear();
        }
    }
    fn handle_expire(&mut self) {
        let (frame, move_id) = self.state.current_state;

        // if the next frame would be out of bounds
        self.state.current_state = if frame >= self.data.states[&move_id].duration() - 1 {
            self.state.allowed_cancels = AllowedCancel::Always;
            self.state.last_hit_using = None;
            self.state.rebeat_chain.clear();

            if move_id == MoveId::HitGround && self.state.dead {
                (0, MoveId::Dead)
            } else {
                let on_expire = &self.data.states[&move_id].on_expire;
                (on_expire.frame, on_expire.state_id)
            }
        } else {
            (frame + 1, move_id)
        };
    }
    fn handle_hitstun(&mut self) {
        let (frame, move_id) = self.state.current_state;
        let flags = &self.data.states[&move_id].flags[frame];
        let state_type = self.data.states[&move_id].state_type;

        if matches!(state_type, StateType::Blockstun | StateType::Hitstun) {
            let hitstun = self.state.extra_data.unwrap_stun_mut();
            *hitstun -= 1;
            if *hitstun == 0 {
                if !flags.airborne {
                    self.state.current_state = (
                        0,
                        if flags.crouching {
                            MoveId::Crouch
                        } else {
                            MoveId::Stand
                        },
                    );
                } else {
                    self.state.current_state = if matches!(state_type, StateType::Blockstun) {
                        (0, MoveId::AirIdle)
                    } else {
                        (frame, move_id)
                    };
                }
            }
        }
    }
    fn handle_input(&mut self, input: &[InputState]) {
        let (frame, move_id) = self.state.current_state;
        let cancels = &self.data.states[&move_id].cancels[frame];
        let flags = &self.data.states[&move_id].flags[frame];
        let state_type = self.data.states[&move_id].state_type;

        self.state.current_state = {
            let inputs = read_inputs(
                input.iter().rev(),
                self.state.facing,
                state_type.buffer_window(),
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
                    .flat_map(|item| data.input_map.get(item))
                    .flat_map(|item| item.iter())
                    .map(|item| (*item, &data.command_map[item]))
                    .find(|(command_id, item)| {
                        item.reqs.iter().all(|requirement| match requirement {
                            Requirement::HasAirActions => state.air_actions > 0,
                            Requirement::InBlockstun => state_type == StateType::Blockstun,
                            Requirement::NotLockedOut => state.lockout == 0,
                            Requirement::CanCancel(new_state_type) => {
                                let is_self = item.state_id == move_id;
                                let is_allowed_cancel = match state.allowed_cancels {
                                    AllowedCancel::Hit => cancels.hit.contains(new_state_type),
                                    AllowedCancel::Block => cancels.block.contains(new_state_type),
                                    AllowedCancel::Always => false,
                                } || cancels
                                    .always
                                    .contains(new_state_type);

                                let can_rebeat = !state.rebeat_chain.contains(&command_id);

                                ((!is_self && can_rebeat) || (is_self && cancels.self_gatling))
                                    && is_allowed_cancel
                            }
                            Requirement::Meter(value) => state.meter >= *value,
                            Requirement::Spirit(value) => state.spirit_gauge >= *value,
                            Requirement::Airborne => flags.airborne,
                            Requirement::Grounded => !flags.airborne,
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
                        }
                    }
                    state.allowed_cancels = AllowedCancel::Always;
                    state.last_hit_using = None;
                    state.rebeat_chain.insert(command_id);

                    match command_id {
                        CommandId::BorderEscapeJump => {
                            state.extra_data = ExtraData::JumpDirection(DirectedAxis::from_facing(
                                input.last().unwrap().axis,
                                state.facing,
                            ));
                        }
                        CommandId::Jump | CommandId::SuperJump => {
                            state.extra_data = ExtraData::JumpDirection(DirectedAxis::from_facing(
                                input.last().unwrap().axis,
                                state.facing,
                            ));
                        }
                        CommandId::Fly => {
                            let mut dir =
                                DirectedAxis::from_facing(input.last().unwrap().axis, state.facing);
                            if dir.is_backward() {
                                state.facing = state.facing.invert();
                                dir = dir.invert();
                            }
                            state.extra_data =
                                ExtraData::FlyDirection(if dir == DirectedAxis::Neutral {
                                    DirectedAxis::Forward
                                } else {
                                    dir
                                });
                        }
                        _ => (),
                    }
                }

                ret
            }
        };
    }

    fn update_spirit(&mut self) {
        let (ref mut frame, ref mut move_id) = &mut self.state.current_state;
        let move_data = &self.data.states[move_id];
        let flags = &move_data.flags[*frame];

        if *move_id == MoveId::Fly {
            self.state.spirit_gauge -= 5; // TODO, move this spirit cost to an editor value
            if self.state.spirit_gauge <= 0 {
                *move_id = MoveId::FlyEnd;
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
        let (frame, move_id) = self.state.current_state;
        let flags = &self.data.states[&move_id].flags[frame];

        let base_velocity = if flags.reset_velocity {
            collision::Vec2::zeros()
        } else {
            self.state.velocity
        };

        // we only run gravity if the move doesn't want to reset velocity, because that [resetting velocity] means the move has a trajectory in mind
        let gravity = if !flags.reset_velocity
            && flags.airborne
            && move_id != MoveId::FlyStart
            && move_id != MoveId::Fly
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
    fn update_position(&mut self, play_area: &PlayArea) {
        let (frame, move_id) = self.state.current_state;
        let state = &self.data.states[&move_id];
        let flags = &state.flags[frame];
        let hitboxes = &state.hitboxes[frame];
        let collision = &hitboxes.collision;

        self.state.position += self.state.velocity;

        // handle landing
        if flags.airborne && self.state.position.y - collision.half_size.y <= -4 {
            let mut reset_hitstun = true;
            let mut reset_velocity = true;
            self.state.current_state = if state.state_type == StateType::Hitstun {
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
                self.state.extra_data = ExtraData::None;
            }
            if reset_velocity {
                self.state.velocity = collision::Vec2::zeros();
            }
            self.state.position.y = hitboxes.collision.half_size.y;
            self.state.air_actions = self.data.properties.max_air_actions;
        }

        self.validate_position(play_area);
    }

    fn spawn_objects(&mut self) {
        let (frame, move_id) = self.state.current_state;
        for spawner in self.data.states[&move_id]
            .spawns
            .iter()
            .filter(|item| item.frame == frame)
        {
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
        let (frame, move_id) = self.state.current_state;
        let sounds = &self.data.states[&move_id].sounds;

        for sound in sounds.iter().filter(|item| item.frame == frame) {
            self.state.sound_state.play_sound(sound.channel, sound.name);
        }
    }

    fn handle_combo_state(&mut self) {
        let (_, move_id) = self.state.current_state;
        let current_state_type = self.data.states[&move_id].state_type;
        crate::roster::impls::handle_combo_state(
            &mut self.state.current_combo,
            &mut self.last_combo_state,
            current_state_type,
        );
    }

    fn update_meter(&mut self, opponent_position: collision::Vec2) {
        let flags = self.current_flags();
        let move_type = self.data.states[&self.state.current_state.1].state_type;
        self.state.meter -= flags.meter_cost;

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
        if matches!(move_type, StateType::Movement) && facing_opponent {
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
        let flags = self.current_flags();
        if !flags.airborne {
            self.state.position.x += force;
        }
    }
    fn validate_position(&mut self, play_area: &PlayArea) {
        let (frame, move_id) = self.state.current_state;
        let state = &self.data.states[&move_id];
        let flags = &state.flags[frame];
        let hitboxes = &state.hitboxes[frame];
        let collision = &hitboxes.collision;
        // handle stage sides
        if i32::abs(self.state.position.x) > play_area.width / 2 - collision.half_size.x {
            self.state.position.x =
                i32::signum(self.state.position.x) * (play_area.width / 2 - collision.half_size.x);
        }

        // if not airborne, make sure the character is locked to the ground properly
        if !flags.airborne {
            self.state.position.y = hitboxes.collision.half_size.y;
        }
    }
    fn prune_bullets(&mut self, _play_area: &PlayArea) {}
    fn would_be_hit(
        &self,
        input: &[InputState],
        info: HitAction,
        old_effect: Option<HitEffect>,
    ) -> (Option<HitEffect>, Option<HitResult>) {
        let attack_info = &info.attack_info;
        let flags = self.current_flags();
        let state_type = self.data.states[&self.state.current_state.1].state_type;
        let axis = DirectedAxis::from_facing(input.last().unwrap().axis, self.state.facing);
        let new_hit_type = old_effect
            .as_ref()
            .and_then(|item| match item.hit_type {
                HitEffectType::Hit
                | HitEffectType::CounterHit
                | HitEffectType::GuardCrush
                | HitEffectType::GrazeCrush => {
                    Some(if item.effect.set_combo.unwrap().available_limit > 0 {
                        Some(HitEffectType::Hit)
                    } else {
                        None
                    })
                }
                HitEffectType::Block | HitEffectType::WrongBlock => {
                    Some(if attack_info.air_unblockable && flags.airborne {
                        Some(HitEffectType::Hit)
                    } else if flags.airborne || axis.is_guarding(attack_info.guard) {
                        Some(HitEffectType::Block)
                    } else {
                        Some(HitEffectType::WrongBlock)
                    })
                }
                HitEffectType::Graze => None,
            })
            .or_else(|| {
                Some(
                    if attack_info.magic && flags.bullet.is_invuln()
                        || attack_info.melee && flags.melee.is_invuln()
                        || self
                            .state
                            .current_combo
                            .map(|item| item.available_limit <= 0)
                            .unwrap_or(false)
                    {
                        None
                    } else if attack_info.grazeable && flags.grazing {
                        Some(HitEffectType::Graze)
                    } else if ( matches!(state_type, StateType::Blockstun)
                        || (flags.can_block
                            // this is crossup protection
                            // if the attack is facing the same direction you're facing
                            // then the attack should be able to be blocked by holding both back
                            // and forward.
                            && (axis.is_blocking(false) || axis.is_blocking(self.state.facing == info.facing))))
                        && !(attack_info.air_unblockable && flags.airborne)
                    {
                        if flags.airborne || axis.is_guarding(attack_info.guard) {
                            Some(HitEffectType::Block)
                        } else {
                            Some(HitEffectType::WrongBlock)
                        }
                    } else if flags.can_be_counter_hit && attack_info.can_counter_hit {
                        Some(HitEffectType::CounterHit)
                    } else {
                        Some(HitEffectType::Hit)
                    },
                )
            })
            .flatten();

        if new_hit_type.is_none() {
            return (old_effect, None);
        }

        let new_hit_type = new_hit_type.unwrap();
        let new_effect = match new_hit_type {
            HitEffectType::Graze => EffectData::graze(&info, flags.airborne).build(),
            HitEffectType::CounterHit => {
                EffectData::counter_hit(&info, self.state.current_combo, flags.airborne).build()
            }
            HitEffectType::Block => EffectData::block(&info, flags.airborne).build(),
            HitEffectType::WrongBlock => EffectData::wrong_block(&info).build(),
            HitEffectType::Hit => {
                EffectData::hit(&info, self.state.current_combo, flags.airborne).build()
            }
            HitEffectType::GuardCrush => unreachable!(),
            HitEffectType::GrazeCrush => unreachable!(),
        };

        let (new_effect, new_hit_type) = match old_effect {
            None => (new_effect, new_hit_type),
            Some(old_effect) => {
                let old_hit_type = old_effect.hit_type;
                let old_effect = old_effect.effect;
                match old_hit_type {
                    HitEffectType::Graze => match new_hit_type {
                        HitEffectType::Graze => (
                            old_effect
                                .into_builder()
                                .inherit_non_hit_data(&new_effect)
                                .set_stop(new_effect.set_stop.max(old_effect.set_stop))
                                .build(),
                            old_hit_type,
                        ),
                        HitEffectType::CounterHit | HitEffectType::Hit => (
                            new_effect
                                .into_builder()
                                .inherit_non_hit_data(&old_effect)
                                .inherit_spirit_delay(&old_effect)
                                .build(),
                            new_hit_type,
                        ),
                        HitEffectType::Block | HitEffectType::WrongBlock => (
                            new_effect
                                .into_builder()
                                .inherit_non_hit_data(&old_effect)
                                .build(),
                            new_hit_type,
                        ),
                        HitEffectType::GuardCrush | HitEffectType::GrazeCrush => unreachable!(),
                    },

                    HitEffectType::Hit
                    | HitEffectType::CounterHit
                    | HitEffectType::GuardCrush
                    | HitEffectType::GrazeCrush => match new_hit_type {
                        HitEffectType::Hit => {
                            assert!(old_effect.set_combo.unwrap().available_limit > 0);
                            (
                                old_effect.into_builder().apply_hit(&info).build(),
                                old_hit_type,
                            )
                        }
                        HitEffectType::GuardCrush
                        | HitEffectType::GrazeCrush
                        | HitEffectType::Block
                        | HitEffectType::WrongBlock
                        | HitEffectType::Graze
                        | HitEffectType::CounterHit => unreachable!(),
                    },
                    HitEffectType::Block => match new_hit_type {
                        HitEffectType::Hit => (
                            new_effect
                                .into_builder()
                                .inherit_non_hit_data(&old_effect)
                                .inherit_spirit_delay(&old_effect)
                                .build(),
                            new_hit_type,
                        ),
                        HitEffectType::Block => (
                            old_effect
                                .into_builder()
                                .inherit_non_hit_data(&new_effect)
                                .build(),
                            old_hit_type,
                        ),
                        HitEffectType::WrongBlock => (
                            new_effect
                                .into_builder()
                                .inherit_non_hit_data(&old_effect)
                                .build(),
                            new_hit_type,
                        ),
                        HitEffectType::GuardCrush
                        | HitEffectType::GrazeCrush
                        | HitEffectType::Graze
                        | HitEffectType::CounterHit => unreachable!(),
                    },
                    HitEffectType::WrongBlock => match new_hit_type {
                        HitEffectType::Hit => (
                            new_effect
                                .into_builder()
                                .inherit_non_hit_data(&old_effect)
                                .inherit_spirit_delay(&old_effect)
                                .build(),
                            new_hit_type,
                        ),
                        HitEffectType::Block | HitEffectType::WrongBlock => (
                            old_effect
                                .into_builder()
                                .inherit_non_hit_data(&new_effect)
                                .build(),
                            old_hit_type,
                        ),
                        HitEffectType::GuardCrush
                        | HitEffectType::GrazeCrush
                        | HitEffectType::Graze
                        | HitEffectType::CounterHit => unreachable!(),
                    },
                }
            }
        };

        let (effect, hit_type) = match new_hit_type {
            HitEffectType::Block | HitEffectType::WrongBlock
                if self.state.spirit_gauge - new_effect.take_spirit_gauge <= 0 =>
            {
                (
                    EffectData::guard_crush(&info, flags.airborne).build(),
                    HitEffectType::GuardCrush,
                )
            }
            _ => (new_effect, new_hit_type),
        };

        (
            Some(HitEffect { hit_type, effect }),
            Some(HitResult {
                hit_type,
                action: info,
            }),
        )
    }
    fn take_hit(&mut self, info: HitEffect, play_area: &PlayArea) {
        let flags = self.current_flags();

        let hit_type = info.hit_type;
        let effect = info.effect;

        let crouching = flags.crouching;

        self.state.health -= effect.take_damage;

        if self.state.health <= 0 && effect.is_lethal {
            self.state.dead = true;
        }

        let airborne = match effect.set_force {
            Force::Airborne(_) => true,
            Force::Grounded(_) => false,
        } || self.state.dead;

        self.state.should_pushback = effect.set_should_pushback;
        self.state.spirit_gauge -= effect.take_spirit_gauge;
        self.state.meter += effect.modify_meter;

        self.state.spirit_delay = if effect.reset_spirit_delay {
            0
        } else {
            self.state.spirit_delay
        } + effect.add_spirit_delay;

        self.state.hitstop = effect.set_stop;

        match hit_type {
            HitEffectType::Graze => {}
            _ => {
                self.state.extra_data = ExtraData::Stun(effect.set_stun);
                self.state.velocity = match effect.set_force {
                    Force::Airborne(value) | Force::Grounded(value) => value,
                };
            }
        }

        self.state.current_combo = effect.set_combo;

        match hit_type {
            HitEffectType::Graze => (),
            HitEffectType::WrongBlock => {
                if crouching {
                    self.state.current_state = (0, MoveId::WrongblockStandStart);
                } else {
                    self.state.current_state = (0, MoveId::WrongblockCrouchStart);
                }
            }
            HitEffectType::Block => {
                if airborne {
                    self.state.current_state = (0, MoveId::BlockstunAirStart);
                } else if crouching {
                    self.state.current_state = (0, MoveId::BlockstunCrouchStart);
                } else {
                    self.state.current_state = (0, MoveId::BlockstunStandStart);
                }
            }
            HitEffectType::Hit | HitEffectType::CounterHit | HitEffectType::GrazeCrush => {
                if airborne {
                    self.state.current_state = (0, MoveId::HitstunAirStart);
                } else {
                    self.state.current_state = (0, MoveId::HitstunStandStart);
                }
            }
            HitEffectType::GuardCrush => {
                if airborne {
                    self.state.current_state = (0, MoveId::HitstunAirStart);
                } else {
                    self.state.current_state = (0, MoveId::GuardCrush);
                }
            }
        }

        self.validate_position(play_area);

        if hit_type == HitEffectType::GuardCrush {
            self.state.spirit_gauge = self.data.properties.max_spirit_gauge
        }
    }
    fn deal_hit(&mut self, info: &HitResult) {
        match info.hit_type {
            HitEffectType::Hit => {
                self.state
                    .sound_state
                    .play_sound(ChannelName::Hit, GlobalSound::Hit.into());
            }
            HitEffectType::CounterHit => {
                self.state
                    .sound_state
                    .play_sound(ChannelName::Hit, GlobalSound::CounterHit.into());
            }
            HitEffectType::GuardCrush => {
                self.state
                    .sound_state
                    .play_sound(ChannelName::Hit, GlobalSound::GuardCrush.into());
            }
            HitEffectType::Block => {
                self.state
                    .sound_state
                    .play_sound(ChannelName::Hit, GlobalSound::Block.into());
            }
            HitEffectType::WrongBlock => {
                self.state
                    .sound_state
                    .play_sound(ChannelName::Hit, GlobalSound::WrongBlock.into());
            }
            _ => (),
        }
        match info.hit_type {
            HitEffectType::Hit => self.state.meter += info.action.attack_info.on_hit.attacker_meter,
            HitEffectType::CounterHit => {
                self.state.meter += info.action.attack_info.on_counter_hit.attacker_meter
            }
            HitEffectType::GuardCrush => {
                self.state.meter += info.action.attack_info.on_guard_crush.attacker_meter
            }
            HitEffectType::Graze => {
                self.state.meter += info.action.attack_info.on_graze.attacker_meter
            }
            HitEffectType::Block => {
                self.state.meter += info.action.attack_info.on_block.attacker_meter
            }
            HitEffectType::WrongBlock => {
                self.state.meter += info.action.attack_info.on_wrongblock.attacker_meter
            }
            HitEffectType::GrazeCrush => {
                //self.state.meter += info.action.attack_info.on_graze_crush.attacker_meter
            }
        }

        if info.action.source == HitSource::Character {
            self.state.last_hit_using = Some(info.action.hash);
        }

        match info.hit_type {
            HitEffectType::Hit
            | HitEffectType::CounterHit
            | HitEffectType::GuardCrush
            | HitEffectType::GrazeCrush => {
                if info.action.source == HitSource::Character {
                    self.state.allowed_cancels = AllowedCancel::Hit;
                    self.state.hitstop = info.action.attack_info.on_hit.attacker_stop;
                }
            }
            HitEffectType::Block | HitEffectType::WrongBlock => {
                if info.action.source == HitSource::Character {
                    self.state.allowed_cancels = AllowedCancel::Block;
                    self.state.hitstop = info.action.attack_info.on_block.attacker_stop;
                }
            }
            HitEffectType::Graze => {}
        }
    }

    fn handle_refacing(&mut self, other_player: collision::Int) {
        let (frame, move_id) = self.state.current_state;
        let flags = &self.data.states[&move_id].flags[frame];
        crate::roster::generic_character::impls::handle_refacing(
            &mut self.state.facing,
            flags,
            &self.state.position,
            other_player,
        );
    }
    fn update_frame_mut(
        &mut self,
        input: &[InputState],
        opponent_position: collision::Vec2,
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
        self.update_lockout();
        self.update_meter(opponent_position);
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
            &self.last_combo_state,
            self.combo_text.borrow_mut().deref_mut(),
            self.state.health,
            self.state.spirit_gauge,
            self.state.meter,
            self.state.lockout,
            &self.data.properties,
        )
    }

    fn draw(&self, ctx: &mut Context, assets: &Assets, world: graphics::Matrix4) -> GameResult<()> {
        let (frame, move_id) = self.state.current_state;

        let collision = &self.data.states[&move_id].hitboxes[frame].collision;

        self.data.states[&move_id].draw_at_time(
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

        self.data.states[&move_id].draw_shadow_at_time(
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
    fn get_attack_data(&self) -> Option<HitAction> {
        let (frame, move_id) = &self.state.current_state;

        self.data.states[move_id].hitboxes[*frame]
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
                HitAction {
                    facing: self.state.facing,
                    attack_info: self.data.attacks[&item.data_id].clone(),
                    hash: hasher.finish(),
                    source: HitSource::Character,
                }
            })
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
    fn render_sound(
        &mut self,
        audio_device: &Device,
        sound_list: &SoundList<GlobalSound>,
        fps: u32,
    ) {
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
        self.data.states[&move_id]
            .flags
            .get(current_frame + 1)
            .map(|item| item.1.cutscene)
            .unwrap_or(false)
    }

    fn get_flash(&self) -> Option<FlashType> {
        self.current_flags().flash
    }

    fn get_lockout(&self) -> (i32, bool) {
        let flags = self.current_flags();
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
        self.state = YuyukoState {
            position: collision::Vec2::new(position, 0),
            velocity: collision::Vec2::zeros(),
            current_state: (0, MoveId::Stand),
            extra_data: ExtraData::None,
            air_actions: self.data.properties.max_air_actions,
            spirit_gauge: 0,
            spirit_delay: 0,
            hitstop: 0,
            facing,
            last_hit_using: None,
            health: self.data.properties.health,
            current_combo: None,
            allowed_cancels: AllowedCancel::Always,
            rebeat_chain: HashSet::new(),
            should_pushback: true,
            sound_state: PlayerSoundState::new(),
            meter: 0,
            lockout: 0,
            dead: false,
        };
        self.validate_position(play_area);
    }
    fn reset_to_position_gamestart(
        &mut self,
        play_area: &PlayArea,
        position: collision::Int,
        facing: Facing,
    ) {
        self.state = YuyukoState {
            position: collision::Vec2::new(position, 0),
            velocity: collision::Vec2::zeros(),
            current_state: (0, MoveId::RoundStart),
            extra_data: ExtraData::None,
            air_actions: self.data.properties.max_air_actions,
            spirit_gauge: 0,
            spirit_delay: 0,
            hitstop: 0,
            facing,
            last_hit_using: None,
            health: self.data.properties.health,
            current_combo: None,
            allowed_cancels: AllowedCancel::Always,
            rebeat_chain: HashSet::new(),
            should_pushback: true,
            sound_state: PlayerSoundState::new(),
            meter: 0,
            lockout: 0,
            dead: false,
        };

        self.validate_position(play_area);
    }

    fn health(&self) -> i32 {
        self.state.health
    }
}
