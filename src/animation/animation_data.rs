
use super::sprite::Sprite;
use crate::assets::Assets;
use crate::timeline::Timeline;
use ggez::{Context, GameResult};
use serde::{Deserialize, Serialize};


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Animation {
	pub frames: Timeline<Option<Sprite>>,
}


impl Animation {
	pub fn load_images(&self, ctx: &mut Context, assets: &mut Assets) -> GameResult<()> {
		for (sprite, _) in &self.frames {
			if let Some(data) = sprite {
				data.load_image(ctx, assets)?
			}
		}
		Ok(())
	}
	pub fn draw(
		ctx: &mut Context,
		assets: &Assets,
		animation: &Animation,
		time: usize,
		world: nalgebra::Matrix4<f32>,
	) -> GameResult<()> {
		use crate::timeline::AtTime;
		let data = animation.frames.at_time(time);
		if let Some(image) = data {
			Sprite::draw(ctx, assets, &image, world)
		} else {
			Ok(())
		}
	}
}