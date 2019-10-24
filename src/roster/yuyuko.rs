mod attacks;
mod bullets;
mod command_list;
mod moves;
mod particles;

use crate::assets::Assets;
use crate::character::components::{AttackInfo, GroundAction};
use crate::character::state::components::{Flags, MoveType};
use crate::character::state::State;
use crate::command_list::CommandList;
use crate::game_match::PlayArea;
use crate::graphics::Animation;
use crate::hitbox::{Hitbox, PositionedHitbox};
use crate::input::{read_inputs, Button, DirectedAxis, Facing, InputBuffer};
use crate::timeline::AtTime;
use crate::typedefs::collision::IntoGraphical;
use crate::typedefs::{collision, graphics};
use attacks::AttackId;
use bullets::BulletSpawn;
pub use bullets::BulletState;
use ggez::{Context, GameResult};
use moves::MoveId;
use particles::Particle;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

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

#[derive(Clone, Debug)]
pub struct Yuyuko {
    pub assets: Assets,
    pub states: StateList,
    pub particles: ParticleList,
    pub bullets: BulletList,
    pub attacks: AttackList,
    pub properties: Properties,
    pub command_list: CommandList<MoveId>,
}

type StateList = HashMap<MoveId, State<MoveId, Particle, BulletSpawn, AttackId>>;
type ParticleList = HashMap<Particle, Animation>;
pub type AttackList = HashMap<AttackId, AttackInfo>;

#[derive(Debug, Clone)]
pub enum HitInfo {
    Character {
        info: AttackInfo,
        move_id: MoveId,
        hitbox_id: usize,
        facing: Facing,
    },
    Bullet(AttackInfo, Facing),
}
impl HitInfo {
    pub fn get_attack_data(&self) -> &AttackInfo {
        match self {
            HitInfo::Character { ref info, .. } => info,
            HitInfo::Bullet(ref info, _) => info,
        }
    }

    pub fn get_facing(&self) -> Facing {
        match self {
            HitInfo::Character { facing, .. } => *facing,
            HitInfo::Bullet(_, facing) => *facing,
        }
    }

    pub fn should_pushback(&self) -> bool {
        match self {
            HitInfo::Character { .. } => true,
            HitInfo::Bullet(_, _) => false,
        }
    }

    pub fn get_hit_by_data(&self) -> Option<(MoveId, usize)> {
        if let HitInfo::Character {
            move_id, hitbox_id, ..
        } = self
        {
            Some((*move_id, *hitbox_id))
        } else {
            None
        }
    }
}

