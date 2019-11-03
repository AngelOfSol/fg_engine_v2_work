mod butterfly;

use super::{AttackList, BulletList};
use crate::assets::Assets;
use crate::game_match::PlayArea;
use crate::hitbox::PositionedHitbox;
use crate::input::Facing;
use crate::roster::generic_character::bullet::{GenericBulletSpawn, GenericBulletState};
use crate::roster::generic_character::hit_info::{HitInfo, HitType};
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

impl GenericBulletSpawn for BulletSpawn {
    type Output = BulletState;
    fn get_spawn_frame(&self) -> usize {
        match self {
            BulletSpawn::Butterfly(spawner) => spawner.get_spawn_frame(),
        }
    }
    fn instantiate(&self, current_position: Vec2, facing: Facing) -> BulletState {
        match self {
            BulletSpawn::Butterfly(spawner) => {
                BulletState::Butterfly(spawner.instantiate(current_position, facing))
            }
        }
    }
}

impl GenericBulletState for BulletState {
    type Resource = BulletList;
    fn update(&mut self, data: &BulletList) {
        match self {
            BulletState::Butterfly(state) => state.update(data),
        }
    }
    fn alive(&self, data: &BulletList, area: &PlayArea) -> bool {
        match self {
            BulletState::Butterfly(state) => state.alive(data, area),
        }
    }
    fn draw(
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
impl BulletState {
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

    pub fn on_touch(&mut self, bullets: &BulletList, hit_type: &HitType) {
        match self {
            BulletState::Butterfly(state) => state.on_touch(bullets, hit_type),
        }
    }
    // REMOVE when actually passing damage in
    #[allow(clippy::unit_arg)]
    pub fn on_touch_bullet(&mut self, bullets: &BulletList, damage: ()) {
        match self {
            BulletState::Butterfly(state) => state.on_touch_bullet(bullets, damage),
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
