mod attacks;
mod bullets;
mod command_list;
mod moves;
mod particles;

use crate::assets::Assets;
use crate::character::components::Properties;
use crate::character::components::{AttackInfo, GroundAction};
use crate::character::state::components::{Flags, GlobalParticle, MoveType, ParticlePath};
use crate::character::state::State;
use crate::command_list::CommandList;
use crate::game_match::sounds::SoundPath;
use crate::game_match::sounds::{ChannelName, GlobalSound, SoundList, SoundRenderer};
use crate::game_match::{FlashType, PlayArea, UiElements};
use crate::graphics::particle::Particle;
use crate::graphics::Animation;
use crate::hitbox::Hitbox;
use crate::hitbox::PositionedHitbox;
use crate::input::button::Button;
use crate::input::{read_inputs, DirectedAxis, Facing, InputState};
use crate::roster::generic_character::bullet::{GenericBulletSpawn, GenericBulletState};
use crate::roster::generic_character::combo_state::{AllowedCancel, ComboState};
use crate::roster::generic_character::extra_data::ExtraData;
use crate::roster::generic_character::hit_info::{
    EffectData, Force, HitAction, HitEffect, HitEffectType, HitResult, HitSource,
};
use crate::roster::generic_character::GenericCharacterBehaviour;
use crate::roster::generic_character::OpaqueStateData;
use crate::timeline::AtTime;
use crate::typedefs::collision;
use crate::typedefs::graphics;
use attacks::AttackId;
use bullets::BulletSpawn;
pub use bullets::BulletState;
use ggez::{Context, GameResult};
use moves::MoveId;
use particles::ParticleId;
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
use strum_macros::{Display, EnumIter};

#[derive(Clone, Debug, Deserialize)]
pub struct BulletData {
    pub animation: Animation,
    pub hitbox: Hitbox,
    pub attack_id: AttackId,
}
#[derive(Clone, Debug, Deserialize)]
pub struct BulletList {
    pub butterfly: BulletData,
}

#[derive(Clone)]
pub struct Yuyuko {
    pub states: StateList,
    pub particles: ParticleList,
    pub bullets: BulletList,
    pub attacks: AttackList,
    pub properties: Properties,
    pub command_list: CommandList<MoveId>,
    pub sounds: SoundList<YuyukoSound>,
}
impl std::fmt::Debug for Yuyuko {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.states)
    }
}

type StateList = HashMap<MoveId, State<MoveId, ParticleId, BulletSpawn, AttackId, YuyukoSound>>;
type ParticleList = HashMap<ParticleId, Particle>;
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
            particles: data.particles,
            properties: data.properties,
            attacks: data.attacks,
            bullets: data.bullets,
            command_list: command_list::generate_command_list(),
            sounds: data.sounds,
        })
    }
}

#[derive(Deserialize)]
pub struct YuyukoData {
    states: StateList,
    particles: ParticleList,
    bullets: BulletList,
    properties: Properties,
    attacks: AttackList,
    #[serde(skip)]
    #[serde(default = "SoundList::new")]
    sounds: SoundList<YuyukoSound>,
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

        path.push("particles");
        for (name, particle) in character.particles.iter_mut() {
            path.push(name.file_name());
            Particle::load(ctx, assets, particle, path.clone())?;
            path.pop();
        }
        path.pop();
        path.push("bullets");
        Animation::load(
            ctx,
            assets,
            &mut character.bullets.butterfly.animation,
            path.clone(),
        )?;

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

        Ok(character)
    }
}
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize, EnumIter, Display)]
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

pub struct YuyukoPlayer {
    pub data: Rc<Yuyuko>,
    pub sound_renderer: SoundRenderer<SoundPath<YuyukoSound>>,
    pub last_combo_state: Option<(ComboState, usize)>,
    pub state: YuyukoState,
    pub combo_text: RefCell<Option<ggez::graphics::Text>>,
}

