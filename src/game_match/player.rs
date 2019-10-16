use super::{PlayArea, Shadow};
use crate::input::control_scheme::PadControlScheme;
use crate::input::InputBuffer;
use crate::roster::{Yuyuko, YuyukoState};
use crate::typedefs::collision;
use crate::typedefs::graphics::{Matrix4, Vec3};
use ggez::graphics;
use ggez::{Context, GameResult};
use gilrs::{Event, EventType};

pub struct Player {
    pub resources: Yuyuko,
    pub state: YuyukoState,
    pub control_scheme: PadControlScheme,
    pub input: InputBuffer,
}

impl Player {
    pub fn handle_refacing(&mut self, other_player: collision::Int) {
        self.state.handle_refacing(&self.resources, other_player);
    }
    pub fn update(&mut self, play_area: &PlayArea) {
        self.state
            .update_frame_mut(&self.resources, &self.input, play_area);
    }
    pub fn update_input<'a>(&mut self, events: impl Iterator<Item = &'a Event>) {
        let mut current_frame = self.control_scheme.update_frame(*self.input.top());
        for event in events {
            let Event { id, event, .. } = event;
            if *id == self.control_scheme.gamepad {
                match event {
                    EventType::ButtonPressed(button, _) => {
                        current_frame = self.control_scheme.handle_press(*button, current_frame);
                    }
                    EventType::ButtonReleased(button, _) => {
                        current_frame = self.control_scheme.handle_release(*button, current_frame);
                    }
                    _ => (),
                }
            }
        }
        self.input.push(current_frame);
    }
    pub fn draw(
        &mut self,
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

            self.state.draw_shadow(ctx, &self.resources, world)?;
        }

        self.state.draw(ctx, &self.resources, world)?;

        graphics::set_transform(ctx, Matrix4::identity());
        graphics::apply_transformations(ctx)?;

        graphics::set_blend_mode(ctx, graphics::BlendMode::Alpha)?;
        self.state.draw_ui(
            ctx,
            &self.resources,
            Matrix4::new_translation(&Vec3::new(30.0, 600.0, 0.0)),
        )
    }

    pub fn draw_bullets(&mut self, ctx: &mut Context, world: Matrix4) -> GameResult<()> {
        self.state.draw_bullets(ctx, &self.resources, world)
    }

    pub fn draw_particles(&mut self, ctx: &mut Context, world: Matrix4) -> GameResult<()> {
        self.state.draw_particles(ctx, &self.resources, world)
    }

    pub fn draw_ui(&mut self, ctx: &mut Context, bottom_line: Matrix4) -> GameResult<()> {
        self.state.draw_ui(ctx, &self.resources, bottom_line)
    }

    pub fn position(&self) -> collision::Vec2 {
        self.state.position
    }
}
