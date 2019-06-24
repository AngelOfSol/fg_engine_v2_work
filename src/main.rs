use ggez::event::{self, EventHandler};

use ggez::graphics;
use ggez::{Context, ContextBuilder, GameResult};
use std::env;
use std::path;

use ggez::conf;
fn main() {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };
    // Make a Context and an EventLoop.
    let (mut ctx, mut event_loop) = ContextBuilder::new("my_game", "angel")
        .add_resource_path(resource_dir)
        .window_setup(conf::WindowSetup::default().title("my_game").vsync(false))
        .build()
        .expect("expected context");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut my_game = FightingGame::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

struct FightingGame {
    // Your state here...
    resource: graphics::Image,
}

impl FightingGame {
    fn new(ctx: &mut Context) -> Self {
        let test = graphics::Image::new(ctx, "/stand/000.png").unwrap();
        Self { resource: test }
    }
}

impl EventHandler for FightingGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
        // Update code here...
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let test = &self.resource;
        graphics::draw(ctx, test, graphics::DrawParam::new().dest([0.0f32, 0.0f32]));
        graphics::present(ctx);
        Ok(())
        // Draw code here...
    }
}