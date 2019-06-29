
use ggez::graphics;
use ggez::graphics::Color;
use ggez::{Context, GameResult};

use crate::animation::{Animation, AnimationUi};

use crate::assets::Assets;
use crate::imgui_wrapper::ImGuiWrapper;
use crate::timeline::AtTime;


use imgui::*;

pub struct AnimationEditor {
    frame: usize,
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

        let dim = (256.0, 256.0);
        let (width, height) = dim;
        let pos = (300.0, 20.0);
        let (x, y) = pos;

        let mut editor_result = Ok(());
        let imgui_render = imgui.frame(ctx).run(|ui| {
            // Window
            ui.window(im_str!("Editor"))
                .size((300.0, 465.0), ImGuiCond::Always)
                .position((0.0, 20.0), ImGuiCond::Always)
                .resizable(false)
                .movable(false)
                .collapsible(false)
                .build(|| {
                    editor_result = self.resource.draw_ui(&ui, ctx, assets, &mut self.ui_data);
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
        imgui_render.render(ctx);
        editor_result?;


        let vertical = graphics::Mesh::new_line(
            ctx,
            &[[0.0, -10.0], [0.0, 10.0]],
            1.0,
            Color::new(0.0, 1.0, 0.0, 1.0),
        )?;

        let horizontal = graphics::Mesh::new_line(
            ctx,
            &[[-10.0, 0.0], [10.0, 0.0]],
            1.0,
            Color::new(0.0, 1.0, 0.0, 1.0),
        )?;


        let dim = (256.0, 256.0);
        let (width, height) = dim;

        let padding = 15.0;

        let draw_cross = |ctx: &mut Context, origin: (f32, f32)| {
            graphics::draw(
                ctx,
                &vertical,
                graphics::DrawParam::default().dest([origin.0, origin.1]),
            )?;
            graphics::draw(
                ctx,
                &horizontal,
                graphics::DrawParam::default().dest([origin.0, origin.1]),
            )
        };

        if self.resource.frames.duration() > 0 {
            {
                // normal animation
                let pos = (300.0, 20.0);
                let (x, y) = pos;
                let origin = (x + width / 2.0, y + height - padding);

                Animation::draw_at_time(
                    ctx,
                    assets,
                    &self.resource,
                    self.frame % self.resource.frames.duration(),
                    nalgebra::Translation2::new(origin.0, origin.1).to_homogeneous(),
                )?;
                draw_cross(ctx, origin)?;
            }

            {
                // current_frame
                let pos = (300.0, 20.0 + height);
                let (x, y) = pos;
                let origin = (x + width / 2.0, y + height - padding);

                Animation::draw_every_frame(
                    ctx,
                    assets,
                    &self.resource,
                    nalgebra::Translation2::new(origin.0, origin.1).to_homogeneous(),
                )?;
            }

            if let Some(frame) = self.ui_data.current_sprite {
                {
                    // current_frame
                    let pos = (300.0 + width, 20.0);
                    let (x, y) = pos;
                    let origin = (x + width / 2.0, y + height - padding);
                    Animation::draw_frame(
                        ctx,
                        assets,
                        &self.resource,
                        frame,
                        nalgebra::Translation2::new(origin.0, origin.1).to_homogeneous(),
                    )?;
                    draw_cross(ctx, origin)?;
                }
                {
                    // current_frame
                    let pos = (300.0, 20.0 + height);
                    let (x, y) = pos;
                    let origin = (x + width / 2.0, y + height - padding);

                    Animation::draw_frame(
                        ctx,
                        assets,
                        &self.resource,
                        frame,
                        nalgebra::Translation2::new(origin.0, origin.1).to_homogeneous(),
                    )?;
                    draw_cross(ctx, origin)?;
                }
            }

        }


        Ok(())

    }
}