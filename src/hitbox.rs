use crate::imgui_extra::UiExtensions;
use crate::typedefs::collision::{Int, IntoGraphical, Vec2};
use crate::typedefs::graphics::{Matrix4, Vec2 as GraphicVec2};
use ggez::graphics;
use ggez::graphics::{BlendMode, Color, DrawMode, DrawParam, FillOptions, Mesh, Rect};
use ggez::{Context, GameResult};
use imgui::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct Hitbox {
    pub center: Vec2,
    pub half_size: Vec2,
}

impl Hitbox {
    pub fn new() -> Self {
        Self {
            center: Vec2::zeros(),
            half_size: Vec2::new(1_00, 1_00),
        }
    }
    pub fn with_half_size(half_size: Vec2) -> Self {
        Self {
            center: Vec2::zeros(),
            half_size,
        }
    }

    pub fn size(&self) -> Vec2 {
        self.half_size * 2
    }

    pub fn width(&self) -> Int {
        self.size().x
    }
    pub fn height(&self) -> Int {
        self.size().y
    }

    pub fn collision_graphic_recenter(&self) -> GraphicVec2 {
        GraphicVec2::new(
            self.center.x.into_graphical(),
            self.half_size.y.into_graphical() - self.center.y.into_graphical(),
        )
    }

    pub fn draw_ui(ui: &Ui<'_>, data: &mut Hitbox) {
        data.center /= 100;
        ui.input_vec2_int(im_str!("Center"), &mut data.center);
        data.center *= 100;
        data.half_size /= 100;
        ui.input_vec2_int(im_str!("Half Size"), &mut data.half_size);
        data.half_size.x = std::cmp::max(data.half_size.x, 1);
        data.half_size.y = std::cmp::max(data.half_size.y, 1);
        data.half_size *= 100;
    }

    pub fn draw(&self, ctx: &mut Context, world: Matrix4, color: Color) -> GameResult<()> {
        graphics::set_blend_mode(ctx, BlendMode::Alpha)?;
        let rect = Mesh::new_rectangle(
            ctx,
            DrawMode::Fill(FillOptions::default()),
            Rect::new(
                -self.half_size.x.into_graphical() + self.center.x.into_graphical(),
                // graphically -y is up/+y is down, but for the purpores of math, our center has +y as up/-y as down
                -self.half_size.y.into_graphical() - self.center.y.into_graphical(),
                self.width().into_graphical(),
                self.height().into_graphical(),
            ),
            color,
        )?;

        graphics::set_transform(ctx, world);
        graphics::apply_transformations(ctx)?;
        graphics::draw(ctx, &rect, DrawParam::default())?;

        Ok(())
    }
}
