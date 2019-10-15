use crate::input::control_scheme::PadControlScheme;
use crate::input::InputBuffer;
use crate::roster::{Yuyuko, YuyukoState};
use crate::stage::Stage;
use crate::typedefs::collision::IntoGraphical;
use crate::typedefs::graphics::{Matrix4, Vec3};
use gfx::{self, *};
use ggez::event::EventHandler;
use ggez::graphics;
use ggez::timer;
use ggez::{Context, GameResult};
use gilrs::{Event, EventType, Gilrs};
use std::path::PathBuf;

pub struct PlayArea {
    pub width: i32,
}
gfx_defines! { constant Shadow { rate: f32 = "u_Rate", } }
pub struct Match {
    p1: Player,
    p2: Player,
    pads_context: Gilrs,
    background: Stage,
    debug_text: graphics::Text,
    shader: graphics::Shader<Shadow>,
    play_area: PlayArea,
}

pub struct Player {
    resources: Yuyuko,
    state: YuyukoState,
    control_scheme: PadControlScheme,
    input: InputBuffer,
}

impl Player {
    fn update(&mut self, play_area: &PlayArea) {
        self.state
            .update_frame_mut(&self.resources, &self.input, play_area);
    }
    fn update_input<'a>(&mut self, events: impl Iterator<Item = &'a Event>) {
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
    fn draw(
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
}

impl Match {
    pub fn new(ctx: &mut Context, p1: PadControlScheme, p2: PadControlScheme) -> GameResult<Self> {
        let background = Stage::new(ctx, "\\bg_14.png")?;
        let resources = Yuyuko::new_with_path(ctx, PathBuf::from(".\\resources\\yuyuko.json"))?;
        let mut p1_state = YuyukoState::new(&resources);
        let mut p2_state = YuyukoState::new(&resources);
        p1_state.position.x = -1000_00;
        p2_state.position.x = 1000_00;
        Ok(Self {
            p1: Player {
                state: p1_state,
                resources: resources.clone(),
                control_scheme: p1,
                input: InputBuffer::new(),
            },
            p2: Player {
                state: p2_state,
                resources,
                control_scheme: p2,
                input: InputBuffer::new(),
            },
            pads_context: Gilrs::new()?,
            debug_text: graphics::Text::new(""),
            play_area: PlayArea {
                width: background.width() as i32 * 100,
            },
            background,
            shader: graphics::Shader::new(
                ctx,
                "/shaders/vertex.glslv",
                "/shaders/fragment.glslf",
                Shadow { rate: 1.0 },
                "Shadow",
                Some(&[graphics::BlendMode::Alpha]),
            )?,
        })
    }
}

impl EventHandler for Match {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while timer::check_update_time(ctx, 60) {
            let mut events = Vec::new();
            while let Some(event) = self.pads_context.next_event() {
                events.push(event);
            }
            let events = events;
            self.p1.update_input(events.iter());
            self.p2.update_input(events.iter());

            self.p1.update(&self.play_area);
            self.p2.update(&self.play_area);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        let game_offset = Matrix4::new_translation(&Vec3::new(640.0, 660.0, 0.0));

        let p1_x = self.p1.state.position.x.into_graphical();
        let p2_x = self.p2.state.position.x.into_graphical();

        let center_point = (p1_x + p2_x) / 2.0;

        let translate = f32::min(
            self.background.give(ctx),
            f32::max(center_point, -self.background.give(ctx)),
        );

        let world = Matrix4::new_translation(&Vec3::new(-(translate), 0.0, 0.0));

        self.background.draw(ctx, world)?;

        let world = world * game_offset * Matrix4::new_scaling(1.0);

        self.p1.draw(ctx, &self.shader, world)?;
        self.p2.draw(ctx, &self.shader, world)?;

        graphics::set_transform(ctx, Matrix4::identity());
        graphics::apply_transformations(ctx)?;

        graphics::set_blend_mode(ctx, graphics::BlendMode::Alpha)?;

        self.debug_text.fragments_mut()[0].text = format!(
            "current center x: {}
        ",
            translate,
        );
        graphics::draw(ctx, &self.debug_text, graphics::DrawParam::default())?;

        self.p1.state.draw_ui(
            ctx,
            &self.p1.resources,
            Matrix4::new_translation(&Vec3::new(30.0, 600.0, 0.0)),
        )?;
        self.p2.state.draw_ui(
            ctx,
            &self.p2.resources,
            Matrix4::new_translation(&Vec3::new(1130.0, 600.0, 0.0)) * Matrix4::new_scaling(-1.0),
        )?;
        graphics::present(ctx)?;
        Ok(())
    }
}
