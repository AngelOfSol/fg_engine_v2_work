use super::super::{AttackList, BulletList};
use crate::assets::Assets;
use crate::game_match::PlayArea;
use crate::hitbox::PositionedHitbox;
use crate::input::Facing;
use crate::roster::generic_character::hit_info::{HitInfo, HitType};
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
    frame: usize,
}

impl ButterflySpawn {
    pub fn get_spawn_frame(&self) -> usize {
        self.frame
    }
    pub fn instantiate(&self, current_position: Vec2, facing: Facing) -> ButterflyState {
        ButterflyState {
            alive: true,
            position: current_position + facing.fix_collision(self.offset),
            velocity: facing.fix_collision(Vec2::new(self.x_vel, self.y_vel)),
            rotation: facing.fix_rotation(self.angle as f32 * std::f32::consts::PI / -180.0),
            facing,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ButterflyState {
    position: Vec2,
    velocity: Vec2,
    rotation: Float,
    alive: bool,
    facing: Facing,
}

impl ButterflyState {
    pub fn update(&mut self, _: &BulletList) {
        self.position += self.velocity;
    }
    pub fn alive(&self, bullets: &BulletList, area: &PlayArea) -> bool {
        let in_x_bounds =
            i32::abs(self.position.x) <= area.width / 2 + bullets.butterfly.hitbox.half_size.x;

        let in_y_bounds =
            i32::abs(self.position.y) <= area.width / 2 + bullets.butterfly.hitbox.half_size.y;

        self.alive && in_x_bounds && in_y_bounds
    }

    pub fn hitbox(&self, bullets: &BulletList) -> Vec<PositionedHitbox> {
        vec![bullets.butterfly.hitbox.with_position(self.position)]
    }

    pub fn attack_data(&self, bullets: &BulletList, attacks: &AttackList) -> HitInfo {
        HitInfo::Bullet(attacks[&bullets.butterfly.attack_id].clone(), self.facing)
    }

    // TODO, make this return a HitType too, that way it can determine if it can hit again
    // for the purpose of multi hit bullets
    pub fn on_touch(&mut self, _: &BulletList, hit_type: &HitType) {
        match hit_type {
            HitType::Block(_)
            | HitType::WrongBlock(_)
            | HitType::Graze(_)
            | HitType::Hit(_)
            | HitType::CounterHit(_) => self.alive = false,

            HitType::Whiff => (),
        }
    }

    pub fn on_touch_bullet(&mut self, _: &BulletList, _: ()) {
        self.alive = false;
    }

    pub fn draw(
        &self,
        ctx: &mut Context,
        data: &BulletList,
        assets: &Assets,
        world: graphics::Matrix4,
    ) -> GameResult<()> {
        data.butterfly.animation.draw_at_time(
            ctx,
            assets,
            0,
            world
                * graphics::Matrix4::new_translation(&graphics::up_dimension(
                    self.position.into_graphical(),
                ))
                * graphics::Matrix4::new_rotation(
                    nalgebra::Vector3::new(0.0, 0.0, 1.0) * self.rotation,
                ),
        )
    }
}