impl Yuyuko {
    pub fn new_with_path(ctx: &mut Context, path: PathBuf) -> GameResult<Yuyuko> {
        let mut assets = Assets::new();
        let data = YuyukoData::load_from_json(ctx, &mut assets, path)?;
        Ok(Yuyuko {
            assets,
            states: data.states,
            particles: data.particles,
            properties: data.properties,
            attacks: data.attacks,
            bullets: data.bullets,
            command_list: command_list::generate_command_list(),
        })
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct YuyukoData {
    states: StateList,
    particles: ParticleList,
    bullets: BulletList,
    properties: Properties,
    attacks: AttackList,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Properties {
    health: i32,
    name: String,
    neutral_jump_accel: collision::Vec2,
    neutral_super_jump_accel: collision::Vec2,
    directed_jump_accel: collision::Vec2,
    directed_super_jump_accel: collision::Vec2,
    max_air_actions: usize,
    max_spirit_gauge: i32,
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
        for (name, state) in character.states.iter_mut() {
            State::load(ctx, assets, state, &name.to_string(), path.clone())?;
        }
        path.push("particles");
        for (_name, particle) in character.particles.iter_mut() {
            Animation::load(ctx, assets, particle, path.clone())?;
        }
        path.pop();
        path.push("bullets");
        Animation::load(
            ctx,
            assets,
            &mut character.bullets.butterfly.animation,
            path.clone(),
        )?;

        Ok(character)
    }
}
#[derive(Debug, Clone)]
pub enum HitType {
    Whiff,
    Block(HitInfo),
    WrongBlock(HitInfo),
    Hit(HitInfo),
    Graze(HitInfo),
}

#[derive(Debug, Clone, Copy)]
enum ExtraData {
    JumpDirection(DirectedAxis),
    FlyDirection(DirectedAxis),
    Stun(i32),
    None,
}

impl ExtraData {
    fn unwrap_jump_direction(self) -> DirectedAxis {
        match self {
            ExtraData::JumpDirection(dir) => dir,
            value => panic!("Expected JumpDirection, found {:?}.", value),
        }
    }
    fn unwrap_fly_direction(self) -> DirectedAxis {
        match self {
            ExtraData::FlyDirection(dir) => dir,
            value => panic!("Expected FlyDirection, found {:?}.", value),
        }
    }
    fn unwrap_stun_mut(&mut self) -> &mut i32 {
        match self {
            ExtraData::Stun(ref mut time) => time,
            value => panic!("Expected HitStun, found {:?}.", value),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ComboState {
    hits: u32,
    total_damage: i32,
    last_hit_damage: i32,
    proration: i32,
    ground_action: GroundAction,
    available_limit: i32,
}

#[derive(Debug, Clone)]
pub enum AllowedCancel {
    Always,
    Hit,
    Block,
}

#[derive(Debug, Clone)]
pub struct YuyukoState {
    pub velocity: collision::Vec2,
    pub position: collision::Vec2,
    pub current_state: (usize, MoveId),
    extra_data: ExtraData,
    pub particles: Vec<(usize, collision::Vec2, Particle)>,
    pub bullets: Vec<BulletState>,
    pub facing: Facing,
    pub air_actions: usize,
    pub spirit_gauge: i32,
    pub spirit_delay: i32,
    pub hitstop: i32,
    pub last_hit_using: Option<(MoveId, usize)>,
    pub current_combo: Option<ComboState>,
    pub health: i32,
    pub allowed_cancels: AllowedCancel,
    pub rebeat_chain: HashSet<MoveId>,
    pub should_pushback: bool,
}

impl YuyukoState {
    pub fn new(data: &Yuyuko) -> Self {
        Self {
            velocity: collision::Vec2::zeros(),
            position: collision::Vec2::zeros(),
            current_state: (0, MoveId::Stand),
            extra_data: ExtraData::None,
            particles: Vec::new(),
            bullets: Vec::new(),
            air_actions: data.properties.max_air_actions,
            spirit_gauge: data.properties.max_spirit_gauge,
            spirit_delay: 0,
            hitstop: 0,
            facing: Facing::Right,
            last_hit_using: None,
            health: data.properties.health,
            current_combo: None,
            allowed_cancels: AllowedCancel::Always,
            rebeat_chain: HashSet::new(),
            should_pushback: true,
        }
    }

    pub fn collision(&self, data: &Yuyuko) -> PositionedHitbox {
        let (frame, move_id) = &self.current_state;
        data.states[move_id]
            .hitboxes
            .at_time(*frame)
            .collision
            .with_position(self.position)
    }
    pub fn in_corner(&self, data: &Yuyuko, play_area: &PlayArea) -> bool {
        let collision = self.collision(data);
        i32::abs(self.position.x) >= play_area.width / 2 - collision.half_size.x
    }

    pub fn apply_pushback(&mut self, data: &Yuyuko, force: collision::Int) {
        let flags = self.current_flags(data);
        if !flags.airborne {
            self.position.x += force;
        }
    }
    pub fn get_pushback(&self, data: &Yuyuko, play_area: &PlayArea) -> collision::Int {
        let (frame, move_id) = &self.current_state;
        let state = &data.states[&move_id];
        let flags = state.flags.at_time(*frame);

        if !flags.airborne
            && (state.state_type == MoveType::Hitstun || state.state_type == MoveType::Blockstun)
            && self.in_corner(data, play_area)
            && self.hitstop == 0
            && self.should_pushback
        {
            -self.velocity.x
        } else {
            0
        }
    }

    pub fn hitboxes(&self, data: &Yuyuko) -> Vec<PositionedHitbox> {
        let (frame, move_id) = &self.current_state;
        data.states[move_id]
            .hitboxes
            .at_time(*frame)
            .hitbox
            .iter()
            .map(|data| {
                data.boxes
                    .iter()
                    .map(|item| item.with_position_and_facing(self.position, self.facing))
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect()
    }
    pub fn hurtboxes(&self, data: &Yuyuko) -> Vec<PositionedHitbox> {
        let (frame, move_id) = &self.current_state;
        data.states[move_id]
            .hitboxes
            .at_time(*frame)
            .hurtbox
            .iter()
            .map(|item| item.with_position_and_facing(self.position, self.facing))
            .collect()
    }
    pub fn get_attack_data(&self, data: &Yuyuko) -> Option<HitInfo> {
        let (frame, move_id) = &self.current_state;

        data.states[move_id]
            .hitboxes
            .at_time(*frame)
            .hitbox
            .as_ref()
            .and_then(|item| {
                if let Some((move_id, hitbox_id)) = self.last_hit_using {
                    if move_id == self.current_state.1 && hitbox_id == item.id {
                        return None;
                    }
                }
                Some(item)
            })
            .map(|item| HitInfo::Character {
                facing: self.facing,
                info: data.attacks[&item.data_id].clone(),
                move_id: self.current_state.1,
                hitbox_id: item.id,
            })
    }

    pub fn prune_bullets(&mut self, data: &Yuyuko, play_area: &PlayArea) {
        // TODO, add on death effects here

        self.bullets
            .retain(|item| item.alive(&data.bullets, play_area));
    }

    pub fn current_flags<'a>(&self, data: &'a Yuyuko) -> &'a Flags {
        let (frame, move_id) = self.current_state;
        data.states[&move_id].flags.at_time(frame)
    }

    pub fn would_be_hit(
        &self,
        data: &Yuyuko,
        input: &InputBuffer,
        touched: bool,
        total_info: Option<HitInfo>,
    ) -> HitType {
        if !touched
            || total_info.is_none()
            || self
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

        let flags = self.current_flags(data);
        let state_type = data.states[&self.current_state.1].state_type;
        let axis = DirectedAxis::from_facing(input.top().axis, self.facing);

        if !info.melee && flags.bullet.is_invuln() || info.melee && flags.melee.is_invuln() {
            return HitType::Whiff;
        } else if info.grazeable && flags.grazing {
            return HitType::Graze(total_info);
        } else if info.air_unblockable && flags.airborne {
            return HitType::Hit(total_info);
        }

        if state_type == MoveType::Blockstun || (flags.can_block && axis.is_backward()) {
            if flags.airborne || axis.is_blocking(info.guard) {
                HitType::Block(total_info)
            } else {
                HitType::WrongBlock(total_info)
            }
        } else {
            HitType::Hit(total_info)
        }
    }
    pub fn guard_crush(&mut self, data: &Yuyuko, info: &HitInfo) {
        if self.spirit_gauge <= 0 {
            let attack_data = info.get_attack_data();
            let flags = self.current_flags(data);
            let hit_direction = info.get_facing();
            let on_hit = &attack_data.on_hit;
            // guard crush time!!!!!!!!!!
            self.spirit_gauge = data.properties.max_spirit_gauge;
            if flags.airborne {
                self.current_state = (0, MoveId::HitstunAirStart);
                //TODO crush velocity mutliplier
                self.velocity = hit_direction.fix_collision(on_hit.air_force) * 3;
            } else {
                self.current_state = (0, MoveId::HitstunStandStart);
            }
            self.extra_data = ExtraData::Stun(attack_data.level.crush_stun());
            self.update_combo_state(&attack_data, true);
        }
    }

    pub fn take_hit(&mut self, data: &Yuyuko, info: &HitType) {
        let flags = self.current_flags(data);

        match info {
            HitType::Hit(info) => {
                let hit_direction = info.get_facing();
                let attack_data = info.get_attack_data();

                let on_hit = &attack_data.on_hit;
                if flags.airborne || attack_data.launcher {
                    self.current_state = (0, MoveId::HitstunAirStart);
                    self.velocity = hit_direction.fix_collision(on_hit.air_force);
                } else {
                    self.current_state = (0, MoveId::HitstunStandStart);
                    self.velocity = hit_direction
                        .fix_collision(collision::Vec2::new(on_hit.ground_pushback, 0_00));
                }
                self.extra_data = ExtraData::Stun(attack_data.level.hitstun());
                self.hitstop = on_hit.defender_stop;
                self.should_pushback = info.should_pushback();

                self.update_combo_state(&attack_data, false);
                let current_combo = self.current_combo.as_ref().unwrap();
                self.health -= current_combo.last_hit_damage;
            }
            HitType::Block(info) => {
                let hit_direction = info.get_facing();
                let attack_data = info.get_attack_data();

                let on_block = &attack_data.on_block;
                if flags.airborne {
                    self.current_state = (0, MoveId::BlockstunAirStart);
                    self.velocity = hit_direction.fix_collision(on_block.air_force);
                } else {
                    self.current_state = (
                        0,
                        if flags.crouching {
                            MoveId::BlockstunCrouchStart
                        } else {
                            MoveId::BlockstunStandStart
                        },
                    );
                    self.velocity = hit_direction
                        .fix_collision(collision::Vec2::new(on_block.ground_pushback, 0_00));
                }

                self.spirit_gauge -= attack_data.spirit_cost;
                self.spirit_gauge = i32::max(0, self.spirit_gauge);
                if attack_data.reset_spirit_delay {
                    self.spirit_delay = 0;
                }
                self.spirit_delay += attack_data.spirit_delay;

                self.extra_data = ExtraData::Stun(attack_data.level.blockstun());
                self.hitstop = on_block.defender_stop;
                self.should_pushback = info.should_pushback();
                self.health -= attack_data.chip_damage;

                if self.spirit_gauge <= 0 {
                    self.guard_crush(data, info);
                }
            }
            HitType::WrongBlock(info) => {
                let hit_direction = info.get_facing();
                let attack_data = info.get_attack_data();

                let on_block = &attack_data.on_block;
                self.current_state = (
                    0,
                    if flags.crouching {
                        MoveId::WrongblockCrouchStart
                    } else {
                        MoveId::WrongblockStandStart
                    },
                );
                self.velocity = hit_direction
                    .fix_collision(collision::Vec2::new(on_block.ground_pushback, 0_00));

                self.spirit_delay = attack_data.level.wrongblock_delay();
                self.spirit_gauge -= attack_data.level.wrongblock_cost();
                self.spirit_gauge = i32::max(0, self.spirit_gauge);

                self.extra_data = ExtraData::Stun(attack_data.level.wrongblockstun());
                self.hitstop = on_block.defender_stop;
                self.should_pushback = info.should_pushback();
                self.health -= attack_data.chip_damage;

                if self.spirit_gauge <= 0 {
                    self.guard_crush(data, info);
                }
            }
            HitType::Whiff | HitType::Graze(_) => {}
        }
    }
    pub fn deal_hit(&mut self, data: &Yuyuko, info: &HitType) {
        let boxes = self.hitboxes(data);

        match info {
            HitType::Hit(info) => {
                if let Some(last_hit) = info.get_hit_by_data() {
                    self.last_hit_using = Some(last_hit);
                }
                let info = info.get_attack_data();
                let on_hit = &info.on_hit;

                self.hitstop = on_hit.attacker_stop;
                self.allowed_cancels = AllowedCancel::Hit;

                if !boxes.is_empty() {
                    // TODO improve hit effect particle spawning determination
                    let spawn_point = boxes
                        .iter()
                        .fold(collision::Vec2::zeros(), |acc, item| acc + item.center)
                        / boxes.len() as i32;
                    self.spawn_particle(Particle::HitEffect, spawn_point);
                }
            }
            HitType::Block(info) | HitType::WrongBlock(info) => {
                if let Some(last_hit) = info.get_hit_by_data() {
                    self.last_hit_using = Some(last_hit);
                }
                let info = info.get_attack_data();
                let on_block = &info.on_block;

                self.allowed_cancels = AllowedCancel::Block;
                self.hitstop = on_block.attacker_stop;
            }
            HitType::Whiff | HitType::Graze(_) => {}
        }
    }

    fn handle_fly(move_id: MoveId, extra_data: &mut ExtraData) -> collision::Vec2 {
        if move_id == MoveId::FlyStart {
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
                    if axis == DirectedAxis::Up {
                        data.neutral_jump_accel
                    } else {
                        data.directed_jump_accel
                            .component_mul(&collision::Vec2::new(
                                axis.direction_multiplier(true),
                                1,
                            ))
                    }
                }
                MoveId::SuperJump => {
                    if axis == DirectedAxis::Up {
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

    fn handle_combo_state(&mut self, data: &Yuyuko) {
        let (_, move_id) = self.current_state;
        let current_state_type = data.states[&move_id].state_type;
        if current_state_type != MoveType::Hitstun && current_state_type != MoveType::Blockstun {
            self.current_combo = None;
        }
    }
    fn handle_rebeat_data(&mut self, data: &Yuyuko) {
        let (_, move_id) = self.current_state;

        if !data.states[&move_id].state_type.is_attack() {
            self.rebeat_chain.clear();
        }
    }

    fn update_combo_state(&mut self, info: &AttackInfo, guard_crush: bool) {
        self.current_combo = Some(match &self.current_combo {
            Some(state) => {
                let proration = info.proration * state.proration / 100;
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
                    available_limit: info.starter_limit,
                }
            }
        });
    }

    fn handle_expire(&mut self, data: &Yuyuko) {
        let (frame, move_id) = self.current_state;

        // if the next frame would be out of bounds
        self.current_state = if frame >= data.states[&move_id].duration() - 1 {
            self.allowed_cancels = AllowedCancel::Always;
            self.last_hit_using = None;
            self.rebeat_chain.clear();
            (0, data.states[&move_id].on_expire_state)
        } else {
            (frame + 1, move_id)
        };
    }

    fn handle_hitstun(&mut self, data: &Yuyuko) {
        let (frame, move_id) = self.current_state;
        let flags = data.states[&move_id].flags.at_time(frame);
        let state_type = data.states[&move_id].state_type;

        if state_type == MoveType::Hitstun || state_type == MoveType::Blockstun {
            let hitstun = self.extra_data.unwrap_stun_mut();
            *hitstun -= 1;
            if *hitstun == 0 {
                if !flags.airborne {
                    self.current_state = (
                        0,
                        if flags.crouching {
                            MoveId::Crouch
                        } else {
                            MoveId::Stand
                        },
                    );
                } else {
                    self.current_state = if state_type == MoveType::Blockstun {
                        (0, MoveId::AirIdle)
                    } else {
                        (frame, move_id)
                    };
                }
            }
        }
    }

    fn handle_input(&mut self, data: &Yuyuko, input: &InputBuffer) {
        let (frame, move_id) = self.current_state;
        let cancels = data.states[&move_id].cancels.at_time(frame);

        self.current_state = {
            let inputs = read_inputs(&input, self.facing);
            if move_id == MoveId::Fly {
                if input.top()[Button::A].is_pressed() && input.top()[Button::B].is_pressed() {
                    (frame, move_id)
                } else {
                    (0, MoveId::FlyEnd)
                }
            } else {
                let possible_new_move = data.command_list
                    .get_commands(&inputs)
                    .into_iter()
                    .copied()
                    .filter(|new_move_id| {
                        *new_move_id != move_id
                            && (cancels
                                .always
                                .contains(&data.states[new_move_id].state_type)
                                || match self.allowed_cancels {
                                    AllowedCancel::Hit => cancels.hit.contains(&data.states[new_move_id].state_type),
                                    AllowedCancel::Block => cancels.block.contains(&data.states[new_move_id].state_type),
                                    AllowedCancel::Always => false,
                                })
                            && !self.rebeat_chain.contains(new_move_id)
                            && !cancels.disallow.contains(new_move_id)
                            // not ideal way to handle disallowing fly, consider separating out from cancel checking
                            && !(*new_move_id == MoveId::FlyStart && self.air_actions == 0)
                            && self.spirit_gauge >= data.states[&new_move_id].minimum_spirit_required
                    })
                    .fold(None, |acc, item| acc.or(Some(item)))
                    .map(|new_move| (0, new_move));

                if let Some((_, new_move)) = &possible_new_move {
                    self.allowed_cancels = AllowedCancel::Always;
                    self.last_hit_using = None;
                    self.rebeat_chain.insert(*new_move);
                }

                possible_new_move.unwrap_or((frame, move_id))
            }
        };
    }

    fn update_extra_data(&mut self, input: &InputBuffer) {
        let (frame, move_id) = self.current_state;
        if frame == 0 {
            if move_id == MoveId::Jump || move_id == MoveId::SuperJump {
                self.extra_data = ExtraData::JumpDirection(DirectedAxis::from_facing(
                    input.top().axis,
                    self.facing,
                ));
            } else if move_id == MoveId::FlyStart {
                if frame == 0 {
                    self.air_actions -= 1;
                }
                let mut dir = DirectedAxis::from_facing(input.top().axis, self.facing);
                if dir.is_backward() {
                    self.facing = self.facing.invert();
                    dir = dir.invert();
                }
                self.extra_data = ExtraData::FlyDirection(if dir == DirectedAxis::Neutral {
                    DirectedAxis::Forward
                } else {
                    dir
                });
            }
        }
    }

    fn update_velocity(&mut self, data: &Yuyuko) {
        let (frame, move_id) = self.current_state;
        let flags = data.states[&move_id].flags.at_time(frame);

        let base_velocity = if flags.reset_velocity {
            collision::Vec2::zeros()
        } else {
            self.velocity
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
        let friction = if !flags.airborne {
            collision::Vec2::new(
                -i32::min(base_velocity.x.abs(), flags.friction) * i32::signum(base_velocity.x),
                0_00,
            )
        } else {
            collision::Vec2::zeros()
        };

        let accel = self.facing.fix_collision(flags.accel)
            + self
                .facing
                .fix_collision(YuyukoState::handle_fly(move_id, &mut self.extra_data))
            + self.facing.fix_collision(YuyukoState::handle_jump(
                flags,
                &data.properties,
                move_id,
                &mut self.extra_data,
            ));
        self.velocity = base_velocity + accel + friction + gravity;
    }

    fn update_position(&mut self, data: &Yuyuko, play_area: &PlayArea) {
        let (frame, move_id) = self.current_state;
        let state = &data.states[&move_id];
        let flags = state.flags.at_time(frame);
        let hitboxes = state.hitboxes.at_time(frame);
        let collision = &hitboxes.collision;

        self.position += self.velocity;

        // handle landing
        if flags.airborne && self.position.y - collision.half_size.y <= -4 {
            let mut reset_hitstun = true;
            let mut reset_velocity = true;
            self.current_state = if state.state_type == MoveType::Hitstun {
                match self.current_combo.as_ref().unwrap().ground_action {
                    GroundAction::Knockdown => (0, MoveId::HitGround),
                    GroundAction::GroundSlam => {
                        self.velocity.y *= -1;
                        self.current_combo.as_mut().unwrap().ground_action =
                            GroundAction::Knockdown;
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
                self.extra_data = ExtraData::None;
            }
            if reset_velocity {
                self.velocity = collision::Vec2::zeros();
            }
            self.position.y = hitboxes.collision.half_size.y;
            self.air_actions = data.properties.max_air_actions;
        }

        // handle stage sides
        if i32::abs(self.position.x) > play_area.width / 2 - collision.half_size.x {
            self.position.x =
                i32::signum(self.position.x) * (play_area.width / 2 - collision.half_size.x);
        }

        // if not airborne, make sure the character is locked to the ground properly
        if !flags.airborne {
            self.position.y = hitboxes.collision.half_size.y;
        }
    }

    fn update_particles(&mut self, data: &Yuyuko) {
        let (frame, move_id) = self.current_state;
        let state_particles = &data.states[&move_id].particles;

        for (ref mut frame, _, _) in self.particles.iter_mut() {
            *frame += 1;
        }
        self.particles
            .retain(|item| item.0 < data.particles[&item.2].frames.duration());
        for particle in state_particles.iter().filter(|item| item.frame == frame) {
            self.spawn_particle(particle.particle_id, self.position + particle.offset);
        }
    }

    fn spawn_particle(&mut self, particle: Particle, offset: collision::Vec2) {
        self.particles.push((0, offset, particle));
    }

    fn update_bullets(&mut self, data: &Yuyuko, play_area: &PlayArea) {
        // first update all active bullets
        for bullet in self.bullets.iter_mut() {
            bullet.update(&data.bullets);
        }

        self.prune_bullets(data, play_area);

        // then spawn bullets
        let (frame, move_id) = self.current_state;
        for spawn in data.states[&move_id]
            .bullets
            .iter()
            .filter(|item| item.get_spawn_frame() == frame)
        {
            self.bullets
                .push(spawn.instantiate(self.position, self.facing));
        }
    }

    fn update_spirit(&mut self, data: &Yuyuko) {
        let (ref mut frame, ref mut move_id) = &mut self.current_state;
        let move_data = &data.states[move_id];
        let flags = move_data.flags.at_time(*frame);

        if move_data.state_type == MoveType::Fly {
            self.spirit_gauge -= 10; // TODO, move this spirit cost to an editor value
            Self::clamp_spirit(&mut self.spirit_gauge, data);
            if self.spirit_gauge == 0 {
                *move_id = MoveId::FlyEnd;
                *frame = 0;
            }
        } else {
            self.spirit_gauge -= flags.spirit_cost;

            if flags.reset_spirit_delay {
                self.spirit_delay = 0;
            }
            self.spirit_delay += flags.spirit_delay;
            self.spirit_delay -= 1;
            self.spirit_delay = std::cmp::max(self.spirit_delay, 0);

            if self.spirit_delay == 0 {
                self.spirit_gauge += 5; // TODO: move this spirit regen to an editor value
            }

            Self::clamp_spirit(&mut self.spirit_gauge, data);
        }
    }
    fn clamp_spirit(spirit_gauge: &mut i32, data: &Yuyuko) {
        *spirit_gauge = std::cmp::max(
            std::cmp::min(*spirit_gauge, data.properties.max_spirit_gauge),
            0,
        );
    }

    pub fn handle_refacing(&mut self, data: &Yuyuko, other_player: collision::Int) {
        let (frame, move_id) = self.current_state;
        let flags = data.states[&move_id].flags.at_time(frame);
        if flags.allow_reface {
            self.facing = if self.position.x > other_player && self.facing == Facing::Right {
                Facing::Left
            } else if self.position.x < other_player && self.facing == Facing::Left {
                Facing::Right
            } else {
                self.facing
            }
        }
    }

    pub fn update_frame_mut(&mut self, data: &Yuyuko, input: &InputBuffer, play_area: &PlayArea) {
        if self.hitstop > 0 {
            self.hitstop -= 1;
        } else {
            self.handle_expire(data);
            self.handle_rebeat_data(data);
            self.handle_hitstun(data);
            self.handle_input(data, input);
            self.update_extra_data(input);
            self.update_velocity(data);
            self.update_position(data, play_area);
        }
        self.handle_combo_state(data);
        self.update_spirit(data);
        self.update_particles(data);
        self.update_bullets(data, play_area);
        self.hitstop = i32::max(0, self.hitstop);
    }
    pub fn draw_ui(
        &self,
        ctx: &mut Context,
        data: &Yuyuko,
        bottom_line: graphics::Matrix4,
    ) -> GameResult<()> {
        ggez::graphics::set_transform(ctx, bottom_line);
        ggez::graphics::apply_transformations(ctx)?;
        ggez::graphics::set_blend_mode(ctx, ggez::graphics::BlendMode::Alpha)?;

        let spirit_current = ggez::graphics::Rect::new(
            0.0,
            0.0,
            100.0 * self.spirit_gauge as f32 / data.properties.max_spirit_gauge as f32,
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
            hp_length * self.health as f32 / data.properties.health as f32,
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

    pub fn draw(
        &self,
        ctx: &mut Context,
        data: &Yuyuko,
        world: graphics::Matrix4,
    ) -> GameResult<()> {
        let (frame, move_id) = self.current_state;

        let collision = &data.states[&move_id].hitboxes.at_time(frame).collision;
        let position = world
            * graphics::Matrix4::new_translation(&graphics::up_dimension(
                self.position.into_graphical(),
            ));

        data.states[&move_id].draw_at_time(
            ctx,
            &data.assets,
            frame,
            position
                * graphics::Matrix4::new_translation(&graphics::up_dimension(
                    self.facing.fix_graphics(-collision.center.into_graphical()),
                ))
                * graphics::Matrix4::new_nonuniform_scaling(&graphics::up_dimension(
                    self.facing.graphics_multiplier(),
                )),
        )?;

        Ok(())
    }
    pub fn draw_particles(
        &self,
        ctx: &mut Context,
        data: &Yuyuko,
        world: graphics::Matrix4,
    ) -> GameResult<()> {
        for (frame, position, id) in &self.particles {
            data.particles[&id].draw_at_time(
                ctx,
                &data.assets,
                *frame,
                world
                    * graphics::Matrix4::new_translation(&graphics::up_dimension(
                        position.into_graphical(),
                    )),
            )?;
        }

        Ok(())
    }

    pub fn draw_bullets(
        &self,
        ctx: &mut Context,
        data: &Yuyuko,
        world: graphics::Matrix4,
    ) -> GameResult<()> {
        for bullet in &self.bullets {
            bullet.draw(ctx, &data.bullets, &data.assets, world)?;
        }

        Ok(())
    }
    pub fn draw_shadow(
        &self,
        ctx: &mut Context,
        data: &Yuyuko,
        world: graphics::Matrix4,
    ) -> GameResult<()> {
        let (frame, move_id) = self.current_state;

        let collision = &data.states[&move_id].hitboxes.at_time(frame).collision;
        let position = world
            * graphics::Matrix4::new_translation(&graphics::up_dimension(
                self.position.into_graphical(),
            ));

        data.states[&move_id].draw_shadow_at_time(
            ctx,
            &data.assets,
            frame,
            position
                * graphics::Matrix4::new_translation(&graphics::up_dimension(
                    self.facing.fix_graphics(-collision.center.into_graphical()),
                ))
                * graphics::Matrix4::new_nonuniform_scaling(&graphics::up_dimension(
                    self.facing.graphics_multiplier(),
                )),
        )?;
        Ok(())
    }
}
