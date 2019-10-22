mod butterfly;

use super::{AttackList, BulletList, HitInfo};
use crate::assets::Assets;
use crate::game_match::PlayArea;
use crate::hitbox::PositionedHitbox;
use crate::input::Facing;
use crate::typedefs::collision::Vec2;
use crate::typedefs::graphics;
use butterfly::{ButterflySpawn, ButterflyState};
use ggez::{Context, GameResult};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BulletId {
    Butterfly,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "bullet_id")]
pub enum BulletSpawn {
    Butterfly(ButterflySpawn),
}
#[derive(Clone, Copy, Debug)]
pub enum BulletState {
    Butterfly(ButterflyState),
}

impl BulletSpawn {
    pub fn get_spawn_frame(&self) -> usize {
        match self {
            BulletSpawn::Butterfly(spawner) => spawner.get_spawn_frame(),
        }
    }
    pub fn instantiate(&self, current_position: Vec2, facing: Facing) -> BulletState {
        match self {
            BulletSpawn::Butterfly(spawner) => {
                BulletState::Butterfly(spawner.instantiate(current_position, facing))
            }
        }
    }
}

impl BulletState {
    pub fn update(&mut self, data: &BulletList) {
        match self {
            BulletState::Butterfly(state) => state.update(data),
        }
    }
    pub fn alive(&self, data: &BulletList, area: &PlayArea) -> bool {
        match self {
            BulletState::Butterfly(state) => state.alive(data, area),
        }
    }
    pub fn hitbox(&self, data: &BulletList) -> Vec<PositionedHitbox> {
        match self {
            BulletState::Butterfly(state) => state.hitbox(data),
        }
    }
    pub fn attack_data(&self, data: &BulletList, attacks: &AttackList) -> HitInfo {
        match self {
            BulletState::Butterfly(state) => state.attack_data(data, attacks),
        }
    }
    pub fn draw(
        &self,
        ctx: &mut Context,
        data: &BulletList,
        assets: &Assets,
        world: graphics::Matrix4,
    ) -> GameResult<()> {
        match self {
            BulletState::Butterfly(state) => state.draw(ctx, data, assets, world),
        }
    }
}

impl Default for BulletId {
    fn default() -> Self {
        BulletId::Butterfly
    }
}
impl Default for BulletSpawn {
    fn default() -> Self {
        panic!("called unnecessary default for BulletSpawn");
    }
}
