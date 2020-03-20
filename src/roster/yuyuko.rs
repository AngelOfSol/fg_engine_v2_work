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
use crate::game_match::sounds::SoundPath;
use crate::game_match::sounds::{ChannelName, GlobalSound, PlayerSoundRenderer, SoundList};
use crate::game_match::PlayArea;
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
use crate::timeline::AtTime;
use crate::typedefs::collision;
use crate::typedefs::collision::IntoGraphical;
use crate::typedefs::graphics;
use attacks::AttackId;
use bullets::BulletSpawn;
pub use bullets::BulletState;
use ggez::{Context, GameResult};
use moves::MoveId;
use particles::Particle;
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

pub struct Yuyuko {
    pub assets: Assets,
    pub states: HashMap<MoveId, State<MoveId, Particle, BulletSpawn, AttackId, YuyukoSound>>,
    pub particles: HashMap<Particle, Animation>,
    pub bullets: BulletList,
    pub attacks: HashMap<AttackId, AttackInfo>,
    pub properties: Properties,
    pub command_list: CommandList<MoveId>,
    pub sounds: SoundList<YuyukoSound>,
}
impl std::fmt::Debug for Yuyuko {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.states)
    }
}

type StateList = HashMap<MoveId, State<MoveId, Particle, BulletSpawn, AttackId, YuyukoSound>>;
type ParticleList = HashMap<Particle, Animation>;
pub type AttackList = HashMap<AttackId, AttackInfo>;

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
    pub sound_renderer: PlayerSoundRenderer<YuyukoSound>,
    pub state: YuyukoState,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YuyukoState {
    pub velocity: collision::Vec2,
    pub position: collision::Vec2,
    pub current_state: (usize, MoveId),
    pub extra_data: ExtraData,
    pub particles: Vec<(usize, collision::Vec2, Particle)>,
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
    pub sound_state: PlayerSoundState<YuyukoSound>,
}

impl YuyukoState {
    fn new(data: &Yuyuko) -> Self {
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
            crushed_orbs: 0,
            uncrush_timer: 0,
            sound_state: PlayerSoundState::new(),
        }
    }
}

use crate::game_match::sounds::PlayerSoundState;

impl YuyukoPlayer {
    pub fn new(data: Rc<Yuyuko>) -> Self {
        Self {
            state: YuyukoState::new(&data),
            data,
            sound_renderer: PlayerSoundRenderer::new(),
        }
    }
    impl_handle_fly!(fly_start: MoveId::FlyStart);
    impl_handle_jump!(
        jump: MoveId::Jump,
        super_jump: MoveId::SuperJump,
        border_escape: MoveId::BorderEscapeJump
    );
    impl_on_enter_move!(
        fly_start: MoveId::FlyStart,
        jump: MoveId::Jump,
        super_jump: MoveId::SuperJump,
        border_escape: MoveId::BorderEscapeJump,
        melee_restitution: MoveId::MeleeRestitution
    );
    impl_spawn_particle!();
    impl_in_corner!();
    impl_current_flags!();
    impl_crush_orb!();
    impl_handle_combo_state!();
    impl_handle_rebeat_data!();
    impl_handle_expire!();
    impl_handle_hitstun!(
        air_idle: MoveId::AirIdle,
        stand_idle: MoveId::Stand,
        crouch_idle: MoveId::Crouch
    );
    impl_handle_input!(
        fly_start: MoveId::FlyStart,
        fly_state: MoveId::Fly,
        fly_end: MoveId::FlyEnd,
        border_escape: MoveId::BorderEscapeJump,
        melee_restitution: MoveId::MeleeRestitution
    );
    impl_update_particles!();

    impl_update_bullets!();
    impl_update_spirit!(fly_end: MoveId::FlyEnd);
    impl_clamp_spirit!();
    impl_update_velocity!(fly_start: MoveId::FlyStart, fly_state: MoveId::Fly);
    impl_update_position!(
        knockdown_start: MoveId::HitGround,
        hitstun_air: MoveId::HitstunAirStart,
        stand_idle: MoveId::Stand
    );
    impl_validate_position!();

    impl_update_sound!();
}

impl GenericCharacterBehaviour for YuyukoPlayer {
    impl_apply_pushback!();

    impl_prune_bullets!();
    impl_would_be_hit!();

    impl_take_hit!(
        hitstun_air: MoveId::HitstunAirStart,
        hitstun_ground: MoveId::HitstunStandStart,
        blockstun_air: MoveId::BlockstunAirStart,
        blockstun_stand: MoveId::BlockstunStandStart,
        blockstun_crouch: MoveId::BlockstunCrouchStart,
        wrongblock_stand: MoveId::WrongblockStandStart,
        wrongblock_crouch: MoveId::WrongblockCrouchStart,
        guard_crush_ground: MoveId::GuardCrush,
        guard_crush_air: MoveId::HitstunAirStart
    );
    impl_deal_hit!(on_hit_particle: Particle::HitEffect);

    impl_handle_refacing!();
    impl_update_frame_mut!();

    impl_draw_ui!();
    impl_draw!();
    impl_draw_particles!();
    impl_draw_bullets!();
    impl_draw_shadow!();

    impl_get_pushback!();
    impl_collision!();

    impl_hitboxes!();
    impl_hurtboxes!();

    impl_get_attack_data!();

    impl_facing!();
    impl_velocity!();
    impl_position!();

    impl_render_sound!();

    impl_save!();
    impl_load!();

    fn bullets_mut<'a>(&'a mut self) -> super::generic_character::OpaqueBulletIterator<'a> {
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
    fn on_touch_bullet(&mut self, value: ()) {
        self.state.on_touch_bullet(&self.list, value);
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
