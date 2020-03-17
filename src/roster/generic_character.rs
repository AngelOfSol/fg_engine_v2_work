pub mod bullet;
pub mod combo_state;
pub mod extra_data;
pub mod hit_info;
pub mod move_id;
pub mod particle_id;

#[macro_use]
pub mod macros;

use crate::character::components::AttackInfo;
use crate::character::state::components::Flags;
use crate::game_match::sounds::SoundList;
use crate::game_match::PlayArea;
use crate::hitbox::PositionedHitbox;
use crate::input::{Facing, InputState};
use crate::typedefs::{collision, graphics};
use enum_dispatch::enum_dispatch;
use extra_data::ExtraData;
use ggez::{Context, GameResult};
use hit_info::{HitInfo, HitType};
use rodio::Device;
use std::rc::Rc;

/* Example structures.
#[derive(Debug, Clone)]
pub struct [Character]State {
    pub data: Rc<[Character]Resources>,
    pub velocity: collision::Vec2,
    pub position: collision::Vec2,
    pub current_state: (usize, MoveId),
    extra_data: ExtraData,
    pub particles: Vec<(usize, collision::Vec2, ParticleId)>,
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
    pub crushed_orbs: i32,
    pub uncrush_timer: i32,
    marker: PhantomData<(AttackId, BulletSpawn, BulletList)>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct [Character]Properties {
    health: i32,
    name: String,
    neutral_jump_accel: collision::Vec2,
    neutral_super_jump_accel: collision::Vec2,
    directed_jump_accel: collision::Vec2,
    directed_super_jump_accel: collision::Vec2,
    max_air_actions: usize,
    max_spirit_gauge: i32,
}

#[derive(Clone, Debug)]
pub struct [Character]Resources {
    pub assets: Assets,
    pub states: HashMap<MoveId, State<MoveId, ParticleId, BulletSpawn, AttackId>>,
    pub particles: HashMap<ParticleId, Animation>,
    pub bullets: BulletList,
    pub attacks: HashMap<AttackId, AttackInfo>,
    pub properties: [Character]Properties,
    pub command_list: CommandList<MoveId>,
}*/

#[enum_dispatch]
pub trait BulletMut {
    fn hitboxes(&self) -> Vec<PositionedHitbox>;
    fn on_touch_bullet(&mut self, value: ());
    fn attack_data(&self) -> HitInfo;
    fn on_touch(&mut self, hit: &HitType);
}

#[enum_dispatch]
pub trait GenericCharacterBehaviour {
    type ParticleId;
    type MoveId;
    type SoundId;
    type Resources;
    type Properties;

    // probably unneeded.
    fn new(data: Rc<Self::Resources>) -> Self;

    fn in_corner(&self, play_area: &PlayArea) -> bool;

    fn apply_pushback(&mut self, force: collision::Int);
    fn get_pushback(&self, play_area: &PlayArea) -> collision::Int;

    fn collision(&self) -> PositionedHitbox;
    fn hitboxes(&self) -> Vec<PositionedHitbox>;
    fn hurtboxes(&self) -> Vec<PositionedHitbox>;

    fn get_attack_data(&self) -> Option<HitInfo>;

    fn prune_bullets(&mut self, play_area: &PlayArea);

    fn current_flags(&self) -> &Flags;

    fn would_be_hit(
        &self,
        input: &[InputState],
        touched: bool,
        total_info: Option<HitInfo>,
    ) -> HitType;
    fn guard_crush(&mut self, info: &HitInfo);

    fn crush_orb(&mut self);
    fn take_hit(&mut self, info: &HitType);
    fn deal_hit(&mut self, info: &HitType);

    // can probably move this to a private method rather than a trait method
    fn handle_fly(move_id: Self::MoveId, extra_data: &mut ExtraData) -> collision::Vec2;

    fn handle_jump(
        flags: &Flags,
        data: &Self::Properties,
        move_id: Self::MoveId,
        extra_data: &mut ExtraData,
    ) -> collision::Vec2;

    fn handle_combo_state(&mut self);

    fn handle_rebeat_data(&mut self);

    // TODO: change these bools into one 3 element enum
    fn update_combo_state(&mut self, info: &AttackInfo, guard_crush: bool, counter_hit: bool);

    fn handle_expire(&mut self);

    fn handle_hitstun(&mut self);

    fn handle_input(&mut self, input: &[InputState]);

    fn on_enter_move(&mut self, input: &[InputState], move_id: Self::MoveId);

    fn update_velocity(&mut self, play_area: &PlayArea);
    fn update_position(&mut self, play_area: &PlayArea);

    fn update_particles(&mut self);
    // can probably move this to a private method rather than a trait method
    fn spawn_particle(&mut self, particle: Self::ParticleId, offset: collision::Vec2);

    fn update_bullets(&mut self, play_area: &PlayArea);

    fn update_spirit(&mut self);
    fn clamp_spirit(&mut self);

    fn handle_refacing(&mut self, other_player: collision::Int);
    fn update_frame_mut(&mut self, input: &[InputState], play_area: &PlayArea);

    fn draw_ui(&self, ctx: &mut Context, bottom_line: graphics::Matrix4) -> GameResult<()>;

    fn draw(&self, ctx: &mut Context, world: graphics::Matrix4) -> GameResult<()>;
    fn draw_particles(&self, ctx: &mut Context, world: graphics::Matrix4) -> GameResult<()>;

    fn draw_bullets(&self, ctx: &mut Context, world: graphics::Matrix4) -> GameResult<()>;
    fn draw_shadow(&self, ctx: &mut Context, world: graphics::Matrix4) -> GameResult<()>;

    fn render_sound(&mut self, audio_device: &Device, sound_list: &SoundList, fps: u32) -> ();

    fn position(&self) -> collision::Vec2;
    fn position_mut(&mut self) -> &mut collision::Vec2;

    fn velocity(&self) -> collision::Vec2;
    fn facing(&self) -> Facing;
    fn bullets_mut<'a>(&'a mut self) -> OpaqueBulletIterator<'a>;

    fn save(&self) -> GameResult<Vec<u8>>;
    fn load(&mut self, value: &[u8]) -> GameResult<()>;
}

use super::yuyuko::{YuyukoBulletIterator, YuyukoBulletMut};

pub enum OpaqueBulletIterator<'a> {
    YuyukoIter(YuyukoBulletIterator<'a>),
}

impl<'a> Iterator for OpaqueBulletIterator<'a> {
    type Item = OpaqueBullet<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            OpaqueBulletIterator::YuyukoIter(iter) => iter.next(),
        }
    }
}

#[enum_dispatch(BulletMut)]
pub enum OpaqueBullet<'a> {
    YuyukoBulletMut(YuyukoBulletMut<'a>),
}