use std::cell::RefCell;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YuyukoState {
    pub velocity: collision::Vec2,
    pub position: collision::Vec2,
    pub current_state: (usize, MoveId),
    pub extra_data: ExtraData,
    pub particles: Vec<(usize, collision::Vec2, ParticlePath<ParticleId>)>,
    pub bullets: Vec<BulletState>,
    pub facing: Facing,
    pub air_actions: usize,
    pub spirit_gauge: i32,
    pub spirit_delay: i32,
    pub hitstop: i32,
    pub last_hit_using: Option<u64>,
    pub current_combo: Option<ComboState>,
    pub health: i32,
    pub allowed_cancels: AllowedCancel,
    pub rebeat_chain: HashSet<MoveId>,
    pub should_pushback: bool,
    pub sound_state: PlayerSoundState<SoundPath<YuyukoSound>>,
    pub meter: i32,
    pub lockout: i32,
    pub dead: bool,
}

impl YuyukoState {
    fn new(data: &Yuyuko) -> Self {
        Self {
            velocity: collision::Vec2::zeros(),
            position: collision::Vec2::zeros(),
            current_state: (0, MoveId::RoundStart),
            extra_data: ExtraData::None,
            particles: Vec::new(),
            bullets: Vec::new(),
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

impl YuyukoPlayer {
    pub fn new(data: Rc<Yuyuko>) -> Self {
        Self {
            state: YuyukoState::new(&data),
            data,
            last_combo_state: None,
            sound_renderer: SoundRenderer::new(),
            combo_text: RefCell::new(None),
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
    fn on_enter_move(&mut self, input: &[InputState], move_id: MoveId) {
        self.state.allowed_cancels = AllowedCancel::Always;
        self.state.last_hit_using = None;
        self.state.rebeat_chain.insert(move_id);

        match move_id {
            MoveId::BorderEscapeJump => {
                self.state.extra_data = ExtraData::JumpDirection(DirectedAxis::from_facing(
                    input.last().unwrap().axis,
                    self.state.facing,
                ));
            }
            MoveId::Jump | MoveId::SuperJump => {
                self.state.extra_data = ExtraData::JumpDirection(DirectedAxis::from_facing(
                    input.last().unwrap().axis,
                    self.state.facing,
                ));
            }
            MoveId::FlyStart => {
                self.state.air_actions -= 1;
                let mut dir =
                    DirectedAxis::from_facing(input.last().unwrap().axis, self.state.facing);
                if dir.is_backward() {
                    self.state.facing = self.state.facing.invert();
                    dir = dir.invert();
                }
                self.state.extra_data = ExtraData::FlyDirection(if dir == DirectedAxis::Neutral {
                    DirectedAxis::Forward
                } else {
                    dir
                });
            }
            _ => (),
        }
    }
    fn in_corner(&self, play_area: &PlayArea) -> bool {
        let collision = self.collision();
        i32::abs(self.state.position.x) >= play_area.width / 2 - collision.half_size.x
    }
    fn current_flags(&self) -> &Flags {
        let (frame, move_id) = self.state.current_state;
        self.data.states[&move_id].flags.at_time(frame)
    }
    fn handle_rebeat_data(&mut self) {
        let (_, move_id) = self.state.current_state;

        if !self.data.states[&move_id].state_type.is_attack() {
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
                (0, self.data.states[&move_id].on_expire_state)
            }
        } else {
            (frame + 1, move_id)
        };
    }
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
                            MoveId::Crouch
                        } else {
                            MoveId::Stand
                        },
                    );
                } else {
                    self.state.current_state = if state_type.is_blockstun() {
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
        let cancels = self.data.states[&move_id].cancels.at_time(frame);
        let flags = self.data.states[&move_id].flags.at_time(frame);
        let state_type = self.data.states[&move_id].state_type;

        self.state.current_state = {
            let inputs = read_inputs(
                input.iter().rev(),
                self.state.facing,
                state_type.buffer_window(),
            );
            if move_id == MoveId::Fly {
                if input.last().unwrap()[Button::E].is_pressed() {
                    (frame, move_id)
                } else {
                    (0, MoveId::FlyEnd)
                }
            } else {
                let possible_new_move = self
                    .data
                    .command_list
                    .get_commands(&inputs)
                    .into_iter()
                    .copied()
                    .filter(|new_move_id| {
                        let new_state_type = &self.data.states[&new_move_id].state_type;

                        let is_self = *new_move_id == move_id;

                        let is_allowed_cancel = match self.state.allowed_cancels {
                            AllowedCancel::Hit => cancels.hit.contains(new_state_type),
                            AllowedCancel::Block => cancels.block.contains(new_state_type),
                            AllowedCancel::Always => false,
                        } || cancels.always.contains(new_state_type)
                            && !cancels.disallow.contains(&new_move_id);

                        let can_rebeat = !self.state.rebeat_chain.contains(&new_move_id);

                        let has_air_actions = self.state.air_actions != 0;

                        let has_required_spirit = self.state.spirit_gauge
                            >= self.data.states[&new_move_id].minimum_spirit_required;

                        let has_required_meter = self.state.meter
                            >= self.data.states[&new_move_id].minimum_meter_required;

                        let in_blockstun = state_type == MoveType::Blockstun;

                        let locked_out = self.state.lockout > 0;

                        let grounded = !flags.airborne;

                        match *new_move_id {
                            MoveId::BorderEscapeJump => {
                                in_blockstun && grounded && has_required_meter && !locked_out
                            }
                            MoveId::MeleeRestitution => {
                                in_blockstun && grounded && has_required_meter && !locked_out
                            }
                            MoveId::FlyStart => !is_self && is_allowed_cancel && has_air_actions,
                            _ => {
                                ((!is_self && can_rebeat) || (is_self && cancels.self_gatling))
                                    && is_allowed_cancel
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
    fn update_particles(&mut self, global_particles: &HashMap<GlobalParticle, Particle>) {
        let (frame, move_id) = self.state.current_state;
        let particle_data = &self.data.particles;
        let state_particles = &self.data.states[&move_id].particles;

        for (ref mut frame, _, _) in self.state.particles.iter_mut() {
            *frame += 1;
        }

        self.state
            .particles
            .retain(|item| item.0 < item.2.get(particle_data, global_particles).duration());

        for (particle_id, position) in state_particles
            .iter()
            .filter(|item| item.frame == frame)
            .map(|particle| (particle.particle_id, self.state.position + particle.offset))
            .collect::<Vec<_>>()
        {
            self.state.particles.push((0, position, particle_id));
        }
    }
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
    fn update_spirit(&mut self) {
        let (ref mut frame, ref mut move_id) = &mut self.state.current_state;
        let move_data = &self.data.states[move_id];
        let flags = move_data.flags.at_time(*frame);

        if move_data.state_type == MoveType::Fly && *move_id != MoveId::FlyEnd {
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
        let flags = self.data.states[&move_id].flags.at_time(frame);

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
        let flags = state.flags.at_time(frame);
        let hitboxes = state.hitboxes.at_time(frame);
        let collision = &hitboxes.collision;

        self.state.position += self.state.velocity;

        // handle landing
        if flags.airborne && self.state.position.y - collision.half_size.y <= -4 {
            let mut reset_hitstun = true;
            let mut reset_velocity = true;
            self.state.current_state = if state.state_type == MoveType::Hitstun {
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
        if (move_type.is_movement() && facing_opponent) || move_type == MoveType::Fly {
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
        let flags = state.flags.at_time(frame);
        let hitboxes = state.hitboxes.at_time(frame);
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
    fn prune_bullets(&mut self, play_area: &PlayArea) {
        let bullet_data = &self.data;
        self.state
            .bullets
            .retain(|item| item.alive(bullet_data, play_area));
    }
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
                    } else if (state_type.is_blockstun()
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
        let flags = self.data.states[&move_id].flags.at_time(frame);
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
        global_particles: &HashMap<GlobalParticle, Particle>,
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
        self.update_particles(global_particles);
        self.update_bullets(play_area);
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

        let collision = &self.data.states[&move_id].hitboxes.at_time(frame).collision;

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
    fn draw_particles(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        world: graphics::Matrix4,
        global_particles: &HashMap<GlobalParticle, Particle>,
    ) -> GameResult<()> {
        crate::roster::impls::draw_particles(
            ctx,
            assets,
            world,
            &self.data.particles,
            global_particles,
            &self.state.particles,
        )
    }
    fn draw_bullets(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        world: graphics::Matrix4,
    ) -> GameResult<()> {
        for bullet in &self.state.bullets {
            bullet.draw(ctx, &self.data, assets, world)?;
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

        let collision = &self.data.states[&move_id].hitboxes.at_time(frame).collision;

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
    fn collision(&self) -> PositionedHitbox {
        let (frame, move_id) = &self.state.current_state;
        self.data.states[move_id]
            .hitboxes
            .at_time(*frame)
            .collision
            .with_collision_position(self.state.position)
    }
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
    fn get_attack_data(&self) -> Option<HitAction> {
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

    fn bullets_mut(&mut self) -> super::generic_character::OpaqueBulletIterator {
        super::generic_character::OpaqueBulletIterator::YuyukoIter(YuyukoBulletIterator {
            iter: self.state.bullets.iter_mut(),
            bullet_list: &self.data.bullets,
            attacks: &self.data.attacks,
        })
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
            .try_time(current_frame + 1)
            .map(|item| item.cutscene)
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
        global_particles: &HashMap<GlobalParticle, Particle>,
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
        self.update_particles(global_particles);
        self.update_bullets(play_area);
        self.state.sound_state.update();
        self.state.hitstop = i32::max(0, self.state.hitstop);
    }

    fn is_dead(&self) -> bool {
        self.state.dead
    }

    fn draw_order_priority(&self) -> i32 {
        match self.data.states[&self.state.current_state.1].state_type {
            MoveType::Blockstun | MoveType::WrongBlockstun | MoveType::Hitstun => -1,
            _ => 0,
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
            particles: Vec::new(),
            bullets: Vec::new(),
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
            particles: Vec::new(),
            bullets: Vec::new(),
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

pub struct YuyukoBulletIterator<'a> {
    iter: std::slice::IterMut<'a, BulletState>,
    bullet_list: &'a BulletList,
    attacks: &'a AttackList,
}

impl<'a> Iterator for YuyukoBulletIterator<'a> {
    type Item = super::generic_character::OpaqueBullet<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|state| {
            YuyukoBulletMut {
                state,
                list: self.bullet_list,
                attacks: self.attacks,
            }
            .into()
        })
    }
}

//impl<'a> super::generic_character::BulletIterator<'a> for YuyukoBulletIterator<'a> {}

use super::generic_character::BulletMut;

pub struct YuyukoBulletMut<'a> {
    state: &'a mut BulletState,
    list: &'a BulletList,
    attacks: &'a AttackList,
}

impl<'a> BulletMut for YuyukoBulletMut<'a> {
    fn hitboxes(&self) -> Vec<PositionedHitbox> {
        self.state.hitbox(self.list)
    }
    fn on_touch_bullet(&mut self) {
        self.state.on_touch_bullet(&self.list);
    }
    fn attack_data(&self) -> AttackInfo {
        self.state.attack_data(&self.list, &self.attacks)
    }
    fn deal_hit(&mut self, hit: &HitResult) {
        self.state.deal_hit(&self.list, hit)
    }
    fn hash(&self) -> u64 {
        self.state.hash()
    }
    fn facing(&self) -> Facing {
        self.state.facing()
    }
}
