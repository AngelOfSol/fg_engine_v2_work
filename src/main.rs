use ggez::conf;
use ggez::event::{self, EventHandler};
use ggez::graphics;

use ggez::timer;
use ggez::{Context, ContextBuilder, GameResult};

use std::collections::HashMap;
use std::env;
use std::path;

mod animation;
mod assets;
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


    let test = animation::Animation {
        frames: vec![
            (
                Some(animation::Sprite {
                    offset: nalgebra::zero(),
                    image: "/yuyuko/stand/000.png".to_owned(),
                    rotation: 0.0,
                }),
                6,
            ),
            (
                Some(animation::Sprite {
                    offset: nalgebra::zero(),
                    image: "/yuyuko/stand/001.png".to_owned(),
                    rotation: 0.0,
                }),
                6,
            ),
            (
                Some(animation::Sprite {
                    offset: nalgebra::zero(),
                    image: "/yuyuko/stand/002.png".to_owned(),
                    rotation: 0.0,
                }),
                6,
            ),
            (
                Some(animation::Sprite {
                    offset: nalgebra::zero(),
                    image: "/yuyuko/stand/003.png".to_owned(),
                    rotation: 0.0,
                }),
                6,
            ),
            (
                Some(animation::Sprite {
                    offset: nalgebra::zero(),
                    image: "/yuyuko/stand/004.png".to_owned(),
                    rotation: 0.0,
                }),
                6,
            ),
            (
                Some(animation::Sprite {
                    offset: nalgebra::zero(),
                    image: "/yuyuko/stand/005.png".to_owned(),
                    rotation: 0.0,
                }),
                6,
            ),
            (
                Some(animation::Sprite {
                    offset: nalgebra::zero(),
                    image: "/yuyuko/stand/006.png".to_owned(),
                    rotation: 0.0,
                }),
                6,
            ),
            (
                Some(animation::Sprite {
                    offset: nalgebra::zero(),
                    image: "/yuyuko/stand/007.png".to_owned(),
                    rotation: 0.0,
                }),
                6,
            ),
            (
                Some(animation::Sprite {
                    offset: nalgebra::zero(),
                    image: "/yuyuko/stand/008.png".to_owned(),
                    rotation: 0.0,
                }),
                6,
            ),
            (
                Some(animation::Sprite {
                    offset: nalgebra::zero(),
                    image: "/yuyuko/stand/009.png".to_owned(),
                    rotation: 0.0,
                }),
                6,
            ),
        ],
    };
    println!("{}", serde_json::to_string(&test).unwrap());

    let mut assets = assets::Assets {
        images: HashMap::new(),
    };

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("./resources/animation.json")
        .unwrap();
    use std::io::Write;
    let _ = file.write(serde_json::to_string(&test).unwrap().as_ref());
    let _ = file.flush();

    let file = std::fs::File::open("./resources/animation.json").unwrap();
    let buf_read = std::io::BufReader::new(file);
    let animation: animation::Animation =
        serde_json::from_reader::<_, animation::Animation>(buf_read).unwrap();

    animation.load_images(&mut ctx, &mut assets).unwrap();


    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut my_game = FightingGame::new(&mut ctx, animation, assets);
    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

struct FightingGame {
    current_frame: usize,
    resource: animation::Animation,
    assets: assets::Assets,
}

impl FightingGame {
    fn new(_ctx: &mut Context, data: animation::Animation, assets: assets::Assets) -> Self {
        Self {
            current_frame: 0,
            resource: data,
            assets,
        }
    }
}

impl EventHandler for FightingGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {

        while timer::check_update_time(ctx, 60) {
            self.current_frame += 1;
        }
        Ok(())
        // Update code here...
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        use timeline::AtTime;

        let test = &self.resource;
        graphics::clear(ctx, graphics::BLACK);

        animation::Animation::draw(
            ctx,
            &self.assets,
            &test,
            self.current_frame % test.frames.duration(),
            nalgebra::Translation3::new(100.0, 100.0, 0.0).to_homogeneous(),
        )?;

        graphics::present(ctx)?;

        Ok(())
        // Draw code here...
    }
}
