use ggez::conf;
use ggez::event;

use ggez::ContextBuilder;

use std::env;
use std::path;

mod animation;
mod assets;
mod game;
mod timeline;

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
        .window_mode(conf::WindowMode::default().dimensions(1280.0, 720.0))
        .build()
        .expect("expected context");


    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut my_game = game::FightingGame::new(&mut ctx).unwrap();
    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}
