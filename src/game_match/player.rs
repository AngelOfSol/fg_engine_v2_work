use super::{PlayArea, Shadow};
use crate::hitbox::PositionedHitbox;
use crate::input::control_scheme::PadControlScheme;
use crate::input::InputBuffer;
use crate::roster::generic_character::hit_info::{HitInfo, HitType};
use crate::roster::generic_character::GenericCharacterBehaviour;
use crate::roster::AttackList;
use crate::roster::BulletState;
use crate::roster::{BulletList, Yuyuko, YuyukoState};
use crate::typedefs::collision;
use crate::typedefs::graphics::{Matrix4, Vec3};
use ggez::graphics;
use ggez::{Context, GameResult};
use gilrs::{Event, EventType};
use std::rc::Rc;

// TODO make this generic
#[derive(Clone)]
pub struct Player {
    pub resources: Rc<Yuyuko>,
    pub state: YuyukoState,
    pub control_scheme: Rc<PadControlScheme>,
}

pub struct BulletsContext<'a> {
    pub bullets: &'a BulletList,
    pub attacks: &'a AttackList,
}

impl Player {
    pub fn hitboxes(&self) -> Vec<PositionedHitbox> {
        self.state.hitboxes()
    }
    pub fn hurtboxes(&self) -> Vec<PositionedHitbox> {
        self.state.hurtboxes()
    }

    pub fn bullets_mut(&mut self) -> (BulletsContext, &mut Vec<BulletState>) {
        (
            BulletsContext {
                bullets: &self.resources.bullets,
                attacks: &self.resources.attacks,
            },
            &mut self.state.bullets,
        )
    }

    pub fn get_attack_data(&self) -> Option<HitInfo> {
        self.state.get_attack_data()
    }

    pub fn would_be_hit(
        &self,
        input: &InputBuffer,
        touched: bool,
        info: Option<HitInfo>,
    ) -> HitType {
        self.state.would_be_hit(input, touched, info)
    }
    pub fn take_hit(&mut self, info: &HitType) {
        self.state.take_hit(info)
    }
    pub fn deal_hit(&mut self, info: &HitType) {
        self.state.deal_hit(info);
    }

    pub fn get_pushback(&self, play_area: &PlayArea) -> collision::Int {
        self.state.get_pushback(play_area)
    }
    pub fn apply_pushback(&mut self, force: collision::Int) {
        self.state.apply_pushback(force);
    }

    pub fn prune_bullets(&mut self, play_area: &PlayArea) {
        self.state.prune_bullets(play_area);
    }

    pub fn collision(&self) -> PositionedHitbox {
        self.state.collision()
    }

    pub fn handle_refacing(&mut self, other_player: collision::Int) {
        self.state.handle_refacing(other_player);
    }
    pub fn update(&mut self, input: &InputBuffer, play_area: &PlayArea) {
        self.state.update_frame_mut(input, play_area);
    }
    pub fn draw(
        &self,
        ctx: &mut Context,
        shadow_shader: &graphics::Shader<Shadow>,
        world: Matrix4,
    ) -> GameResult<()> {
        {
            let _lock = graphics::use_shader(ctx, &shadow_shader);
            let skew = Matrix4::new(
                1.0, -0.7, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            );
            let world = world * skew * Matrix4::new_nonuniform_scaling(&Vec3::new(1.0, -0.3, 1.0));

            self.state.draw_shadow(ctx, world)?;
        }

        self.state.draw(ctx, world)?;

        graphics::set_transform(ctx, Matrix4::identity());
        graphics::apply_transformations(ctx)?;

        graphics::set_blend_mode(ctx, graphics::BlendMode::Alpha)?;
        self.state
            .draw_ui(ctx, Matrix4::new_translation(&Vec3::new(30.0, 600.0, 0.0)))
    }

    pub fn draw_bullets(&self, ctx: &mut Context, world: Matrix4) -> GameResult<()> {
        self.state.draw_bullets(ctx, world)
    }

    pub fn draw_particles(&self, ctx: &mut Context, world: Matrix4) -> GameResult<()> {
        self.state.draw_particles(ctx, world)
    }

    pub fn draw_ui(&self, ctx: &mut Context, bottom_line: Matrix4) -> GameResult<()> {
        self.state.draw_ui(ctx, bottom_line)
    }

    pub fn position(&self) -> collision::Vec2 {
        self.state.position
    }
    pub fn position_mut(&mut self) -> &mut collision::Vec2 {
        &mut self.state.position
    }
}
