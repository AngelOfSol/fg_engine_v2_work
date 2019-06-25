use ggez::graphics;
use ggez::{Context, GameResult};
use serde::{Deserialize, Serialize};

use crate::assets::Assets;


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Sprite {
	pub offset: nalgebra::Vector2<f32>,
	pub image: String,
	pub rotation: f32,
}


impl Sprite {
	pub fn load_image(&self, ctx: &mut Context, assets: &mut Assets) -> GameResult<()> { 
		assets.images.insert(self.image.clone(), graphics::Image::new(ctx, &self.image)?);
		Ok(())
	}
	pub fn draw(
		ctx: &mut Context,
		assets: &Assets,
		sprite: &Sprite,
		world: nalgebra::Matrix4<f32>,
	) -> GameResult<()> {
		let image = &assets.images[&sprite.image];
		graphics::set_transform(
			ctx,
			world
				* nalgebra::Translation3::new(
					sprite.offset.x - f32::from(image.width()) / 2.0,
					sprite.offset.y - f32::from(image.height()),
					0.0,
				)
				.to_homogeneous(),
		);
		graphics::apply_transformations(ctx)?;
		graphics::draw(ctx, image, graphics::DrawParam::default())
	}
}