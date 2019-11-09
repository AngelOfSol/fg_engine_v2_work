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
use crate::hitbox::Hitbox;
use crate::hitbox::PositionedHitbox;
use crate::input::{read_inputs, Button, DirectedAxis, Facing, InputBuffer};
use crate::roster::generic_character::bullet::{GenericBulletSpawn, GenericBulletState};
use crate::roster::generic_character::combo_state::{AllowedCancel, ComboState};
use crate::roster::generic_character::extra_data::ExtraData;
use crate::roster::generic_character::hit_info::{HitInfo, HitType};
use crate::roster::generic_character::move_id::GenericMoveId;
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
use serde::Deserialize;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::BufReader;
use std::path::PathBuf;
use std::rc::Rc;

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
#[derive(Clone, Debug)]
pub struct Yuyuko {
    pub assets: Assets,
    pub states: HashMap<MoveId, State<MoveId, Particle, BulletSpawn, AttackId>>,
    pub particles: HashMap<Particle, Animation>,
    pub bullets: BulletList,
    pub attacks: HashMap<AttackId, AttackInfo>,
    pub properties: Properties,
    pub command_list: CommandList<MoveId>,
}

type StateList = HashMap<MoveId, State<MoveId, Particle, BulletSpawn, AttackId>>;
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
pub struct YuyukoState {
    pub data: Rc<Yuyuko>,
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
    pub last_hit_using: Option<u64>,
    pub current_combo: Option<ComboState>,
    pub health: i32,
    pub allowed_cancels: AllowedCancel,
    pub rebeat_chain: HashSet<MoveId>,
    pub should_pushback: bool,
    pub crushed_orbs: i32,
    pub uncrush_timer: i32,
}

impl GenericCharacterBehaviour for YuyukoState {
    type MoveId = MoveId;
    type Particle = Particle;
    type ResourceData = Yuyuko;
    type Properties = Properties;

    fn new(data: Rc<Yuyuko>) -> Self {
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
            data,
        }
    }

    impl_in_corner!();
    impl_apply_pushback!();
    impl_get_pushback!();
    impl_collision!();

    impl_hitboxes!();
    impl_hurtboxes!();

    impl_get_attack_data!();
    impl_prune_bullets!();
    impl_current_flags!();
    impl_would_be_hit!();

    impl_guard_crush!(
        hitstun_air: MoveId::HitstunAirStart,
        hitstun_ground: MoveId::HitstunStandStart
    );

    impl_crush_orb!();
    // TODO FIX THESE VALUES
    // THEN SWITCH FROM EXPR TO PATTERN WHERE PERTINENT
    impl_take_hit!(
        hitstun_air: MoveId::HitstunAirStart,
        hitstun_ground: MoveId::HitstunStandStart,
        blockstun_air: MoveId::BlockstunAirStart,
        blockstun_stand: MoveId::BlockstunStandStart,
        blockstun_crouch: MoveId::BlockstunCrouchStart,
        wrongblock_stand: MoveId::WrongblockStandStart,
        wrongblock_crouch: MoveId::WrongblockCrouchStart
    );
    impl_deal_hit!(on_hit_particle: Particle::HitEffect);

    impl_handle_fly!(fly_start: MoveId::FlyStart);

    impl_handle_jump!(
        jump: MoveId::Jump,
        super_jump: MoveId::SuperJump,
        border_escape: MoveId::BorderEscapeJump
    );
    impl_handle_combo_state!();
    impl_handle_rebeat_data!();

    impl_update_combo_state!();
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
    impl_on_enter_move!(
        fly_start: MoveId::FlyStart,
        jump: MoveId::Jump,
        super_jump: MoveId::SuperJump,
        border_escape: MoveId::BorderEscapeJump,
        melee_restitution: MoveId::MeleeRestitution
    );
    impl_update_velocity!();
    impl_update_position!(
        knockdown_start: MoveId::HitGround,
        hitstun_air: MoveId::HitstunAirStart,
        stand_idle: MoveId::Stand
    );
    impl_update_particles!();
    impl_spawn_particle!();

    impl_update_bullets!();
    impl_update_spirit!();
    impl_clamp_spirit!();
    impl_handle_refacing!();
    impl_update_frame_mut!();

    impl_draw_ui!();
    impl_draw!();
    impl_draw_particles!();
    impl_draw_bullets!();
    impl_draw_shadow!();
}
