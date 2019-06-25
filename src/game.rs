mod animation_editor;

use crate::assets::Assets;

use ggez::event::EventHandler;
use ggez::graphics;
use ggez::timer;
use ggez::{Context, GameResult};

use std::collections::HashMap;

use animation_editor::AnimationEditor;

pub struct FightingGame {
    game_state: GameState,
    assets: Assets,
}

enum GameState {
    Animating(AnimationEditor),
}

impl FightingGame {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let mut assets = Assets {
            images: HashMap::new(),
        };
        Ok(Self {
            game_state: GameState::Animating(AnimationEditor::new(ctx, &mut assets)?),
            assets,
        })
    }
}

impl EventHandler for FightingGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {

        while timer::check_update_time(ctx, 60) {
            match self.game_state {
                GameState::Animating(ref mut editor) => editor.update(),
            }?;
        }
        Ok(())
        // Update code here...
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        match self.game_state {
            GameState::Animating(ref editor) => editor.draw(ctx, &self.assets),
        }?;

        graphics::present(ctx)?;

        Ok(())
        // Draw code here...
    }
}
