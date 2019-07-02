use ggez::error::GameError;
use ggez::graphics;
use ggez::graphics::{Color, DrawMode, Rect};
use ggez::{Context, GameResult};

use serde::{Deserialize, Serialize};

use crate::assets::Assets;

use rgb::ComponentBytes;
use std::io::Read;
use std::path::Path;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Sprite {
	pub offset: nalgebra::Vector2<f32>,
	pub image: String,
	pub rotation: f32,
}


pub fn load_image<S: Into<String>, P: AsRef<Path>>(
	key: S,
	path: P,
	ctx: &mut Context,
	assets: &mut Assets,
) -> GameResult<()> {
	let key = key.into();
	let image = match graphics::Image::new(ctx, &path) {
		Ok(result) => result,
		Err(GameError::ResourceNotFound(_, _)) => {
			let img = {
				let mut buf = Vec::new();
				let mut reader = std::fs::File::open(&path)?;
				let _ = reader.read_to_end(&mut buf)?;
				image::load_from_memory(&buf)?.to_rgba()
			};
			let (width, height) = img.dimensions();

			graphics::Image::from_rgba8(ctx, width as u16, height as u16, &img)?
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

	pub fn draw_ex(
		ctx: &mut Context,
		assets: &Assets,
		sprite: &Sprite,
		world: nalgebra::Matrix3<f32>,
		debug: bool,
	) -> GameResult<()> {
		let image = assets
			.images
			.get(&sprite.image)
			.ok_or_else(|| GameError::ResourceNotFound(sprite.image.clone(), Vec::new()))?;

		let base = nalgebra::Point2::new(0.0f32, 0.0f32);
		let transform = nalgebra::Translation2::new(
			sprite.offset.x - f32::from(image.width()) / 2.0,
			sprite.offset.y - f32::from(image.height()),
		)
		.to_homogeneous()
			* world;
		graphics::draw(
			ctx,
			image,
			graphics::DrawParam::default().dest(transform.transform_point(&base)),
		)?;

		if debug {
			let origin = transform.transform_point(&base);
			let rectangle = graphics::Mesh::new_rectangle(
				ctx,
				DrawMode::stroke(1.0),
				Rect::new(
					0.0,
					0.0,
					f32::from(image.width()),
					f32::from(image.height()),
				),
				Color::new(1.0, 0.0, 0.0, 1.0),
			)?;
			graphics::draw(ctx, &rectangle, graphics::DrawParam::default().dest(origin))?;
		}

		Ok(())
	}
	pub fn draw_debug(

		ctx: &mut Context,
		assets: &Assets,
		sprite: &Sprite,
		world: nalgebra::Matrix3<f32>,

	) -> GameResult<()> {
		Self::draw_ex(ctx, assets, sprite, world, true)

	}
	pub fn draw(
		ctx: &mut Context,
		assets: &Assets,
		sprite: &Sprite,
		world: nalgebra::Matrix3<f32>,
	) -> GameResult<()> {
		Self::draw_ex(ctx, assets, sprite, world, false)

	}
}