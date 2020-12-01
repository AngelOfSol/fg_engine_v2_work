mod butterfly;

use super::{AttackList, BulletList, Yuyuko};
use crate::assets::Assets;
use crate::game_match::PlayArea;
use crate::hitbox::PositionedHitbox;
use crate::input::Facing;
use crate::roster::generic_character::bullet::{GenericBulletSpawn, GenericBulletState};
use crate::roster::generic_character::hit_info::HitResult;
use crate::roster::AttackInfo;
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
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
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
    type Resource = Yuyuko;
    fn update(&mut self, data: &Yuyuko) {
        match self {
            BulletState::Butterfly(state) => state.update(data),
        }
    }
    fn alive(&self, data: &Yuyuko, area: &PlayArea) -> bool {
        match self {
            BulletState::Butterfly(state) => state.alive(data, area),
        }
    }
    fn draw(
        &self,
        ctx: &mut Context,
        data: &Yuyuko,
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
    pub fn attack_data(&self, data: &BulletList, attacks: &AttackList) -> AttackInfo {
        match self {
            BulletState::Butterfly(state) => state.attack_data(data, attacks),
        }
    }

    pub fn deal_hit(&mut self, bullets: &BulletList, hit_type: &HitResult) {
        match self {
            BulletState::Butterfly(state) => state.deal_hit(bullets, hit_type),
        }
    }
    pub fn hash(&self) -> u64 {
        match self {
            BulletState::Butterfly(state) => state.hash(),
        }
    }
    pub fn facing(&self) -> Facing {
        match self {
            BulletState::Butterfly(state) => state.facing(),
        }
    }
    // REMOVE when actually passing damage in
    #[allow(clippy::unit_arg)]
    pub fn on_touch_bullet(&mut self, bullets: &BulletList) {
        match self {
            BulletState::Butterfly(state) => state.on_touch_bullet(bullets),
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
