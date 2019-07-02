use super::sprite::{load_image, Sprite};
use crate::assets::Assets;
use crate::timeline::{AtTime, Timeline};

use crate::imgui_extra::UiExtensions;

use ggez::{Context, GameResult};
use imgui::{im_str, ImString};
use serde::{Deserialize, Serialize};

use std::convert::TryFrom;


use std::fs::File;
use std::path::Path;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Animation {
	pub name: String,
	pub frames: Timeline<Sprite>,
}

pub struct AnimationUi {
	pub current_sprite: Option<usize>,
}

impl AnimationUi {
	pub fn new() -> Self {
		Self {
			current_sprite: None,
		}
	}
}

impl Animation {
	pub fn new<S: Into<String>>(name: S) -> Self {
		Self {
			name: name.into(),
			frames: Timeline::new(),
		}
	}

	pub fn load_images(&self, ctx: &mut Context, assets: &mut Assets) -> GameResult<()> {
		for (sprite, _) in &self.frames {
			sprite.load_image(ctx, assets)?
		}
		Ok(())
	}

	pub fn save_tar<P: AsRef<Path>>(
		&self,
		ctx: &mut Context,
		assets: &Assets,
		path: P,
	) -> GameResult<()> {
		let file = File::create(path)?;
		let mut tar = tar::Builder::new(file);

		let data_file_name = "data.json"; // format!("{}-animation.json", self.name);
		{
			let mut json_target = File::create(&data_file_name)?;
			serde_json::to_writer(&mut json_target, &self)
				.map_err(|err| ggez::GameError::FilesystemError(format!("{}", err)))?;
		}

		tar.append_path(&data_file_name)?;
		std::fs::remove_file(&data_file_name)?;

		for (sprite, _) in self.frames.iter() {

			let image = &assets.images[&sprite.image];

			let file_name = "temp.png";

			let temp_file = File::create(&file_name)?;

			let writer = std::io::BufWriter::new(temp_file);

			let png_writer = image::png::PNGEncoder::new(writer);

			let image: image::ImageBuffer<image::Rgba<_>, _> = image::ImageBuffer::from_raw(
				u32::from(image.width()),
				u32::from(image.height()),
				image.to_rgba8(ctx)?.to_vec(),
			)
			.unwrap();

			// image buffers are flipped in memory for ggez, so we have to unflip them
			let image = image::imageops::flip_vertical(&image);

			png_writer.encode(
				&image,
				image.width(),
				image.height(),
				image::ColorType::RGBA(8),
			)?;

			tar.append_path_with_name(file_name, &sprite.image)?;

			std::fs::remove_file(&file_name)?;
		}
		Ok(())

	}
	pub fn load_tar<P: AsRef<Path>>(
		ctx: &mut Context,
		assets: &mut Assets,
		path: P,
	) -> GameResult<Animation> {
		let file = File::open(path)?;
		let mut tar = tar::Archive::new(file);
		tar.unpack("./temp/")?;

		let file = std::fs::File::open("./temp/data.json")?;
		let buf_read = std::io::BufReader::new(file);
		let animation: Animation = serde_json::from_reader::<_, Animation>(buf_read).unwrap();

		for file in std::fs::read_dir("./temp/")? {
			let entry = file?;
			let path = entry.path();
			if let Some(file_type) = path.extension() {
				if file_type == "png" {
					let file_name = path.file_name().unwrap();
					load_image(file_name.to_str().expect("expected valid utf8 filename").to_owned(), path, ctx, assets)?;
				}
			}

		}

		std::fs::remove_dir_all("./temp/")?;

		Ok(animation)
		/*let files: Vec<_> = tar.entries()?.filter(|item| item.is_ok()).map(|item| item.unwrap()).collect();
		use std::io::Read;
		let temp = &files[0];
		//temp.
		serde_json::from_reader(temp).map_err(|err| ggez::GameError::FilesystemError(format!("{}", err)))*/
		/*for item in tar.entries()? {
			let item = item?;


		}*/
	}


