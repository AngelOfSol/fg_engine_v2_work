#![allow(
    clippy::type_repetition_in_bounds,
    clippy::zero_prefixed_literal,
    clippy::inconsistent_digit_grouping
)]

use ggez::conf;
use ggez::ContextBuilder;
use std::env;
use std::error::Error;
use std::path;
#[macro_use]
mod imgui_extra;
mod assets;
mod graphics;
mod hitbox;
mod imgui_wrapper;
mod timeline;
mod typedefs;
mod ui;
#[macro_use]
mod character;
mod game_match;
mod input;
#[macro_use]
mod roster;
mod stage;
#[macro_use]
mod command_list;
#[macro_use]
mod input_macros;
mod app_state;
mod button_check;

fn main() {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };
    let mut mode = conf::WindowMode::default().dimensions(1280.0, 720.0);
    for arg in std::env::args() {
        if arg == "--editor" {
            mode = conf::WindowMode::default().dimensions(1500.0, 720.0);
        }
    }
    // Make a Context and an EventLoop.
    let (mut ctx, mut event_loop) = ContextBuilder::new("my_game", "angel")
        .add_resource_path(resource_dir)
        .window_setup(conf::WindowSetup::default().title("my_game").vsync(false))
        .window_mode(mode)
        .build()
        .expect("expected context");

    // first call to dialog doesn't work after creating the ggez Context
    // so we manually call the first one ourselves and let the error pass through
    let result = nfd::dialog().open();

    if let Err(error) = result {
        if error.description() == "Could not initialize COM." {
            println!("Attempted to open unnecessary dialog.  This is in place because the first dialog after building a context breaks.");
        } else {
            println!("Unexpected error: {}", error);
        }
    } else {
        println!("Unexpected success.");
    }
    ggez::graphics::set_default_filter(&mut ctx, ggez::graphics::FilterMode::Nearest);

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let editor = crate::ui::editor::GameEditor::new(&mut ctx).unwrap();
    let mut runner = crate::app_state::AppStateRunner::new(&mut ctx, Box::new(editor)).unwrap();
    ggez::event::run(&mut ctx, &mut event_loop, &mut runner).ok();
}
