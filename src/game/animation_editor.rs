use ggez::{Context, GameResult};


use crate::animation::{Animation, AnimationUi};

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
        Animation::draw(
            ctx,
            assets,
            &self.resource,
            self.frame % self.resource.frames.duration(),
            nalgebra::Translation3::new(100.0, 100.0, 0.0).to_homogeneous(),
        )?;
        imgui.render(ctx, |ui| {
            // Window
            ui.window(im_str!("Animation"))
                .scrollable(true)
                .size((300.0, 200.0), ImGuiCond::FirstUseEver)
                .build(|| {
                    self.resource.draw_ui(&ui, &mut self.ui_data);
                });
        });

        self.resource.load_images(ctx, assets)?;


        Ok(())

    }
}