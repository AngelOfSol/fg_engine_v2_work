
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
	#[serde(default = "val")]
	pub name: String,
	pub frames: Timeline<Option<Sprite>>,
}

fn val() -> String {
	"a".to_owned()
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

	pub fn draw_ui(&mut self, ui: &imgui::Ui, ui_data: &mut AnimationUi) {

		let mut buffer = im_str_owned!("{}", self.name.clone());
		buffer.reserve_exact(16);
		ui.input_text(im_str!("Name"), &mut buffer).build();
		self.name = buffer.to_str().to_owned();

		ui.text(im_str!("Duration: {}", self.frames.duration()));

		if ui.collapsing_header(im_str!("Frames")).build() {
			let mut buffer = i32::try_from(ui_data.current_sprite).unwrap();
			ui.list_box_owned(
				im_str!("Frame List"),
				&mut buffer,
				&self
					.frames
					.iter()
					.enumerate()
					.map(|(idx, _)| im_str_owned!("{}", idx))
					.collect::<Vec<_>>(),
				5,
			);
			ui_data.current_sprite = usize::try_from(buffer).unwrap();

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
										Some(Sprite::new(path))
									},
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
							// let sprite = Sprite::new("test");
							// sprite.load_image(ctx);
							//dbg!(test.err().unwrap());
							//Some(Sprite::new())
						}
					}
					_ => unreachable!(),
				}
			};

		}

	}

}