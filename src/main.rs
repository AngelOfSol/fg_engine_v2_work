use ggez::event::{self, EventHandler};

use ggez::graphics;
use ggez::{Context, ContextBuilder, GameResult};
use std::env;
use std::path;

use ggez::conf;
use ggez::timer;

mod animation;
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
        .build()
        .expect("expected context");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut my_game = FightingGame::new(&mut ctx);

    let test = animation::DataAnimation {
        frames: vec![
            (
                Some(animation::DataSprite {
                    offset: (0, 0),
                    image: "/yuyuko/stand/000.png".to_owned(),
                    rotation: 0.0,
                }),
                6,
            ),
            (
                Some(animation::DataSprite {
                    offset: (0, 0),
                    image: "/yuyuko/stand/001.png".to_owned(),
                    rotation: 0.0,
                }),
                6,
            ),
            (
                Some(animation::DataSprite {
                    offset: (0, 0),
                    image: "/yuyuko/stand/002.png".to_owned(),
                    rotation: 0.0,
                }),
                6,
            ),
            (
                Some(animation::DataSprite {
                    offset: (0, 0),
                    image: "/yuyuko/stand/003.png".to_owned(),
                    rotation: 0.0,
                }),
                6,
            ),
            (
                Some(animation::DataSprite {
                    offset: (0, 0),
                    image: "/yuyuko/stand/004.png".to_owned(),
                    rotation: 0.0,
                }),
                6,
            ),
            (
                Some(animation::DataSprite {
                    offset: (0, 0),
                    image: "/yuyuko/stand/005.png".to_owned(),
                    rotation: 0.0,
                }),
                6,
            ),
            (
                Some(animation::DataSprite {
                    offset: (0, 0),
                    image: "/yuyuko/stand/006.png".to_owned(),
                    rotation: 0.0,
                }),
                6,
            ),
            (
                Some(animation::DataSprite {
                    offset: (0, 0),
                    image: "/yuyuko/stand/007.png".to_owned(),
                    rotation: 0.0,
                }),
                6,
            ),
            (
                Some(animation::DataSprite {
                    offset: (0, 0),
                    image: "/yuyuko/stand/008.png".to_owned(),
                    rotation: 0.0,
                }),
                6,
            ),
            (
                Some(animation::DataSprite {
                    offset: (0, 0),
                    image: "/yuyuko/stand/009.png".to_owned(),
                    rotation: 0.0,
                }),
                6,
            ),
        ],
    };
    println!("{}", serde_json::to_string(&test).unwrap());

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
    let test: animation::DataAnimation = serde_json::from_reader(buf_read).unwrap();
    println!("test: {:?}", test);
    let next = test.with_images(&mut ctx).unwrap();
    use timeline::AtTime;
    println!("test: {:?}", next.frames.duration());


    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

struct FightingGame {
    current_frame: usize,
    resource: [graphics::Image; 10],
}

impl FightingGame {
    fn new(ctx: &mut Context) -> Self {
        Self {
            current_frame: 0,
            resource: [
                graphics::Image::new(ctx, "/yuyuko/stand/000.png").unwrap(),
                graphics::Image::new(ctx, "/yuyuko/stand/001.png").unwrap(),
                graphics::Image::new(ctx, "/yuyuko/stand/002.png").unwrap(),
                graphics::Image::new(ctx, "/yuyuko/stand/003.png").unwrap(),
                graphics::Image::new(ctx, "/yuyuko/stand/004.png").unwrap(),
                graphics::Image::new(ctx, "/yuyuko/stand/005.png").unwrap(),
                graphics::Image::new(ctx, "/yuyuko/stand/006.png").unwrap(),
                graphics::Image::new(ctx, "/yuyuko/stand/007.png").unwrap(),
                graphics::Image::new(ctx, "/yuyuko/stand/008.png").unwrap(),
                graphics::Image::new(ctx, "/yuyuko/stand/009.png").unwrap(),
            ],
        }
    }
}

impl EventHandler for FightingGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {

        while timer::check_update_time(ctx, 60) {
            self.current_frame += 1;
            self.current_frame %= 10;
        }
        Ok(())
        // Update code here...
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let test = &self.resource;
        graphics::clear(ctx, graphics::BLACK);
        let _ = graphics::draw(
            ctx,
            &test[self.current_frame],
            graphics::DrawParam::new().dest([0.0f32, 0.0f32]),
        );
        let _ = graphics::present(ctx);
        Ok(())
        // Draw code here...
    }
}
