pub mod bullet;
pub mod combo_state;
pub mod extra_data;
pub mod hit_info;
pub mod move_id;
pub mod particle_id;

#[macro_use]
pub mod macros;
pub mod impls;

use crate::assets::Assets;
use crate::character::state::components::GlobalParticle;
use crate::game_match::sounds::{GlobalSound, SoundList};
use crate::game_match::UiElements;
use crate::game_match::{FlashType, PlayArea};
use crate::graphics::particle::Particle;
use crate::hitbox::PositionedHitbox;
use crate::input::{Facing, InputState};
use crate::roster::AttackInfo;
use crate::typedefs::{collision, graphics};
use enum_dispatch::enum_dispatch;
use ggez::{Context, GameResult};
use hit_info::{HitAction, HitEffect, HitResult};
use rodio::Device;
use std::collections::HashMap;

#[enum_dispatch(OpaqueBullet)]
pub trait BulletMut {
    fn hitboxes(&self) -> Vec<PositionedHitbox>;
    fn on_touch_bullet(&mut self);
    fn attack_data(&self) -> AttackInfo;
    fn deal_hit(&mut self, hit: &HitResult);
    fn hash(&self) -> u64;
    fn facing(&self) -> Facing;
}

#[enum_dispatch(CharacterBehavior)]
pub trait GenericCharacterBehaviour {
    fn apply_pushback(&mut self, force: collision::Int);
    fn get_pushback(&self, play_area: &PlayArea) -> collision::Int;

    fn collision(&self) -> PositionedHitbox;
    fn hitboxes(&self) -> Vec<PositionedHitbox>;
    fn hurtboxes(&self) -> Vec<PositionedHitbox>;

    fn get_attack_data(&self) -> Option<HitAction>;

    fn prune_bullets(&mut self, play_area: &PlayArea);

    fn would_be_hit(
        &self,
        input: &[InputState],
        total_info: HitAction,
        effect: Option<HitEffect>,
    ) -> (Option<HitEffect>, Option<HitResult>);

    fn take_hit(&mut self, info: HitEffect, play_area: &PlayArea);
    fn deal_hit(&mut self, info: &HitResult);
    fn handle_refacing(&mut self, other_player: collision::Int);

    fn update_frame_mut(
        &mut self,
        input: &[InputState],
        opponent_position: collision::Vec2,
        play_area: &PlayArea,
        global_particles: &HashMap<GlobalParticle, Particle>,
    );
    fn update_cutscene(&mut self, play_area: &PlayArea);
    fn update_no_input(
        &mut self,
        play_area: &PlayArea,
        global_particles: &HashMap<GlobalParticle, Particle>,
    );

    #[allow(clippy::clippy::too_many_arguments)]
    fn draw_ui(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        ui: &UiElements,
        bottom_line: graphics::Matrix4,
        flipped: bool,
        wins: usize,
        first_to: usize,
    ) -> GameResult<()>;

    fn draw(&self, ctx: &mut Context, assets: &Assets, world: graphics::Matrix4) -> GameResult<()>;
    fn draw_particles(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        world: graphics::Matrix4,
        global_particles: &HashMap<GlobalParticle, Particle>,
    ) -> GameResult<()>;

    fn draw_bullets(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        world: graphics::Matrix4,
    ) -> GameResult<()>;
    fn draw_shadow(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        world: graphics::Matrix4,
    ) -> GameResult<()>;

    fn render_sound(
        &mut self,
        audio_device: &Device,
        sound_list: &SoundList<GlobalSound>,
        fps: u32,
    );

    fn position(&self) -> collision::Vec2;
    fn position_mut(&mut self) -> &mut collision::Vec2;

    fn velocity(&self) -> collision::Vec2;
    fn facing(&self) -> Facing;
    fn bullets_mut(&mut self) -> OpaqueBulletIterator;
    fn in_cutscene(&self) -> bool;
    fn draw_order_priority(&self) -> i32;

    fn save(&self) -> GameResult<OpaqueStateData>;
    fn load(&mut self, value: OpaqueStateData) -> GameResult<()>;

    fn get_flash(&self) -> Option<FlashType>;
    fn get_lockout(&self) -> (i32, bool);
    fn modify_lockout(&mut self, timer: i32, reset: bool);
    fn is_locked_out(&self) -> bool;
    fn validate_position(&mut self, play_area: &PlayArea);
    fn is_dead(&self) -> bool;
    fn health(&self) -> i32;

    fn reset_to_position_roundstart(
        &mut self,
        play_area: &PlayArea,
        position: collision::Int,
        facing: Facing,
    );
    fn reset_to_position_gamestart(
        &mut self,
        play_area: &PlayArea,
        position: collision::Int,
        facing: Facing,
    );
}

use super::yuyuko::{YuyukoBulletIterator, YuyukoBulletMut, YuyukoState};

#[derive(Clone)]
#[non_exhaustive]
pub enum OpaqueStateData {
    Yuyuko(YuyukoState),
    #[allow(dead_code)]
    Broken,
}

#[enum_dispatch]
pub enum OpaqueBulletIterator<'a> {
    YuyukoIter(YuyukoBulletIterator<'a>),
}

impl<'a> Iterator for OpaqueBulletIterator<'a> {
    type Item = OpaqueBullet<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            OpaqueBulletIterator::YuyukoIter(iter) => iter.next(),
        }
    }
}

#[enum_dispatch]
pub enum OpaqueBullet<'a> {
    YuyukoBulletMut(YuyukoBulletMut<'a>),
}
