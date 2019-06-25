use crate::animation::Animation;
use crate::assets::Assets;
use crate::timeline::AtTime;

use ggez::event::EventHandler;
use ggez::graphics;

use ggez::timer;
use ggez::{Context, GameResult};

use std::collections::HashMap;


pub struct FightingGame {
    game_state: GameState,
    resource: Animation,
    assets: Assets,
}

enum GameState {
    Animating(AnimatingState),
}

struct AnimatingState {
    frame: usize,
}

impl FightingGame {
    pub fn new(ctx: &mut Context, data: Animation) -> Self {
        let mut assets = Assets {
            images: HashMap::new(),
        };
        data.load_images(ctx, &mut assets).unwrap();
        Self {
            game_state: GameState::Animating(AnimatingState { frame: 0 }),
            resource: data,
            assets,
        }
    }
}

impl EventHandler for FightingGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {

        while timer::check_update_time(ctx, 60) {
            match self.game_state {
                GameState::Animating(ref mut data) => {
                    data.frame += 1;
                }
            }
        }
        Ok(())
        // Update code here...
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let test = &self.resource;
        graphics::clear(ctx, graphics::BLACK);

        match self.game_state {
            GameState::Animating(ref data) => {
                Animation::draw(
                    ctx,
                    &self.assets,
                    &test,
                    data.frame % test.frames.duration(),
                    nalgebra::Translation3::new(100.0, 100.0, 0.0).to_homogeneous(),
                )?;
            }
        }

        graphics::present(ctx)?;

        Ok(())
        // Draw code here...
    }
}
