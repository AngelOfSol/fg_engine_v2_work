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
use crate::game_match::sounds::{
    AudioBuffer, ChannelName, GlobalSound, PlayerSoundRenderer, SoundList,
};
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
#[derive(Clone)]
pub struct Yuyuko {
    pub assets: Assets,
    pub states: HashMap<MoveId, State<MoveId, Particle, BulletSpawn, AttackId>>,
    pub particles: HashMap<Particle, Animation>,
    pub bullets: BulletList,
    pub attacks: HashMap<AttackId, AttackInfo>,
    pub properties: Properties,
    pub command_list: CommandList<MoveId>,
    pub sounds: HashMap<YuyukoSound, AudioBuffer>,
}
impl std::fmt::Debug for Yuyuko {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.states)
    }
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
            //T TODO load this from data
            sounds: HashMap::new(),
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
            State::load(ctx, assets, state, &name.file_name(), path.clone())?;
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
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum YuyukoSound {}

pub struct YuyukoPlayer {
    pub data: Rc<Yuyuko>,
    pub sound_renderer: PlayerSoundRenderer<YuyukoSound>,
    pub state: YuyukoState,
}
use serde::Serialize;
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
}

impl GenericCharacterBehaviour for YuyukoPlayer {
    impl_apply_pushback!();

    impl_prune_bullets!();
    impl_would_be_hit!();

