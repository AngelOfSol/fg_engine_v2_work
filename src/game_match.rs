use ggez::event::EventHandler;
use ggez::{Context, GameResult};

use ggez::graphics;

use crate::typedefs::graphics::{Matrix4, Vec3};

use crate::stage::Stage;

use crate::roster::{Yuyuko, YuyukoState};

use crate::assets::Assets;

use ggez::timer;

use std::path::PathBuf;

use ggez::event::{KeyCode, KeyMods};

use gilrs::{Button, EventType, Gilrs};

use crate::input::{InputBuffer, PadControlScheme};

pub struct PlayArea {
    // play area is from -320_00 to 320_00, and from -225_00 to 0_00
}

pub struct Match {
    resources: Yuyuko,
    state: YuyukoState,
    control_scheme: PadControlScheme,
    input: InputBuffer,
    pads_context: Gilrs,
    background: Stage,
}

impl Match {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(Self {
            resources: Yuyuko::new_with_path(ctx, PathBuf::from(".\\resources\\yuyuko.json"))?,
            state: YuyukoState::new(),
            pads_context: Gilrs::new()?,
            control_scheme: PadControlScheme::new(),
            input: InputBuffer::new(),
            background: Stage::new(ctx, "\\bg_14.png")?,
        })
    }
}

impl EventHandler for Match {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while timer::check_update_time(ctx, 60) {
            let mut current_frame = self.control_scheme.update_frame(*self.input.top());
            while let Some(event) = self.pads_context.next_event() {
                let id = event.id;
                let event = event.event;
                match event {
                    EventType::ButtonPressed(button, _) => {
                        current_frame = self.control_scheme.handle_press(button, current_frame);
                    }
                    EventType::ButtonReleased(button, _) => {
                        current_frame = self.control_scheme.handle_release(button, current_frame);
                    }
                    _ => (),
                }
            }
            self.input.push(current_frame);

            self.state = self.state.update_frame(&self.resources, &self.input);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        let world = Matrix4::identity();

        self.background.draw(ctx, world)?;

        let world =
            Matrix4::new_translation(&Vec3::new(640.0, 550.0, 0.0)) * Matrix4::new_scaling(2.0);

        self.state.draw(ctx, &self.resources, world)?;

        graphics::present(ctx)?;
        Ok(())
    }
}
