mod attacks;
mod bullets;
mod command_list;
mod moves;
mod particles;

use crate::assets::Assets;
use crate::character::components::AttackInfo;
use crate::character::state::State;
use crate::graphics::Animation;
use crate::hitbox::Hitbox;
use crate::roster::generic_character::{hit_info, GenericCharacterState, Properties, ResourceData};
use attacks::AttackId;
use bullets::BulletSpawn;
pub use bullets::BulletState;
use ggez::{Context, GameResult};
use moves::MoveId;
use particles::Particle;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use std::path::PathBuf;

pub type HitInfo = hit_info::HitInfo<MoveId>;
pub type HitType = hit_info::HitType<MoveId>;

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

pub type Yuyuko = ResourceData<MoveId, AttackId, BulletList, BulletSpawn, Particle>;

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

pub type YuyukoState =
    GenericCharacterState<MoveId, AttackId, BulletList, BulletState, BulletSpawn, Particle>;
