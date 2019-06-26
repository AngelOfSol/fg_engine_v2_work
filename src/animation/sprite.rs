use ggez::error::GameError;
use ggez::graphics;
use ggez::{Context, GameResult};

use serde::{Deserialize, Serialize};

use crate::assets::Assets;

use rgb::ComponentBytes;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Sprite {
	pub offset: nalgebra::Vector2<f32>,
	pub image: String,
	pub rotation: f32,
}

pub fn load_image<S: Into<String>>(
	key: S,
	path: &str,
	ctx: &mut Context,
	assets: &mut Assets,
) -> GameResult<()> {
	let key = key.into();
	let image = match graphics::Image::new(ctx, path) {
		Ok(result) => result,
		Err(GameError::ResourceNotFound(_, _)) => {
			let raw_image = lodepng::decode32_file(path).map_err(|_| {
				GameError::ResourceNotFound(format!("image not valid at '{}'", path), vec![])
			})?;
			graphics::Image::from_rgba8(
				ctx,
				raw_image.width as u16,
				raw_image.height as u16,
				&raw_image.buffer.as_bytes(),
			)?
		}
		Err(err) => return Err(err),
	};
	assets.images.insert(key, image);
	Ok(())
}


impl Sprite {
	pub fn new<S: Into<String>>(path: S) -> Self {
		Self {
			offset: nalgebra::zero(),
			image: path.into(),
			rotation: 0.0,
		}
	}

	pub fn load_image(&self, ctx: &mut Context, assets: &mut Assets) -> GameResult<()> {
		if assets.images.contains_key(&self.image) {
			return Ok(());
		}
		load_image(self.image.clone(), &self.image, ctx, assets)
	}
	pub fn draw(
		ctx: &mut Context,
		assets: &Assets,
		sprite: &Sprite,
		world: nalgebra::Matrix4<f32>,
	) -> GameResult<()> {
		let image = assets
			.images
			.get(&sprite.image)
			.ok_or_else(|| GameError::ResourceNotFound(sprite.image.clone(), Vec::new()))?;
		graphics::set_transform(
			ctx,
			world
				* nalgebra::Translation3::new(
					sprite.offset.x - f32::from(image.width()) / 2.0,
					sprite.offset.y - f32::from(image.height()),
					0.0,
				)
				.to_homogeneous()
				* nalgebra::Rotation3::from_axis_angle(
					&nalgebra::Vector3::z_axis(),
					sprite.rotation,
				)
				.to_homogeneous(),
		);
		graphics::apply_transformations(ctx)?;
		graphics::draw(ctx, image, graphics::DrawParam::default())
	}
}