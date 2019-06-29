use super::sprite::{load_image, Sprite};
use crate::assets::Assets;
use crate::timeline::{AtTime, Timeline};

use crate::imgui_extra::UiExtensions;

use ggez::{Context, GameResult};
use imgui::{im_str, ImString};
use serde::{Deserialize, Serialize};

use std::convert::TryFrom;


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Animation {
	pub name: String,
	pub frames: Timeline<Option<Sprite>>,
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
			if let Some(data) = sprite {
				data.load_image(ctx, assets)?
			}
		}
		Ok(())
	}
	pub fn draw_frame(
		ctx: &mut Context,
		assets: &Assets,
		animation: &Animation,
		index: usize,
		world: nalgebra::Matrix3<f32>,
	) -> GameResult<()> {
		let data = animation.frames.get(index);
		if let Some((Some(ref image), _)) = data {
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
		for sprite in animation
			.frames
			.iter()
			.filter(|(item, _)| item.is_some())
			.map(|(ref item, _)| item.as_ref().unwrap())
		{
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
		let data = animation.frames.at_time(time);
		if let Some(image) = data {
			Sprite::draw(ctx, assets, &image, world)
		} else {
			Ok(())
		}
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
					.map(|(idx, item)| {
						im_str_owned!(
							"{} ({})",
							idx,
							item.0
								.as_ref()
								.map(|ref item| item.image.clone())
								.unwrap_or_else(|| "none".to_owned())
						)
					})
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

				if ui.small_button(im_str!("Normalize All Names")) {
					for (idx, ref mut sprite) in self
						.frames
						.iter_mut()
						.map(|item| &mut item.0)
						.enumerate()
						.filter(|(_, item)| item.is_some())
						.map(|(idx, item)| (idx, item.as_mut().unwrap()))
					{
						rename_sprite(format!("/{}/{:03}.png", self.name, idx), sprite, assets)
					}
				}
			}


			if ui.small_button(im_str!("New")) {
				self.frames.push((None, 1));
				ui_data.current_sprite = Some(self.frames.len() - 1);
			}
			ui.same_line(0.0);
			if ui.small_button(im_str!("New Bulk")) {
				let result = nfd::open_file_multiple_dialog(Some("png"), None);
				if let Ok(response) = result {
					match response {
						nfd::Response::Cancel => (),
						nfd::Response::Okay(path) => {
							self.frames.push((Some(Sprite::new(path)), 1));
							self.load_images(ctx, assets)?;
						}
						nfd::Response::OkayMultiple(paths) => {
							for path in paths {
								self.frames.push((Some(Sprite::new(path)), 1));
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

				let mut pick = if sprite.is_none() { 0 } else { 1 };
				let old_pick = pick;
				if ui.combo(
					im_str!("Type"),
					&mut pick,
					&[im_str!("None"), im_str!("Sprite")],
					2,
				) && old_pick != pick
				{
					match pick {
						0 => *sprite = None,
						1 => {
							*sprite = {
								let result = nfd::open_file_dialog(None, None);
								match result {
									Ok(response) => match response {
										nfd::Response::Cancel => None,
										nfd::Response::Okay(path) => {
											let new_sprite = Sprite::new(path);
											new_sprite.load_image(ctx, assets)?;
											Some(new_sprite)
										}
										nfd::Response::OkayMultiple(_) => {
											dbg!("no sprite loaded because multiple paths were given");
											None
										}
									},
									Err(err) => {
										dbg!(err);
										None
									}

								}
							}
						}
						_ => unreachable!(),
					}
				};
				if let Some(ref mut sprite) = sprite {
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
									nfd::Response::OkayMultiple(_) => println!(
										"Cancelling because multiple images were specified."
									),
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
								format!("/{}/{:03}.png", self.name, current_sprite),
								sprite,
								assets,
							)
						}
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