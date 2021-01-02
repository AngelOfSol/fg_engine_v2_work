pub mod attacks;
pub mod moves;

use super::{
    hit_info::{
        block, counter_hit, graze, guard_crush, hit, wrong_block, ComboEffect, HitResultNew,
        OnHitEffect, OnHitType, Source,
    },
    PlayerState,
};
use crate::game_match::sounds::PlayerSoundState;
use crate::game_object::state::Timer;
use crate::graphics::animation_group::AnimationGroup;
use crate::hitbox::PositionedHitbox;
use crate::input::button::Button;
use crate::input::{read_inputs, DirectedAxis, Facing, InputState};
use crate::roster::generic_character::hit_info::Force;
use crate::roster::generic_character::AllowedCancel;
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
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::hash::Hash;
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
    #[tab = "State to Graphics"]
    pub state_graphics_map: HashMap<MoveId, YuyukoGraphic>,
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
            state_graphics_map: data.state_graphics_map,
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
    state_graphics_map: HashMap<MoveId, YuyukoGraphic>,
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
    Debug,
    Copy,
    Clone,
    Hash,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    EnumIter,
    Display,
    Inspect,
    PartialOrd,
    Ord,
)]
#[serde(rename_all = "snake_case")]
pub enum YuyukoGraphic {
    SuperJumpParticle,
    HitEffect,
    Butterfly1,
    Butterfly2,
    Butterfly3,
    Butterfly4,
    Stand,
    WalkBackward,
    WalkForward,
    Attack5A,
    Attack2A,
    Attack5B,
    Attack3B,
    Attack2B,
    Attack6B,
    Attack5C,
    Attack2C,
    Air5A,
    Air8A,
    Air5B,
    Air2B,
    Air5C,
    Air2C,
    Crouch,
    ToCrouch,
    ToStand,
    ForwardDashStart,
    ForwardDash,
    ForwardDashEnd,
    BackDash,
    Jump,
    AirIdle,
    Fly,
    FlyStart,
    FlyEnd,
    HitstunStandStart,
    HitstunStandLoop,
    HitstunAirStart,
    HitstunAirMid1,
    HitstunAirMid2,
    HitstunAirLoop,
    BlockstunAirStart,
    BlockstunAirLoop,
    BlockstunCrouchStart,
    BlockstunCrouchLoop,
    BlockstunStandStart,
    BlockstunStandLoop,
    WrongblockCrouchStart,
    WrongblockCrouchLoop,
    WrongblockStandStart,
    WrongblockStandLoop,
    HitGround,
    GetUp,
    MeleeRestitution,
    GuardCrush,
    RoundStart,
    Dead,
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
    pub last_combo_state: Option<(ComboEffect, usize)>,
    pub state: YuyukoState,
    pub combo_text: RefCell<Option<ggez::graphics::Text>>,
}

use std::cell::RefCell;

pub type YuyukoState = PlayerState<MoveId, YuyukoSound, CommandId, AttackId>;

