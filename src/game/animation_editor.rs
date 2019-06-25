use ggez::{Context, GameResult};

use crate::animation::Animation;
use crate::timeline::AtTime;

use crate::assets::Assets;

pub struct AnimationEditor {
    pub frame: usize,
    resource: Animation,
}

impl AnimationEditor {
    pub fn new(ctx: &mut Context, assets: &mut Assets) -> GameResult<Self> {
        let file = std::fs::File::open("./resources/animation.json").unwrap();
        let buf_read = std::io::BufReader::new(file);
        let resource: Animation = serde_json::from_reader::<_, Animation>(buf_read).unwrap();
        resource.load_images(ctx, assets)?;
        Ok(Self { frame: 0, resource })
    }

    pub fn update(&mut self) -> GameResult<()> {
        self.frame += 1;
        Ok(())
    }

    pub fn draw(&self, ctx: &mut Context, assets: &Assets) -> GameResult<()> {
        Animation::draw(
            ctx,
            assets,
            &self.resource,
            self.frame % self.resource.frames.duration(),
            nalgebra::Translation3::new(100.0, 100.0, 0.0).to_homogeneous(),
        )?;

        Ok(())
    }
}