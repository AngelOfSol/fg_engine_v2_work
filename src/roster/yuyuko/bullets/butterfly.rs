use super::super::particles::ParticleId;
use super::super::{AttackList, BulletList, Yuyuko};
use crate::assets::Assets;
use crate::game_match::PlayArea;
use crate::hitbox::PositionedHitbox;
use crate::input::Facing;
use crate::roster::generic_character::hit_info::{HitEffectType, HitResult};
use crate::roster::AttackInfo;
use crate::typedefs::collision::*;
use crate::typedefs::graphics::{self, Float};
use ggez::{Context, GameResult};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ButterflySpawn {
    offset: Vec2,
    angle: Int,
    x_vel: Int,
    y_vel: Int,
    color: usize,
    frame: usize,
}

impl ButterflySpawn {
    pub fn get_spawn_frame(&self) -> usize {
        self.frame
    }
    pub fn instantiate(&self, current_position: Vec2, facing: Facing) -> ButterflyState {
        ButterflyState {
            alive_duration: 0,
            alive: true,
            color: self.color,
            position: current_position + facing.fix_collision(self.offset),
            velocity: facing.fix_collision(Vec2::new(self.x_vel, self.y_vel)),
            rotation: facing.fix_rotation(self.angle as f32 * std::f32::consts::PI / -180.0),
            facing,
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct ButterflyState {
    position: Vec2,
    velocity: Vec2,
    rotation: Float,
    alive: bool,
    color: usize,
    facing: Facing,
    alive_duration: i32,
}

impl ButterflyState {
    pub fn update(&mut self, _: &Yuyuko) {
        self.alive_duration += 1;
        self.position += self.velocity;
    }
    pub fn alive(&self, data: &Yuyuko, area: &PlayArea) -> bool {
        let in_x_bounds =
            i32::abs(self.position.x) <= area.width / 2 + data.bullets.butterfly.hitbox.half_size.x;

        let in_y_bounds =
            i32::abs(self.position.y) <= area.width / 2 + data.bullets.butterfly.hitbox.half_size.y;

        self.alive && in_x_bounds && in_y_bounds
    }

    pub fn hitbox(&self, bullets: &BulletList) -> Vec<PositionedHitbox> {
        vec![bullets.butterfly.hitbox.with_position(self.position)]
    }

    pub fn attack_data(&self, bullets: &BulletList, attacks: &AttackList) -> AttackInfo {
        attacks[&bullets.butterfly.attack_id].clone()
    }

    pub fn deal_hit(&mut self, _: &BulletList, hit_result: &HitResult) {
        match hit_result.hit_type {
            HitEffectType::Block
            | HitEffectType::WrongBlock
            | HitEffectType::Hit
            | HitEffectType::CounterHit
            | HitEffectType::GuardCrush
            | HitEffectType::GrazeCrush
            | HitEffectType::Graze => self.alive = false,
        }
    }
    pub fn hash(&self) -> u64 {
        0
    }

    pub fn facing(&self) -> Facing {
        self.facing
    }

    pub fn on_touch_bullet(&mut self, _: &BulletList, _: ()) {
        self.alive = false;
    }

    pub fn draw(
        &self,
        ctx: &mut Context,
        data: &Yuyuko,
        assets: &Assets,
        world: graphics::Matrix4,
    ) -> GameResult<()> {
        let world = world
            * graphics::Matrix4::new_translation(&graphics::up_dimension(
                self.position.into_graphical(),
            ));
        let scale = f32::cos(self.alive_duration as f32 / 25.0);
        data.particles[&ParticleId::ButterflyFlare].draw_frame(
            ctx,
            assets,
            self.color,
            world * graphics::Matrix4::new_scaling(scale),
        )?;
        data.bullets.butterfly.animation.draw_frame(
            ctx,
            assets,
            self.color,
            world
                * graphics::Matrix4::new_rotation(
                    nalgebra::Vector3::new(0.0, 0.0, 1.0) * self.rotation,
                ),
        )
    }
}