impl YuyukoState {
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
            last_hit_using: None,
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
            last_hit_using_new: None,
            current_combo_new: None,
        }
    }
}

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
            self.state.last_hit_using_new = None;
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

        if let Some(ref mut stun) = self.state.stun {
            assert!(matches!(
                state_type,
                StateType::Blockstun | StateType::Hitstun
            ));
            *stun -= 1;
            if *stun == 0 {
                self.state.stun = None;

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
                        (0, MoveId::Untech)
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
                            crate::character::command::Effect::FlipFacing => {
                                state.facing = state.facing.invert();
                            }
                        }
                    }
                    state.allowed_cancels = AllowedCancel::Always;
                    state.last_hit_using = None;
                    state.last_hit_using_new = None;
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
        let gravity = if flags.gravity && flags.airborne {
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

        let accel = self.state.facing.fix_collision(flags.accel);
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
                let combo = self.state.current_combo_new.as_mut().unwrap();
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
            &mut self.state.current_combo_new,
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

    fn would_be_hit_new(
        &self,
        input: &[InputState],
        attack_info: &AttackInfo,
        source: &Source,
        combo_effect: Option<&ComboEffect>,
        old_effect: Option<OnHitEffect>,
    ) -> HitResultNew {
        let flags = self.current_flags();
        let state_type = self.data.states[&self.state.current_state.1].state_type;
        let axis = DirectedAxis::from_facing(input.last().unwrap().axis, self.state.facing);
        match old_effect {
            Some(effect) => match effect {
                OnHitEffect::Hit(effect) => {
                    if effect.combo.available_limit > 0 {
                        effect.append_hit(attack_info, source).into()
                    } else {
                        effect.into()
                    }
                }
                OnHitEffect::GuardCrush(effect) => {
                    if effect.combo.available_limit > 0 {
                        effect.append_hit(attack_info).into()
                    } else {
                        effect.into()
                    }
                }
                OnHitEffect::CounterHit(effect) => {
                    if effect.combo.available_limit > 0 {
                        effect.append_hit(attack_info).into()
                    } else {
                        effect.into()
                    }
                }
                OnHitEffect::Graze(effect) => {
                    if attack_info.magic && flags.bullet.is_invuln()
                        || attack_info.melee && flags.melee.is_invuln()
                        || attack_info.air && flags.air.is_invuln()
                        || attack_info.foot && flags.foot.is_invuln()
                    {
                        effect.into()
                    } else if attack_info.grazeable {
                        effect.append_graze(attack_info).into()
                    } else if (flags.can_block
                        && (axis.is_blocking(false)
                            || axis.is_blocking(self.state.facing == source.facing)))
                        && !(attack_info.air_unblockable && flags.airborne)
                    {
                        if flags.airborne || axis.is_guarding(attack_info.guard) {
                            if block::Effect::would_crush(
                                Some(effect.defender.take_spirit_gauge),
                                attack_info,
                                self.state.spirit_gauge,
                            ) {
                                effect
                                    .append_guard_crush(attack_info, source, flags.airborne)
                                    .into()
                            } else {
                                effect
                                    .append_block(attack_info, source, flags.airborne)
                                    .into()
                            }
                        } else if wrong_block::Effect::would_crush(
                            Some(effect.defender.take_spirit_gauge),
                            attack_info,
                            self.state.spirit_gauge,
                        ) {
                            effect
                                .append_guard_crush(attack_info, source, flags.airborne)
                                .into()
                        } else {
                            effect.append_wrongblock(attack_info, source).into()
                        }
                    } else if flags.can_be_counter_hit && attack_info.can_counter_hit {
                        effect
                            .append_counterhit(attack_info, source, flags.airborne)
                            .into()
                    } else {
                        effect
                            .append_hit(attack_info, source, flags.airborne)
                            .into()
                    }
                }
                OnHitEffect::Block(effect) => {
                    if !(attack_info.air_unblockable && flags.airborne) {
                        if flags.airborne || axis.is_guarding(attack_info.guard) {
                            if block::Effect::would_crush(
                                Some(effect.defender.take_spirit_gauge),
                                attack_info,
                                self.state.spirit_gauge,
                            ) {
                                effect
                                    .append_guard_crush(attack_info, source, flags.airborne)
                                    .into()
                            } else {
                                effect
                                    .append_block(attack_info, source, flags.airborne)
                                    .into()
                            }
                        } else if wrong_block::Effect::would_crush(
                            Some(effect.defender.take_spirit_gauge),
                            attack_info,
                            self.state.spirit_gauge,
                        ) {
                            effect
                                .append_guard_crush(attack_info, source, flags.airborne)
                                .into()
                        } else {
                            effect.append_wrongblock(attack_info, source).into()
                        }
                    } else {
                        effect
                            .append_hit(attack_info, source, flags.airborne)
                            .into()
                    }
                }
                OnHitEffect::WrongBlock(effect) => {
                    if axis.is_guarding(attack_info.guard) {
                        if block::Effect::would_crush(
                            Some(effect.defender.take_spirit_gauge),
                            attack_info,
                            self.state.spirit_gauge,
                        ) {
                            effect
                                .append_guard_crush(attack_info, source, flags.airborne)
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
                            .append_guard_crush(attack_info, source, flags.airborne)
                            .into()
                    } else {
                        effect.append_wrongblock(attack_info, source).into()
                    }
                }
            },
            None => {
                if attack_info.magic && flags.bullet.is_invuln()
                    || attack_info.melee && flags.melee.is_invuln()
                    || attack_info.air && flags.air.is_invuln()
                    || attack_info.foot && flags.foot.is_invuln()
                    || self
                        .state
                        .current_combo_new
                        .as_ref()
                        .map(|item| item.available_limit <= 0)
                        .unwrap_or(false)
                {
                    HitResultNew::None
                } else if attack_info.grazeable && flags.grazing {
                    graze::Effect::build(attack_info).into()
                } else if (matches!(state_type, StateType::Blockstun)
                    || (flags.can_block
                    // this is crossup protection
                    // if the attack is facing the same direction you're facing
                    // then the attack should be able to be blocked by holding both back
                    // and forward.
                    && (axis.is_blocking(false) || axis.is_blocking(self.state.facing == source.facing))))
                    && !(attack_info.air_unblockable && flags.airborne)
                {
                    if flags.airborne || axis.is_guarding(attack_info.guard) {
                        if block::Effect::would_crush(None, attack_info, self.state.spirit_gauge) {
                            guard_crush::Effect::build(attack_info, source, flags.airborne).into()
                        } else {
                            block::Effect::build(attack_info, source, flags.airborne).into()
                        }
                    } else if wrong_block::Effect::would_crush(
                        None,
                        attack_info,
                        self.state.spirit_gauge,
                    ) {
                        guard_crush::Effect::build(attack_info, source, flags.airborne).into()
                    } else {
                        wrong_block::Effect::build(attack_info, source).into()
                    }
                } else if flags.can_be_counter_hit && attack_info.can_counter_hit {
                    counter_hit::Effect::build(attack_info, source, flags.airborne).into()
                } else {
                    combo_effect
                        .map(|combo| {
                            if combo.available_limit > 0 {
                                hit::Effect::build(
                                    attack_info,
                                    source,
                                    flags.airborne,
                                    combo.clone(),
                                )
                                .into()
                            } else {
                                HitResultNew::None
                            }
                        })
                        .unwrap_or_else(|| {
                            hit::Effect::build_starter(attack_info, source, flags.airborne).into()
                        })
                }
            }
        }
    }

    fn take_hit_new(&mut self, info: &OnHitEffect, play_area: &PlayArea) {
        let airborne = match info {
            OnHitEffect::Hit(hit::Effect {
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
            | OnHitEffect::CounterHit(counter_hit::Effect {
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
            | OnHitEffect::GuardCrush(guard_crush::Effect {
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
            | OnHitEffect::Block(block::Effect {
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
            | OnHitEffect::WrongBlock(wrong_block::Effect {
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

            OnHitEffect::Graze(effect) => {
                let effect = &effect.defender;

                self.state.health -= effect.take_damage;

                self.state.hitstop = effect.set_stop;
                self.current_flags().airborne
            }
        };

        match info {
            OnHitEffect::Hit(hit::Effect { combo, .. })
            | OnHitEffect::GuardCrush(guard_crush::Effect { combo, .. })
            | OnHitEffect::CounterHit(counter_hit::Effect { combo, .. }) => {
                self.state.current_combo_new = Some(combo.clone());
            }
            _ => {}
        }

        match info {
            OnHitEffect::Hit(hit::Effect {
                defender:
                    hit::DefenderEffect {
                        take_spirit_gauge,
                        reset_spirit_delay,
                        add_spirit_delay,
                        ..
                    },
                ..
            })
            | OnHitEffect::CounterHit(counter_hit::Effect {
                defender:
                    counter_hit::DefenderEffect {
                        take_spirit_gauge,
                        reset_spirit_delay,
                        add_spirit_delay,
                        ..
                    },
                ..
            })
            | OnHitEffect::Graze(graze::Effect {
                defender:
                    graze::DefenderEffect {
                        take_spirit_gauge,
                        reset_spirit_delay,
                        add_spirit_delay,
                        ..
                    },
                ..
            })
            | OnHitEffect::Block(block::Effect {
                defender:
                    block::DefenderEffect {
                        take_spirit_gauge,
                        reset_spirit_delay,
                        add_spirit_delay,
                        ..
                    },
                ..
            })
            | OnHitEffect::WrongBlock(wrong_block::Effect {
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
            OnHitEffect::GuardCrush(_) => {
                self.state.spirit_gauge = self.data.properties.max_spirit_gauge;
            }
        }

        match info {
            OnHitEffect::GuardCrush(_) => {
                if airborne {
                    self.state.current_state = (0, MoveId::HitstunAirStart);
                } else {
                    self.state.current_state = (0, MoveId::GuardCrush);
                }
            }
            OnHitEffect::Hit(_) | OnHitEffect::CounterHit(_) => {
                if airborne {
                    self.state.current_state = (0, MoveId::HitstunAirStart);
                } else {
                    self.state.current_state = (0, MoveId::HitstunStandStart);
                }
            }
            OnHitEffect::Block(_) => {
                if airborne {
                    self.state.current_state = (0, MoveId::BlockstunAirStart);
                } else if self.current_flags().crouching {
                    self.state.current_state = (0, MoveId::BlockstunCrouchStart);
                } else {
                    self.state.current_state = (0, MoveId::BlockstunStandStart);
                }
            }
            OnHitEffect::WrongBlock(_) => {
                if self.current_flags().crouching {
                    self.state.current_state = (0, MoveId::WrongblockStandStart);
                } else {
                    self.state.current_state = (0, MoveId::WrongblockCrouchStart);
                }
            }
            OnHitEffect::Graze(_) => {}
        }

        self.validate_position(play_area);
    }

    fn deal_hit_new(&mut self, info: &OnHitType) {
        match info {
            OnHitType::Hit => {
                self.state
                    .sound_state
                    .play_sound(ChannelName::Hit, GlobalSound::Hit.into());
            }
            OnHitType::GuardCrush => {
                self.state
                    .sound_state
                    .play_sound(ChannelName::Hit, GlobalSound::GuardCrush.into());
            }
            OnHitType::CounterHit => {
                self.state
                    .sound_state
                    .play_sound(ChannelName::Hit, GlobalSound::CounterHit.into());
            }
            OnHitType::Block => {
                self.state
                    .sound_state
                    .play_sound(ChannelName::Hit, GlobalSound::Block.into());
            }
            OnHitType::WrongBlock => {
                self.state
                    .sound_state
                    .play_sound(ChannelName::Hit, GlobalSound::WrongBlock.into());
            }
            OnHitType::Graze => {}
        }
        let (frame, move_id) = &self.state.current_state;
        let hitbox = self.data.states[move_id].hitboxes[*frame]
            .hitbox
            .as_ref()
            .unwrap();
        let attack_info = &self.data.attacks[&hitbox.data_id];

        self.state.meter += match info {
            OnHitType::Hit => attack_info.on_hit.attacker_meter,
            OnHitType::GuardCrush => attack_info.on_guard_crush.attacker_meter,
            OnHitType::CounterHit => attack_info.on_counter_hit.attacker_meter,
            OnHitType::Graze => attack_info.on_graze.attacker_meter,
            OnHitType::Block => attack_info.on_block.attacker_meter,
            OnHitType::WrongBlock => attack_info.on_wrongblock.attacker_meter,
        };
        self.state.hitstop = match info {
            OnHitType::Hit => attack_info.on_hit.attacker_stop,
            OnHitType::GuardCrush => attack_info.on_guard_crush.attacker_stop,
            OnHitType::CounterHit => attack_info.on_counter_hit.attacker_stop,
            OnHitType::Graze => 0,
            OnHitType::Block => attack_info.on_block.attacker_stop,
            OnHitType::WrongBlock => attack_info.on_wrongblock.attacker_stop,
        };

        self.state.last_hit_using_new = Some((hitbox.data_id, hitbox.id));

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
            OnHitType::Hit | OnHitType::GuardCrush | OnHitType::CounterHit => {
                self.state.allowed_cancels = AllowedCancel::Hit;
            }
            OnHitType::Graze => {}
            OnHitType::Block | OnHitType::WrongBlock => {
                self.state.allowed_cancels = AllowedCancel::Block;
            }
        }
    }

    fn get_current_combo(&self) -> Option<&ComboEffect> {
        self.state.current_combo_new.as_ref()
    }

    fn get_attack_data_new(&self) -> Option<&AttackInfo> {
        let (frame, move_id) = &self.state.current_state;
        self.data.states[move_id].hitboxes[*frame]
            .hitbox
            .as_ref()
            .and_then(|hitbox| {
                if Some((hitbox.data_id, hitbox.id)) != self.state.last_hit_using_new {
                    Some(&self.data.attacks[&hitbox.data_id])
                } else {
                    None
                }
            })
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
        self.data.states[&move_id].flags[current_frame + 1].cutscene
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
            stun: None,
            air_actions: self.data.properties.max_air_actions,
            spirit_gauge: 0,
            spirit_delay: 0,
            hitstop: 0,
            facing,
            last_hit_using: None,
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
            last_hit_using_new: None,
            current_combo_new: None,
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
            stun: None,
            air_actions: self.data.properties.max_air_actions,
            spirit_gauge: 0,
            spirit_delay: 0,
            hitstop: 0,
            facing,
            last_hit_using: None,
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
            last_hit_using_new: None,
            current_combo_new: None,
        };

        self.validate_position(play_area);
    }

    fn health(&self) -> i32 {
        self.state.health
    }
}
