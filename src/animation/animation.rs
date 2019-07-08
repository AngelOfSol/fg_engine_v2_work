use super::sprite::{rename_sprite, Sprite, SpriteUi};

use crate::assets::Assets;
use crate::imgui_extra::UiExtensions;
use crate::timeline::{AtTime, Timeline};
use crate::typedefs::graphics::Matrix4;

use imgui::im_str;

use ggez::graphics;
use ggez::{Context, GameError, GameResult};

use nfd::Response;

use serde::{Deserialize, Serialize};

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum BlendMode {
    Alpha,
    Add,
}

impl Into<graphics::BlendMode> for BlendMode {
    fn into(self) -> graphics::BlendMode {
        match self {
            BlendMode::Add => graphics::BlendMode::Add,
            BlendMode::Alpha => graphics::BlendMode::Alpha,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Animation {
    pub name: String,
    pub frames: Timeline<Sprite>,
    #[serde(default = "default_blend_mode")]
    pub blend_mode: BlendMode,
}

fn default_blend_mode() -> BlendMode {
    BlendMode::Alpha
}

impl Animation {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            frames: Timeline::new(),
            blend_mode: BlendMode::Alpha,
        }
    }
    pub fn load_from_json(
        ctx: &mut Context,
        assets: &mut Assets,
        mut path: PathBuf,
    ) -> GameResult<Animation> {
        let file = File::open(&path).unwrap();
        let buf_read = BufReader::new(file);
        let animation = serde_json::from_reader::<_, Animation>(buf_read).unwrap();
        path.pop();
        Animation::load(ctx, assets, &animation, path)?;
        Ok(animation)
    }
    pub fn load(
        ctx: &mut Context,
        assets: &mut Assets,
        animation: &Animation,
        mut path: PathBuf,
    ) -> GameResult<()> {
        path.push(&animation.name);
        for (sprite, _) in &animation.frames {
            Sprite::load(ctx, assets, sprite, path.clone())?;
        }
        Ok(())
    }

    pub fn save(
        ctx: &mut Context,
        assets: &mut Assets,
        animation: &Animation,
        mut path: PathBuf,
    ) -> GameResult<()> {
        path.push(format!("{}.json", &animation.name));
        dbg!(&path);
        let mut json = File::create(&path)?;
        serde_json::to_writer(&mut json, &animation)
            .map_err(|err| GameError::FilesystemError(format!("{}", err)))?;
        path.pop();
        path.push(&animation.name);
        dbg!(&path);
        std::fs::create_dir_all(&path)?;
        for (sprite, _) in &animation.frames {
            Sprite::save(ctx, assets, sprite, path.clone())?;
        }
        Ok(())
    }

    pub fn load_images(&self, ctx: &mut Context, assets: &mut Assets) -> GameResult<()> {
        for (sprite, _) in &self.frames {
            sprite.load_image(ctx, assets)?
        }
        Ok(())
    }

    pub fn draw_frame(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        index: usize,
        world: Matrix4,
    ) -> GameResult<()> {
        let data = self.frames.get(index);
        if let Some((ref sprite, _)) = data {
            graphics::set_blend_mode(ctx, self.blend_mode.into())?;
            sprite.draw(ctx, assets, world)
        } else {
            Ok(())
        }
    }

    pub fn draw_every_frame(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        world: Matrix4,
    ) -> GameResult<()> {
        graphics::set_blend_mode(ctx, self.blend_mode.into())?;
        for sprite in self.frames.iter().map(|(ref sprite, _)| sprite) {
            sprite.draw_debug(ctx, assets, world)?
        }

        Ok(())
    }

    pub fn draw_at_time(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        time: usize,
        world: Matrix4,
    ) -> GameResult<()> {
        graphics::set_blend_mode(ctx, self.blend_mode.into())?;
        let image = self.frames.at_time(time);
        image.draw(ctx, assets, world)
    }
    pub fn draw_at_time_debug(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        time: usize,
        world: Matrix4,
    ) -> GameResult<()> {
        graphics::set_blend_mode(ctx, self.blend_mode.into())?;
        let image = self.frames.at_time(time);
        image.draw_debug(ctx, assets, world)
    }
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

    pub fn draw_ui(
        &mut self,
        ui: &imgui::Ui,
        ctx: &mut Context,
        assets: &mut Assets,
        animation: &mut Animation,
    ) -> GameResult<()> {
        ui.input_string(im_str!("Name"), &mut animation.name);

        ui.label_text(
            im_str!("Duration"),
            &im_str!("{}", animation.frames.duration()),
        );

        if ui
            .collapsing_header(im_str!("Blend Mode"))
            .default_open(true)
            .build()
        {
            ui.radio_button(
                im_str!("Alpha"),
                &mut animation.blend_mode,
                BlendMode::Alpha,
            );
            ui.radio_button(im_str!("Add"), &mut animation.blend_mode, BlendMode::Add);
        }

        if ui
            .collapsing_header(im_str!("Frames"))
            .default_open(true)
            .build()
        {
            ui.rearrangable_list_box(
                im_str!("Frame List"),
                &mut self.current_sprite,
                &mut animation.frames,
                |item| im_str!("{}", item.0.image.clone()),
                5,
            );

            if ui.small_button(im_str!("Normalize All Names")) {
                for (idx, ref mut sprite) in animation
                    .frames
                    .iter_mut()
                    .map(|item| &mut item.0)
                    .enumerate()
                {
                    rename_sprite(format!("{}-{:03}.png", animation.name, idx), sprite, assets)
                }
            }
            if ui.small_button(im_str!("New")) {
                let result = nfd::open_file_dialog(None, None);
                match result {
                    Ok(response) => match response {
                        Response::Cancel => (),
                        Response::Okay(path) => {
                            let new_sprite = Sprite::new(path);
                            new_sprite.load_image(ctx, assets)?;
                            animation.frames.push((new_sprite, 1));
                            self.current_sprite = Some(animation.frames.len() - 1);
                        }
                        Response::OkayMultiple(_) => {
                            dbg!("no sprite loaded because multiple paths were given");
                        }
                    },
                    Err(err) => {
                        dbg!(err);
                    }
                }
            }
            ui.same_line(0.0);
            if ui.small_button(im_str!("New Bulk")) {
                let result = nfd::open_file_multiple_dialog(Some("png"), None);
                if let Ok(response) = result {
                    match response {
                        Response::Cancel => (),
                        Response::Okay(path) => {
                            animation.frames.push((Sprite::new(path), 1));
                            animation.load_images(ctx, assets)?;
                        }
                        Response::OkayMultiple(paths) => {
                            for path in paths {
                                animation.frames.push((Sprite::new(path), 1));
                            }
                            animation.load_images(ctx, assets)?;
                        }
                    }
                }
            }
            if let Some(current_sprite) = self.current_sprite {
                ui.same_line(0.0);
                if ui.small_button(im_str!("Delete")) {
                    animation.frames.remove(current_sprite);
                    if animation.frames.is_empty() {
                        self.current_sprite = None;
                    } else {
                        self.current_sprite =
                            Some(std::cmp::min(animation.frames.len() - 1, current_sprite));
                    }
                }
            }
            ui.same_line(0.0);
            if ui.small_button(im_str!("Delete All")) {
                animation.frames.clear();
                self.current_sprite = None;
            }

            if let Some(current_sprite) = self.current_sprite {
                ui.separator();

                let (ref mut sprite, ref mut duration) = animation.frames[current_sprite];
                let _ = ui.input_whole(im_str!("Duration"), duration);
                *duration = std::cmp::max(1, *duration);
                ui.separator();

                SpriteUi::new().draw_ui(ctx, assets, ui, sprite)?;
            }
        }

        Ok(())
    }
}