	pub fn draw_frame(
		ctx: &mut Context,
		assets: &Assets,
		animation: &Animation,
		index: usize,
		world: nalgebra::Matrix3<f32>,
	) -> GameResult<()> {
		let data = animation.frames.get(index);
		if let Some((ref image, _)) = data {
			Sprite::draw(ctx, assets, image, world)
		} else {
			Ok(())
		}
	}

	pub fn draw_every_frame(
		ctx: &mut Context,
		assets: &Assets,
		animation: &Animation,
		world: nalgebra::Matrix3<f32>,
	) -> GameResult<()> {
		for sprite in animation.frames.iter().map(|(ref sprite, _)| sprite) {
			Sprite::draw_debug(ctx, assets, sprite, world)?
		}

		Ok(())
	}

	pub fn draw_at_time(
		ctx: &mut Context,
		assets: &Assets,
		animation: &Animation,
		time: usize,
		world: nalgebra::Matrix3<f32>,
	) -> GameResult<()> {
		let image = animation.frames.at_time(time);
		Sprite::draw(ctx, assets, &image, world)
	}

	#[allow(clippy::cognitive_complexity)]
	pub fn draw_ui(
		&mut self,
		ui: &imgui::Ui,
		ctx: &mut Context,
		assets: &mut Assets,
		ui_data: &mut AnimationUi,
	) -> GameResult<()> {
		ui.input_string(im_str!("Name"), &mut self.name);

		ui.text(im_str!("Duration: {}", self.frames.duration()));

		if ui
			.collapsing_header(im_str!("Frames"))
			.default_open(true)
			.build()
		{
			let mut buffer = ui_data
				.current_sprite
				.and_then(|item| i32::try_from(item).ok())
				.unwrap_or(-1);
			ui.list_box_owned(
				im_str!("Frame List"),
				&mut buffer,
				&self
					.frames
					.iter()
					.enumerate()
					.map(|(idx, item)| im_str_owned!("{} ({})", idx, item.0.image.clone()))
					.collect::<Vec<_>>(),
				5,
			);
			ui_data.current_sprite = usize::try_from(buffer).ok();

			if let Some(current_sprite) = ui_data.current_sprite {
				let (up, down) = if current_sprite == 0 {
					let temp = ui.small_button(im_str!("Swap Down"));
					(false, temp)
				} else if current_sprite == self.frames.len() - 1 {
					let temp = ui.small_button(im_str!("Swap Up"));
					(temp, false)
				} else {
					let up = ui.small_button(im_str!("Swap Up"));
					ui.same_line(0.0);
					let down = ui.small_button(im_str!("Swap Down"));
					(up, down)
				};
				if up && current_sprite != 0 {
					self.frames.swap(current_sprite, current_sprite - 1);
					ui_data.current_sprite = Some(current_sprite - 1);
				} else if down && current_sprite != self.frames.len() - 1 {
					self.frames.swap(current_sprite, current_sprite + 1);
					ui_data.current_sprite = Some(current_sprite + 1);
				}

			}
			if ui.small_button(im_str!("Normalize All Names")) {
				for (idx, ref mut sprite) in
					self.frames.iter_mut().map(|item| &mut item.0).enumerate()
				{
					rename_sprite(format!("{}-{:03}.png", self.name, idx), sprite, assets)
				}
			}
			if ui.small_button(im_str!("New")) {
				let result = nfd::open_file_dialog(None, None);
				match result {
					Ok(response) => match response {
						nfd::Response::Cancel => (),
						nfd::Response::Okay(path) => {
							let new_sprite = Sprite::new(path);
							new_sprite.load_image(ctx, assets)?;
							self.frames.push((new_sprite, 1));
						}
						nfd::Response::OkayMultiple(_) => {
							dbg!("no sprite loaded because multiple paths were given");
						}
					},
					Err(err) => {
						dbg!(err);
					}
				}
				ui_data.current_sprite = Some(self.frames.len() - 1);
			}
			ui.same_line(0.0);
			if ui.small_button(im_str!("New Bulk")) {
				let result = nfd::open_file_multiple_dialog(Some("png"), None);
				if let Ok(response) = result {
					match response {
						nfd::Response::Cancel => (),
						nfd::Response::Okay(path) => {
							self.frames.push((Sprite::new(path), 1));
							self.load_images(ctx, assets)?;
						}
						nfd::Response::OkayMultiple(paths) => {
							for path in paths {
								self.frames.push((Sprite::new(path), 1));
							}
							self.load_images(ctx, assets)?;
						}
					}
				}
			}
			if let Some(current_sprite) = ui_data.current_sprite {
				ui.same_line(0.0);
				if ui.small_button(im_str!("Delete")) {
					self.frames.remove(current_sprite);
					if self.frames.is_empty() {
						ui_data.current_sprite = None;
					} else {
						ui_data.current_sprite =
							Some(std::cmp::min(self.frames.len() - 1, current_sprite));
					}
				}
			}
			ui.same_line(0.0);
			if ui.small_button(im_str!("Delete All")) {
				self.frames.clear();
				ui_data.current_sprite = None;
			}

			if let Some(current_sprite) = ui_data.current_sprite {
				ui.separator();

				let (ref mut sprite, ref mut duration) = self.frames[current_sprite];
				let _ = ui.input_whole(im_str!("Duration"), duration);
				ui.separator();

				if ui
					.collapsing_header(im_str!("Offset"))
					.default_open(true)
					.build()
				{
					ui.input_float(im_str!("X"), &mut sprite.offset.x).build();
					ui.input_float(im_str!("Y"), &mut sprite.offset.y).build();
					ui.separator();
				}
				ui.input_float(im_str!("Rotation"), &mut sprite.rotation)
					.build();

				ui.separator();
				if ui
					.collapsing_header(im_str!("Image"))
					.default_open(true)
					.build()
				{
					let mut buffer = sprite.image.clone();
					if ui.input_string(im_str!("Name##Frame"), &mut buffer) {
						rename_sprite(buffer, sprite, assets);
					}

					ui.columns(2, im_str!("Image Buttons"), false);

					if ui.button(
						im_str!("Load New Image"),
						[
							ui.get_content_region_avail().0,
							ui.get_text_line_height_with_spacing(),
						],
					) {
						let result = nfd::open_file_dialog(Some("png"), None);
						match result {
							Ok(result) => match result {
								nfd::Response::Cancel => (),
								nfd::Response::Okay(path) => {
									replace_asset(sprite.image.clone(), &path, ctx, assets)?;
								}
								nfd::Response::OkayMultiple(_) => {
									println!("Cancelling because multiple images were specified.")
								}
							},
							Err(err) => {
								dbg!(err);
							}
						}

					}
					ui.next_column();
					if ui.button(
						im_str!("Normalize Name"),
						[
							ui.get_content_region_avail().0,
							ui.get_text_line_height_with_spacing(),
						],
					) {
						rename_sprite(
							format!("{}-{:03}.png", self.name, current_sprite),
							sprite,
							assets,
						)
					}
				}
			}
		}

		Ok(())
	}

}

fn replace_asset<S: Into<String>>(
	asset: S,
	path: &str,
	ctx: &mut Context,
	assets: &mut Assets,
) -> GameResult<()> {
	load_image(asset, path, ctx, assets)
}
fn rename_sprite<S: Into<String>>(new_name: S, sprite: &mut Sprite, assets: &mut Assets) {
	let asset = assets.images.remove(&sprite.image);
	let new_name = new_name.into();
	if let Some(asset) = asset {
		assets.images.insert(new_name.clone(), asset);
	}
	sprite.image = new_name;
}