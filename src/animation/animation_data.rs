
use super::sprite::Sprite;
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


pub enum UiAction {
	ReloadAssets,
	RenameAsset { from: String, to: String },
	ReplaceAsset { asset: String, path: String },
}

pub struct AnimationUi {
	current_sprite: usize,
}

impl AnimationUi {
	pub fn new() -> Self {
		Self { current_sprite: 0 }
	}
}

impl Animation {
	pub fn new<N: Into<String>>(name: N) -> Self {
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
	pub fn draw(
		ctx: &mut Context,
		assets: &Assets,
		animation: &Animation,
		time: usize,
		world: nalgebra::Matrix4<f32>,
	) -> GameResult<()> {
		let data = animation.frames.at_time(time);
		if let Some(image) = data {
			Sprite::draw(ctx, assets, &image, world)
		} else {
			Ok(())
		}
	}

	pub fn draw_ui(&mut self, ui: &imgui::Ui, ui_data: &mut AnimationUi) -> Vec<UiAction> {
		let mut actions = vec![];

		let mut buffer = im_str_owned!("{}", self.name.clone());
		buffer.reserve_exact(16);
		if ui.input_text(im_str!("Name"), &mut buffer).build() {
			self.name = buffer.to_str().to_owned();
		}

		ui.text(im_str!("Duration: {}", self.frames.duration()));

		if ui
			.collapsing_header(im_str!("Frames"))
			.default_open(true)
			.build()
		{
			let mut buffer = i32::try_from(ui_data.current_sprite).unwrap();
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
			ui_data.current_sprite = usize::try_from(buffer).unwrap();

			let (up, down) = if ui_data.current_sprite == 0 {
				let temp = ui.small_button(im_str!("Swap Down"));
				(false, temp)
			} else if ui_data.current_sprite == self.frames.len() - 1 {
				let temp = ui.small_button(im_str!("Swap Up"));
				(temp, false)
			} else {
				let up = ui.small_button(im_str!("Swap Up"));
				ui.same_line(0.0);
				let down = ui.small_button(im_str!("Swap Down"));
				(up, down)
			};
			if ui.small_button(im_str!("Normalize All Names")) {
				for (idx, ref mut sprite) in self
					.frames
					.iter_mut()
					.map(|item| &mut item.0)
					.enumerate()
					.filter(|(_, item)| item.is_some())
					.map(|(idx, item)| (idx, item.as_mut().unwrap()))
				{
					actions.push(normalize_sprite_name(
						format!("/{}/{:03}.png", self.name, idx),
						sprite,
					));
				}
			}

			if ui.small_button(im_str!("New")) {}
			ui.same_line(0.0);
			if ui.small_button(im_str!("Delete")) {}


			if up && ui_data.current_sprite != 0 {
				self.frames
					.swap(ui_data.current_sprite, ui_data.current_sprite - 1);
				ui_data.current_sprite -= 1;
			} else if down && ui_data.current_sprite != self.frames.len() - 1 {
				self.frames
					.swap(ui_data.current_sprite, ui_data.current_sprite + 1);
				ui_data.current_sprite += 1;
			}

			ui.separator();
			let (ref mut current_sprite, ref mut duration) = self.frames[ui_data.current_sprite];
			let mut buffer =
				i32::try_from(*duration).expect("expected duration to be within i32 range");
			ui.slider_int(im_str!("Duration"), &mut buffer, 1, 16)
				.build();
			*duration = usize::try_from(buffer).unwrap_or(1);
			ui.separator();

			let mut pick = if current_sprite.is_none() { 0 } else { 1 };
			let old_pick = pick;
			if ui.combo(
				im_str!("Type"),
				&mut pick,
				&[im_str!("None"), im_str!("Sprite")],
				2,
			) && old_pick != pick
			{
				match pick {
					0 => *current_sprite = None,
					1 => {
						*current_sprite = {
							let result = nfd::open_file_dialog(None, None);
							match result {
								Ok(response) => match response {
									nfd::Response::Cancel => None,
									nfd::Response::Okay(path) => {
										actions.push(UiAction::ReloadAssets);
										Some(Sprite::new(path))
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
			if let Some(ref mut current_sprite) = current_sprite {
				if ui
					.collapsing_header(im_str!("Offset"))
					.default_open(true)
					.build()
				{
					ui.input_float(im_str!("X"), &mut current_sprite.offset.x)
						.build();
					ui.input_float(im_str!("Y"), &mut current_sprite.offset.y)
						.build();
					ui.separator();
				}
				ui.input_float(im_str!("Rotation"), &mut current_sprite.rotation)
					.build();
				ui.separator();
				if ui
					.collapsing_header(im_str!("Image"))
					.default_open(true)
					.build()
				{
					let mut buffer = im_str_owned!("{}", current_sprite.image.clone());
					buffer.reserve_exact(16);
					if ui.input_text(im_str!("Name##Frame"), &mut buffer).build() {
						actions.push(UiAction::RenameAsset {
							from: current_sprite.image.clone(),
							to: buffer.to_str().to_owned(),
						});
						current_sprite.image = buffer.to_str().to_owned();
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
								nfd::Response::Okay(path) => actions.push(UiAction::ReplaceAsset {
									asset: current_sprite.image.clone(),
									path,
								}),
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
						actions.push(normalize_sprite_name(
							format!("/{}/{:03}.png", self.name, ui_data.current_sprite),
							current_sprite,
						));
					}
				}

			}
		}

		actions

	}

}

fn normalize_sprite_name<S: Into<String>>(new_name: S, sprite: &mut Sprite) -> UiAction {
	let new_name = new_name.into();
	let ret = UiAction::RenameAsset {
		from: sprite.image.clone(),
		to: new_name.clone(),
	};

	sprite.image = new_name;

	ret

}