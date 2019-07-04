use ggez::error::GameError;
use ggez::graphics;
use ggez::graphics::{Color, DrawMode, Rect, Mesh, DrawParam};
use ggez::{Context, GameResult};

use serde::{Deserialize, Serialize};

use crate::assets::Assets;

use std::io::Read;
use std::path::Path;

use crate::typedefs::graphics::{Vec2, Vec3, Matrix4};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Sprite {
	pub offset: Vec2,
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
		&self,
		ctx: &mut Context,
		assets: &Assets,
		world: Matrix4,
		debug: bool,
	) -> GameResult<()> {
		let image = assets
			.images
			.get(&self.image)
			.ok_or_else(|| GameError::ResourceNotFound(self.image.clone(), Vec::new()))?;

		let image_offset = Matrix4::new_translation(&Vec3::new(
			-f32::from(image.width()) / 2.0,
			-f32::from(image.height()) / 2.0,
			0.0,
		));

		let sprite_offset = Matrix4::new_translation(&Vec3::new(
			self.offset.x,
			self.offset.y,
			0.0,
		));

		let transform = world * image_offset * sprite_offset;

		graphics::set_transform(ctx, transform);
		graphics::apply_transformations(ctx)?;

		graphics::draw(ctx, image, DrawParam::default())?;


		if debug {
			let rectangle = Mesh::new_rectangle(
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
			graphics::draw(ctx, &rectangle, DrawParam::default())?;
		}
		graphics::set_transform(ctx, Matrix4::identity());
		graphics::apply_transformations(ctx)?;

		Ok(())
	}
	pub fn draw_debug(
		&self,
		ctx: &mut Context,
		assets: &Assets,
		world: Matrix4,
	) -> GameResult<()> {
		self.draw_ex(ctx, assets, world, true)
	}

	pub fn draw(
		&self,
		ctx: &mut Context,
		assets: &Assets,
		world: Matrix4,
	) -> GameResult<()> {
		self.draw_ex(ctx, assets, world, false)
	}
	
}