use ggez::{Context, GameResult};


use crate::animation::{load_image, Animation, AnimationUi, UiAction};

use crate::assets::Assets;
use crate::imgui_wrapper::ImGuiWrapper;
use crate::timeline::AtTime;


use imgui::*;

pub struct AnimationEditor {
    pub frame: usize,
    resource: Animation,
    ui_data: AnimationUi,
}

impl AnimationEditor {
    pub fn new(ctx: &mut Context, assets: &mut Assets) -> GameResult<Self> {
        let file = std::fs::File::open("./resources/animation.json").unwrap();
        let buf_read = std::io::BufReader::new(file);
        let resource: Animation = serde_json::from_reader::<_, Animation>(buf_read).unwrap();
        resource.load_images(ctx, assets)?;
        Ok(Self {
            frame: 0,
            resource,
            ui_data: AnimationUi::new(),
        })
    }

    pub fn update(&mut self) -> GameResult<()> {
        self.frame += 1;
        Ok(())
    }

    pub fn draw<'a>(
        &mut self,
        ctx: &mut Context,
        assets: &mut Assets,
        imgui: &'a mut ImGuiWrapper,
    ) -> GameResult<()> {
        if self.resource.frames.duration() > 0 {
            Animation::draw(
                ctx,
                assets,
                &self.resource,
                self.frame % self.resource.frames.duration(),
                nalgebra::Translation3::new(100.0, 100.0, 0.0).to_homogeneous(),
            )?;
        }
        let mut ui_actions = Vec::new();

        imgui.render(ctx, |ui| {
            // Window
            ui.window(im_str!("Animation"))
                .size((300.0, 465.0), ImGuiCond::Always)
                .position((0.0, 0.0), ImGuiCond::Always)
                .resizable(false)
                .movable(false)
                .collapsible(false)
                .build(|| {
                    ui_actions = self.resource.draw_ui(&ui, &mut self.ui_data);
                });
        });

        for item in ui_actions {
            match item {
                UiAction::ReloadAssets => {
                    self.resource.load_images(ctx, assets)?;
                }

                UiAction::RenameAsset { from, to } => {
                    let asset = assets.images.remove(&from);
                    if let Some(asset) = asset {
                        assets.images.insert(to, asset);
                    }
                }
                UiAction::ReplaceAsset { asset, path } => {
                    load_image(asset, &path, ctx, assets)?;
                }
            }
        }


        Ok(())

    }
}