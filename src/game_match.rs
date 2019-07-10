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

pub struct PlayArea {
    // play area is from -320_00 to 320_00, and from -225_00 to 0_00
}

pub struct Match {
    resources: Yuyuko,
    state: YuyukoState,
    input: bool,
    background: Stage,
}

impl Match {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(Self {
            resources: Yuyuko::new_with_path(ctx, PathBuf::from(".\\resources\\yuyuko.json"))?,
            state: YuyukoState::new(),
            input: false,
            background: Stage::new(ctx, "\\bg_14.png")?,
        })
    }
}

impl EventHandler for Match {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while timer::check_update_time(ctx, 60) {
            if self.input {
                self.state = self.state.update_frame(&self.resources);
            }
        }
        Ok(())
    }

    
    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        keymod: KeyMods,
        _repeat: bool,
    ) {
        if keycode == KeyCode::A {
            self.input = true;
        }
    }
    
    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        keymod: KeyMods,
    ) {
        if keycode == KeyCode::A {
            self.input = false;
        }
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