    fn would_be_hit(
        &self,
        input: &[InputState],
        info: HitAction,
        effect: Option<HitEffect>,
    ) -> (Option<HitEffect>, Option<HitResult>) {
        let attack_info = &info.attack_info;
        let flags = self.current_flags();
        let state_type = self.data.states[&self.state.current_state.1].state_type;
        let axis = DirectedAxis::from_facing(input.last().unwrap().axis, self.state.facing);
        let hit_type = effect
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
                    } else if flags.airborne || axis.is_blocking(attack_info.guard) {
                        Some(HitEffectType::Block)
                    } else {
                        Some(HitEffectType::WrongBlock)
                    })
                }
                HitEffectType::Graze => None,
            })
            .or_else(|| {
                Some(
                    if !attack_info.melee && flags.bullet.is_invuln()
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
                    } else if (state_type.is_blockstun() || (flags.can_block && axis.is_backward()))
                        && !(attack_info.air_unblockable && flags.airborne)
                    {
                        if flags.airborne || axis.is_blocking(attack_info.guard) {
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
        match hit_type {
            None => (effect, None),
            Some(HitEffectType::Graze) => match effect {
                effect @ None
                | effect
                @
                Some(HitEffect {
                    hit_type: HitEffectType::Graze,
                    ..
                }) => (
                    effect,
                    Some(HitResult {
                        hit_type: HitEffectType::Graze,
                        action: info,
                    }),
                ),
                _ => unreachable!(),
            },
            Some(HitEffectType::CounterHit) => match effect {
                None => {
                    let effect =
                        EffectData::counter_hit(&info, self.state.current_combo, flags.airborne)
                            .build();
                    (
                        Some(HitEffect {
                            hit_type: HitEffectType::CounterHit,
                            effect,
                        }),
                        Some(HitResult {
                            hit_type: HitEffectType::CounterHit,
                            action: info,
                        }),
                    )
                }
                Some(
                    effect
                    @
                    HitEffect {
                        hit_type: HitEffectType::Graze,
                        ..
                    },
                ) => {
                    let effect =
                        EffectData::counter_hit(&info, self.state.current_combo, flags.airborne)
                            .take_spirit_gauge(effect.effect.take_spirit_gauge)
                            .take_damage(effect.effect.take_damage)
                            .build();
                    (
                        Some(HitEffect {
                            hit_type: HitEffectType::CounterHit,
                            effect,
                        }),
                        Some(HitResult {
                            hit_type: HitEffectType::CounterHit,
                            action: info,
                        }),
                    )
                }
                _ => unreachable!(),
            },
            Some(HitEffectType::Block) => match effect {
                None => {
                    if self.state.spirit_gauge - attack_info.spirit_cost <= 0 {
                        let effect = EffectData::guard_crush(&info, flags.airborne).build();
                        (
                            Some(HitEffect {
                                hit_type: HitEffectType::GuardCrush,
                                effect,
                            }),
                            Some(HitResult {
                                hit_type: HitEffectType::GuardCrush,
                                action: info,
                            }),
                        )
                    } else {
                        let effect = EffectData::block(&info, flags.airborne).build();
                        (
                            Some(HitEffect {
                                hit_type: HitEffectType::Block,
                                effect,
                            }),
                            Some(HitResult {
                                hit_type: HitEffectType::Block,
                                action: info,
                            }),
                        )
                    }
                }
                Some(
                    old_effect
                    @
                    HitEffect {
                        hit_type:
                            HitEffectType::Block | HitEffectType::WrongBlock | HitEffectType::Graze,
                        ..
                    },
                ) => {
                    let effect = old_effect
                        .effect
                        .into_builder()
                        .take_spirit_gauge(info.attack_info.spirit_cost)
                        .take_damage(info.attack_info.chip_damage)
                        .build();

                    if self.state.spirit_gauge - effect.take_spirit_gauge <= 0 {
                        let effect = EffectData::guard_crush(&info, flags.airborne)
                            .take_damage(old_effect.effect.take_damage)
                            .build();
                        (
                            Some(HitEffect {
                                hit_type: HitEffectType::GuardCrush,
                                effect,
                            }),
                            Some(HitResult {
                                hit_type: HitEffectType::GuardCrush,
                                action: info,
                            }),
                        )
                    } else {
                        let hit_type = old_effect.hit_type;
                        (
                            Some(HitEffect { hit_type, effect }),
                            Some(HitResult {
                                hit_type: HitEffectType::Block,
                                action: info,
                            }),
                        )
                    }
                }
                Some(HitEffect {
                    hit_type:
                        HitEffectType::Hit
                        | HitEffectType::CounterHit
                        | HitEffectType::GuardCrush
                        | HitEffectType::GrazeCrush,
                    ..
                }) => unreachable!(),
            },
            Some(HitEffectType::WrongBlock) => match effect {
                None => {
                    // TODO write getters for attaack_info for block_cost and wrongblock_cost
                    if self.state.spirit_gauge - attack_info.level.wrongblock_cost() <= 0 {
                        let effect = EffectData::guard_crush(&info, flags.airborne).build();
                        (
                            Some(HitEffect {
                                hit_type: HitEffectType::GuardCrush,
                                effect,
                            }),
                            Some(HitResult {
                                hit_type: HitEffectType::GuardCrush,
                                action: info,
                            }),
                        )
                    } else {
                        let effect = EffectData::wrong_block(&info, flags.airborne).build();
                        (
                            Some(HitEffect {
                                hit_type: HitEffectType::WrongBlock,
                                effect,
                            }),
                            Some(HitResult {
                                hit_type: HitEffectType::WrongBlock,
                                action: info,
                            }),
                        )
                    }
                }
                Some(
                    old_effect
                    @
                    HitEffect {
                        hit_type: HitEffectType::Block | HitEffectType::Graze,
                        ..
                    },
                ) => {
                    let effect = EffectData::wrong_block(&info, flags.airborne)
                        .take_spirit_gauge(old_effect.effect.take_spirit_gauge)
                        .take_damage(old_effect.effect.take_damage)
                        .build();

                    if self.state.spirit_gauge - effect.take_spirit_gauge <= 0 {
                        let effect = EffectData::guard_crush(&info, flags.airborne)
                            .take_damage(old_effect.effect.take_damage)
                            .build();
                        (
                            Some(HitEffect {
                                hit_type: HitEffectType::GuardCrush,
                                effect,
                            }),
                            Some(HitResult {
                                hit_type: HitEffectType::GuardCrush,
                                action: info,
                            }),
                        )
                    } else {
                        (
                            Some(HitEffect {
                                hit_type: HitEffectType::WrongBlock,
                                effect,
                            }),
                            Some(HitResult {
                                hit_type: HitEffectType::WrongBlock,
                                action: info,
                            }),
                        )
                    }
                }
                Some(
                    old_effect
                    @
                    HitEffect {
                        hit_type: HitEffectType::WrongBlock,
                        ..
                    },
                ) => {
                    let effect = old_effect
                        .effect
                        .into_builder()
                        .take_spirit_gauge(info.attack_info.spirit_cost)
                        .take_damage(info.attack_info.chip_damage)
                        .build();

                    if self.state.spirit_gauge - effect.take_spirit_gauge <= 0 {
                        let effect = EffectData::guard_crush(&info, flags.airborne)
                            .take_damage(old_effect.effect.take_damage)
                            .build();
                        (
                            Some(HitEffect {
                                hit_type: HitEffectType::GuardCrush,
                                effect,
                            }),
                            Some(HitResult {
                                hit_type: HitEffectType::GuardCrush,
                                action: info,
                            }),
                        )
                    } else {
                        (
                            Some(HitEffect {
                                hit_type: HitEffectType::WrongBlock,
                                effect,
                            }),
                            Some(HitResult {
                                hit_type: HitEffectType::WrongBlock,
                                action: info,
                            }),
                        )
                    }
                }
                Some(HitEffect {
                    hit_type:
                        HitEffectType::Hit
                        | HitEffectType::CounterHit
                        | HitEffectType::GuardCrush
                        | HitEffectType::GrazeCrush,
                    ..
                }) => unreachable!(),
            },
            Some(HitEffectType::Hit) => match effect {
                None => {
                    let effect =
                        EffectData::hit(&info, self.state.current_combo, flags.airborne).build();
                    (
                        Some(HitEffect {
                            hit_type: HitEffectType::Hit,
                            effect,
                        }),
                        Some(HitResult {
                            hit_type: HitEffectType::Hit,
                            action: info,
                        }),
                    )
                }
                Some(
                    effect
                    @
                    HitEffect {
                        hit_type:
                            HitEffectType::GrazeCrush
                            | HitEffectType::GuardCrush
                            | HitEffectType::CounterHit
                            | HitEffectType::Hit,
                        ..
                    },
                ) => {
                    assert!(effect.effect.set_combo.unwrap().available_limit > 0);
                    let hit_type = effect.hit_type;
                    let effect = effect.effect.into_builder().apply_hit(&info).build();
                    (
                        Some(HitEffect { hit_type, effect }),
                        Some(HitResult {
                            hit_type: HitEffectType::Hit,
                            action: info,
                        }),
                    )
                }
                Some(
                    effect
                    @
                    HitEffect {
                        hit_type:
                            HitEffectType::Block | HitEffectType::WrongBlock | HitEffectType::Graze,
                        ..
                    },
                ) => {
                    let effect = EffectData::hit(&info, None, flags.airborne)
                        .inherit_non_hit_data(&effect.effect)
                        .build();

                    (
                        Some(HitEffect {
                            hit_type: HitEffectType::Hit,
                            effect,
                        }),
                        Some(HitResult {
                            hit_type: HitEffectType::Hit,
                            action: info,
                        }),
                    )
                }
            },
            Some(HitEffectType::GuardCrush) => unreachable!(),
            Some(HitEffectType::GrazeCrush) => unreachable!(),
        }
    }

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

    fn render_sound(&mut self, audio_device: &Device, sound_list: &SoundList, fps: u32) -> () {
        self.sound_renderer.render_frame(
            &audio_device,
            &self.data.sounds,
            &sound_list.data,
            &self.state.sound_state,
            fps,
        );
    }
    fn bullets_mut<'a>(&'a mut self) -> super::generic_character::OpaqueBulletIterator<'a> {
        super::generic_character::OpaqueBulletIterator::YuyukoIter(YuyukoBulletIterator {
            iter: self.state.bullets.iter_mut(),
            bullet_list: &self.data.bullets,
            attacks: &self.data.attacks,
        })
    }
    fn save(&self) -> GameResult<Vec<u8>> {
        bincode::serialize(&self.state).map_err(|_| {
            ggez::GameError::EventLoopError("Saving a player's state had an error.".to_owned())
        })
    }
    fn load(&mut self, value: &[u8]) -> GameResult<()> {
        self.state = bincode::deserialize(value).map_err(|_| {
            ggez::GameError::EventLoopError("Loading a player's state had an error.".to_owned())
        })?;
        Ok(())
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
