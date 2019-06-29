
use ggez::graphics;
use ggez::graphics::{Color, DrawMode, DrawParam, Rect};
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
    rectangle: ggez::graphics::Mesh,
}

impl AnimationEditor {
    pub fn new(ctx: &mut Context, assets: &mut Assets) -> GameResult<Self> {
        let file = std::fs::File::open("./resources/animation.json").unwrap();
        let buf_read = std::io::BufReader::new(file);
        let resource: Animation = serde_json::from_reader::<_, Animation>(buf_read).unwrap();
        resource.load_images(ctx, assets)?;
        let rectangle = graphics::Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(-5.0, -5.0, 10.0, 10.0),
            Color::new(1.0, 0.0, 0.0, 1.0),
        )?;
        Ok(Self {
            frame: 0,
            resource,
            ui_data: AnimationUi::new(),
            rectangle,
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

        let mut ui_actions = Vec::new();

        let dim = (256.0, 256.0);
        let (width, height) = dim;
        let pos = (300.0, 20.0);
        let (x, y) = pos;
        if true {
            imgui.render(ctx, |ui| {
                // Window
                ui.window(im_str!("Editor"))
                    .size((300.0, 465.0), ImGuiCond::Always)
                    .position((0.0, 20.0), ImGuiCond::Always)
                    .resizable(false)
                    .movable(false)
                    .collapsible(false)
                    .build(|| {
                        ui_actions = self.resource.draw_ui(&ui, &mut self.ui_data);
                    });

                if self.resource.frames.duration() > 0 {
                    ui.window(im_str!("Animation"))
                        .size(dim, ImGuiCond::Always)
                        .position(pos, ImGuiCond::Always)
                        .resizable(false)
                        .movable(false)
                        .collapsible(false)
                        .build(|| {});

                    ui.window(im_str!("Current Frame"))
                        .size(dim, ImGuiCond::Always)
                        .position((x + width, y), ImGuiCond::Always)
                        .resizable(false)
                        .movable(false)
                        .collapsible(false)
                        .build(|| {});

                    ui.window(im_str!("Every Frame"))
                        .size(dim, ImGuiCond::Always)
                        .position((x, y + height), ImGuiCond::Always)
                        .resizable(false)
                        .movable(false)
                        .collapsible(false)
                        .build(|| {});
                }
                ui.main_menu_bar(|| {
                    ui.menu(im_str!("File")).build(|| {
                        ui.menu_item(im_str!("New")).build();
                        ui.menu_item(im_str!("Save")).build();
                        ui.menu_item(im_str!("Open")).build();
                    });
                });
            });

        }

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

        let dim = (256.0, 256.0);
        let (width, height) = dim;
        let pos = (300.0, 20.0);
        let (x, y) = pos;

        let padding = 15.0;


        let rectangle2 = graphics::Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(-5.0, -5.0, 10.0, 10.0),
            Color::new(0.0, 1.0, 0.0, 1.0),
        )?;
        if self.resource.frames.duration() > 0 {
            let origin = (x + width / 2.0, y + height - padding);

            graphics::set_transform(
                ctx,
                nalgebra::Translation3::new(0.0, 100.0, 0.0).to_homogeneous(),
            );
            graphics::apply_transformations(ctx)?;
            graphics::draw(
                ctx,
                &rectangle,
                graphics::DrawParam::default().dest([10.0, 10.0]),
            )?;

            graphics::set_transform(
                ctx,
                nalgebra::Translation3::new(100.0, 100.0, 0.0).to_homogeneous(),
            );
            graphics::apply_transformations(ctx)?;
            graphics::draw(ctx, &self.rectangle, graphics::DrawParam::default())?;

            graphics::set_transform(ctx, nalgebra::Matrix4::identity());
            graphics::apply_transformations(ctx)?;
            graphics::apply_transformations(ctx)?;
            Animation::draw_at_time(
                ctx,
                assets,
                &self.resource,
                self.frame % self.resource.frames.duration(),
                nalgebra::Translation3::new(origin.0, origin.1, 0.0).to_homogeneous(),
            )?;

        }


        Ok(())

    }
}