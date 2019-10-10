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
    resources: Yuyuko,
    state: YuyukoState,
    control_scheme: PadControlScheme,
    input: InputBuffer,
    pads_context: Gilrs,
    background: Stage,
    debug_text: graphics::Text,
    shader: graphics::Shader<Shadow>,
    play_area: PlayArea,
}

impl Match {
    pub fn new(ctx: &mut Context, control_scheme: PadControlScheme) -> GameResult<Self> {
        let background = Stage::new(ctx, "\\bg_14.png")?;
        Ok(Self {
            resources: Yuyuko::new_with_path(ctx, PathBuf::from(".\\resources\\yuyuko.json"))?,
            state: YuyukoState::new(),
            pads_context: Gilrs::new()?,
            control_scheme,
            input: InputBuffer::new(),
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
            let mut current_frame = self.control_scheme.update_frame(*self.input.top());
            while let Some(event) = self.pads_context.next_event() {
                // let id = event.id;
                let Event { id, event, .. } = event;
                if id == self.control_scheme.gamepad {
                    match event {
                        EventType::ButtonPressed(button, _) => {
                            current_frame = self.control_scheme.handle_press(button, current_frame);
                        }
                        EventType::ButtonReleased(button, _) => {
                            current_frame =
                                self.control_scheme.handle_release(button, current_frame);
                        }
                        _ => (),
                    }
                }
            }
            self.input.push(current_frame);

            self.state
                .update_frame_mut(&self.resources, &self.input, &self.play_area);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        let game_offset = Matrix4::new_translation(&Vec3::new(640.0, 660.0, 0.0));

        let char_position = (self.state.position.x.into_graphical() + 100.0) / 2.0;

        let translate = f32::min(
            self.background.give(ctx),
            f32::max(char_position, -self.background.give(ctx)),
        );

        let world = Matrix4::new_translation(&Vec3::new(-(translate), 0.0, 0.0));

        self.background.draw(ctx, world)?;

        let world = world * game_offset * Matrix4::new_scaling(1.0);
        {
            let _lock = graphics::use_shader(ctx, &self.shader);
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
        /*
                self.debug_text.fragments_mut()[0].text = format!(
                    "Frame: {}, State: {}",
                    self.state.current_state.0,
                    self.state.current_state.1.to_string()
                );
                graphics::draw(ctx, &self.debug_text, graphics::DrawParam::default())?;
        */
        graphics::present(ctx)?;
        Ok(())
    }
}
