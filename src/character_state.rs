pub mod animation_data;
pub mod cancel_set;
pub mod flags;
pub mod hitbox_set;
pub mod particle_spawn_data;

mod ui;

use crate::assets::Assets;
use crate::graphics::Animation;

use ggez::{Context, GameResult};
use serde::{Deserialize, Serialize};

use std::cmp;

pub use animation_data::{AnimationData, AnimationDataUi};
pub use cancel_set::{CancelSet, CancelSetUi, MoveType};
pub use flags::{Flags, FlagsUi, MovementData};
pub use hitbox_set::{HitboxSet, HitboxSetUi};
pub use particle_spawn_data::{ParticleSpawn, ParticleSpawnUi};

pub use ui::CharacterStateUi;

use crate::timeline::{AtTime, Timeline};

use crate::typedefs::graphics::Matrix4;

use crate::typedefs::{HashId, StateId};

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use ggez::GameError;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CharacterState<Id>
where
    Id: HashId,
{
    pub animations: Vec<AnimationData>,
    pub flags: Timeline<Flags>,
    pub cancels: Timeline<CancelSet<Id>>,
    pub hitboxes: Timeline<HitboxSet>,
    #[serde(default)]
    pub particles: Vec<ParticleSpawn>,
    #[serde(default = "default_move_type")]
    pub state_type: MoveType,
    #[serde(default)]
    pub on_expire_state: Id,
}
fn default_move_type() -> MoveType {
    MoveType::Idle
}

impl<Id: StateId> CharacterState<Id> {
    pub fn load_from_json(
        ctx: &mut Context,
        assets: &mut Assets,
        mut path: PathBuf,
    ) -> GameResult<Self> {
        let file = File::open(&path).unwrap();
        let buf_read = BufReader::new(file);
        let state = serde_json::from_reader::<_, Self>(buf_read).unwrap();
        let name = path.file_stem().unwrap().to_str().unwrap().to_owned();
        path.pop();
        CharacterState::load(ctx, assets, &state, &name, path)?;
        Ok(state)
    }
    pub fn load(
        ctx: &mut Context,
        assets: &mut Assets,
        state: &Self,
        name: &str,
        mut path: PathBuf,
    ) -> GameResult<()> {
        path.push(name);
        for animation in &state.animations {
            Animation::load(ctx, assets, &animation.animation, path.clone())?;
        }
        Ok(())
    }
    pub fn save(
        ctx: &mut Context,
        assets: &mut Assets,
        state: &Self,
        mut path: PathBuf,
    ) -> GameResult<()> {
        let name = path.file_stem().unwrap().to_str().unwrap().to_owned();

        let mut json = File::create(&path)?;
        serde_json::to_writer(&mut json, &state)
            .map_err(|err| GameError::FilesystemError(format!("{}", err)))?;

        path.pop();
        path.push(&name);
        std::fs::create_dir_all(&path)?;
        for animation in &state.animations {
            path.push(&format!("{}.json", &animation.animation.name));
            Animation::save(ctx, assets, &animation.animation, path.clone())?;
            path.pop();
        }
        Ok(())
    }

    pub fn duration(&self) -> usize {
        self.animations
            .iter()
            .map(|item| item.delay + item.duration())
            .fold(0, cmp::max)
    }

    pub fn fix_duration(&mut self) {
        if self.duration() > 0 {
            self.flags.fix_duration(self.duration());
            self.cancels.fix_duration(self.duration());
            self.hitboxes.fix_duration(self.duration());
        }
    }

    pub fn draw_at_time(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        time: usize,
        world: Matrix4,
    ) -> GameResult<()> {
        if time < self.duration() {
            for animation in self.animations.iter() {
                animation.draw_at_time(ctx, assets, time, world)?
            }
        }
        Ok(())
    }
    pub fn draw_at_time_debug(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        time: usize,
        world: Matrix4,
    ) -> GameResult<()> {
        if time < self.duration() {
            for animation in self.animations.iter() {
                animation.draw_at_time_debug(ctx, assets, time, world)?
            }
        }
        Ok(())
    }
}

impl CharacterState<String> {
    pub fn new() -> Self {
        Self {
            animations: vec![],
            flags: vec![(Flags::new(), 1)],
            cancels: vec![(CancelSet::new(), 1)],
            hitboxes: vec![(HitboxSet::new(), 1)],
            state_type: default_move_type(),
            on_expire_state: "stand".to_owned(),
            particles: Vec::new(),
        }
    }
}
