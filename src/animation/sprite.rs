use ggez::graphics;
use ggez::{Context, GameResult};
use serde::{Deserialize, Serialize, };

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Sprite<T> {
	pub offset: (i32, i32),
	pub image: T,
	pub rotation: f32,
}

pub type RenderSprite = Sprite<graphics::Image>;
pub type DataSprite = Sprite<String>;

impl DataSprite {
	pub fn with_image(self, ctx: &mut Context) -> GameResult<RenderSprite> {
		Ok(RenderSprite {
			offset: self.offset,
			image:  graphics::Image::new(ctx, self.image)?,
			rotation: self.rotation,
		})
	}
}

