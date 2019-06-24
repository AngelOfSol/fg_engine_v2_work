
use super::sprite::Sprite;
use crate::timeline::Timeline;
use ggez::graphics;
use ggez::{Context, GameResult};
use serde::{Deserialize, Serialize};


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Animation<T> {
	pub frames: Timeline<Option<Sprite<T>>>,
}

pub type RenderAnimation = Animation<graphics::Image>;
pub type DataAnimation = Animation<String>;


impl DataAnimation {
	pub fn with_images(self, ctx: &mut Context) -> GameResult<RenderAnimation> {
		Ok(RenderAnimation {
			frames: self
				.frames
				.into_iter()
				.map(|(frame, duration)| {
					frame
						.map(|unresolved| unresolved.with_image(ctx))
						.transpose()
						.map(|resolved| (resolved, duration))
				})
				.collect::<GameResult<Vec<_>>>()?,
		})

	}
}
