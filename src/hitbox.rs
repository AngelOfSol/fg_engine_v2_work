use std::marker::PhantomData;

use crate::game_match::PlayArea;
use crate::imgui_extra::UiExtensions;
use crate::input::Facing;
use crate::typedefs::collision::{Int, IntoGraphical, Vec2};
use crate::typedefs::graphics::{Matrix4, Vec2 as GraphicVec2};
use ggez::graphics;
use ggez::graphics::{BlendMode, Color, DrawMode, DrawParam, FillOptions, Mesh, Rect};
use ggez::{Context, GameResult};
use imgui::*;
use inspect_design::Inspect;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Deserialize, PartialEq, Eq, Serialize, Inspect)]
pub struct GenericHitbox<T> {
    pub center: Vec2,
    pub half_size: Vec2,
    #[serde(skip)]
    _secret: std::marker::PhantomData<T>,
}
impl<T> Default for GenericHitbox<T> {
    fn default() -> Self {
        Self {
            center: Vec2::new(0_00, 0_00),
            half_size: Vec2::new(1_00, 1_00),
            _secret: PhantomData,
        }
    }
}

#[derive(Debug, Copy, Clone, Deserialize, PartialEq, Eq, Serialize, Inspect, Default)]
pub struct Relative;
#[derive(Debug, Copy, Clone, Deserialize, PartialEq, Eq, Serialize, Inspect, Default)]
pub struct Absolute;

pub type Hitbox = GenericHitbox<Relative>;
pub type PositionedHitbox = GenericHitbox<Absolute>;

impl Hitbox {
    pub fn with_collision_position(&self, position: Vec2) -> PositionedHitbox {
        PositionedHitbox {
            center: position,
            half_size: self.half_size,
            _secret: std::marker::PhantomData,
        }
    }
    pub fn with_position_and_facing(&self, position: Vec2, facing: Facing) -> PositionedHitbox {
        PositionedHitbox {
            center: facing.fix_collision(self.center) + position,
            half_size: self.half_size,
            _secret: std::marker::PhantomData,
        }
    }
}

impl PositionedHitbox {
    pub fn overlaps(self, target: Self) -> bool {
        let distance = (self.center - target.center).abs();
        let allowed_distance = self.half_size + target.half_size;
        distance.x < allowed_distance.x && distance.y < allowed_distance.y
    }
    pub fn overlaps_any(lhs: &[Self], rhs: &[Self]) -> bool {
        lhs.iter().any(|left_hitbox| {
            rhs.iter()
                .copied()
                .any(|right_hitbox| left_hitbox.overlaps(right_hitbox))
        })
    }

    pub fn fix_distances(
        self,
        target: Self,
        play_area: &PlayArea,
        vels: (Int, Int),
        facing: Facing,
    ) -> (Int, Int) {
        let distance = (self.center - target.center).abs();
        let allowed_distance = self.half_size + target.half_size;

        let overlap = allowed_distance - distance;
        let overlap_distance = overlap.x / 2;

        let direction_mod = if self.center.x < target.center.x {
            -1
        } else if self.center.x > target.center.x {
            1
        } else if self.center.x - vels.0 < target.center.x - vels.1 {
            -1
        } else if self.center.x - vels.0 > target.center.x - vels.1 {
            1
        } else if vels.0 + vels.1 != 0 {
            Int::signum(vels.0 + vels.1)
        } else {
            facing.invert().collision_multiplier().x
        };

        let (left_mod, right_mod) = (
            direction_mod * overlap_distance,
            -direction_mod * (overlap.x - overlap_distance),
        );

        let self_mod =
            if Int::abs(self.center.x + left_mod) > play_area.width / 2 - self.half_size.x {
                Int::signum(self.center.x + left_mod) * (play_area.width / 2 - self.half_size.x)
                    - (self.center.x + left_mod)
            } else {
                0
            };
        let target_mod = if Int::abs(target.center.x + right_mod)
            > play_area.width / 2 - target.half_size.x
        {
            Int::signum(target.center.x + right_mod) * (play_area.width / 2 - target.half_size.x)
                - (target.center.x + right_mod)
        } else {
            0
        };

        (
            left_mod + target_mod + self_mod,
            right_mod + target_mod + self_mod,
        )
    }
}

impl<T> GenericHitbox<T> {
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
        ui.input_vec2_pixels(im_str!("Center"), &mut data.center);
        ui.input_vec2_pixels(im_str!("Half Size"), &mut data.half_size);
        data.half_size.x = std::cmp::max(data.half_size.x, 1_00);
        data.half_size.y = std::cmp::max(data.half_size.y, 1_00);
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
