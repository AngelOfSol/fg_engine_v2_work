use super::{
    typedefs::{Character, Timed},
    Player,
};
use crate::{
    assets::{Assets, UiProgress},
    character::state::components::{GlobalGraphic, GlobalGraphicMap},
    game_match::UiElements,
    game_object::state::{Position, Timer},
    input::Facing,
    roster::hit_info::ComboEffect,
    typedefs::{
        collision::{self, IntoGraphical},
        graphics,
    },
};
use ggez::{Context, GameResult};

pub struct UiContext<'a> {
    pub ui: &'a UiElements,
    pub bottom_line: graphics::Matrix4,
    pub flipped: bool,
    pub wins: usize,
    pub first_to: usize,
    pub last_combo_state: &'a Option<Timed<ComboEffect>>,
}

pub fn get_transform(
    world: graphics::Matrix4,
    offset: collision::Vec2,
    position: collision::Vec2,
    facing: Facing,
) -> graphics::Matrix4 {
    world
        * graphics::Matrix4::new_translation(&graphics::up_dimension(position.into_graphical()))
        * graphics::Matrix4::new_translation(&graphics::up_dimension(
            facing.fix_graphics(-offset.into_graphical()),
        ))
        * graphics::Matrix4::new_nonuniform_scaling(&graphics::up_dimension(
            facing.graphics_multiplier(),
        ))
}

impl<C: Character> Player<C> {
    pub fn draw(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        world: graphics::Matrix4,
    ) -> GameResult<()> {
        let Timed {
            time: frame,
            id: move_id,
        } = &self.state.current_state;

        let collision = &self.data.get(&self.state).hitboxes.collision;

        let graphic = self.data.state_graphics_map[&move_id];

        self.data.graphics[&graphic].draw_at_time(
            ctx,
            assets,
            *frame,
            get_transform(
                world,
                collision.center,
                self.state.position,
                self.state.facing,
            ),
        )
    }
    pub fn draw_shadow(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        world: graphics::Matrix4,
    ) -> GameResult<()> {
        let Timed {
            time: frame,
            id: move_id,
        } = &self.state.current_state;

        let collision = &self.data.get(&self.state).hitboxes.collision;

        let graphic = self.data.state_graphics_map[&move_id];

        self.data.graphics[&graphic].draw_shadow_at_time(
            ctx,
            assets,
            *frame,
            get_transform(
                world,
                collision.center,
                self.state.position,
                self.state.facing,
            ),
        )
    }
    pub fn draw_objects(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        world: graphics::Matrix4,
        global_graphics: &GlobalGraphicMap,
    ) -> GameResult<()> {
        for (_, (position, graphic, Timer(frame))) in self
            .world
            .query::<(&Position, &C::Graphic, &Timer)>()
            .iter()
        {
            self.data.graphics[graphic].draw_at_time(
                ctx,
                assets,
                *frame % self.data.graphics[graphic].duration(),
                world
                    * graphics::Matrix4::new_translation(&graphics::up_dimension(
                        position.value.into_graphical(),
                    )),
            )?;
        }
        for (_, (position, graphic, Timer(frame))) in self
            .world
            .query::<(&Position, &GlobalGraphic, &Timer)>()
            .iter()
        {
            global_graphics[graphic].draw_at_time(
                ctx,
                assets,
                *frame % global_graphics[graphic].duration(),
                world
                    * graphics::Matrix4::new_translation(&graphics::up_dimension(
                        position.value.into_graphical(),
                    )),
            )?;
        }

        Ok(())
    }

    pub fn draw_ui(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        UiContext {
            ui,
            bottom_line,
            flipped,
            wins,
            first_to,
            last_combo_state,
        }: UiContext<'_>,
    ) -> GameResult<()> {
        if let Some(Timed { id: combo, time }) = last_combo_state {
            let text = format!(
                "{} hits\n{} damage\n{} limit",
                combo.hits,
                combo.total_damage,
                combo.available_limit.max(0),
            );

            let mut combo_text = self.ui_state.combo_text.borrow_mut();

            if let Some(combo_text) = combo_text.as_mut() {
                if combo_text.fragments()[0].text != text {
                    combo_text.fragments_mut()[0] = ggez::graphics::TextFragment::new(text);
                }
            } else {
                let mut temp = ggez::graphics::Text::new(text);
                temp.set_font(ui.font, ggez::graphics::Scale::uniform(30.0));
                temp.set_bounds(
                    [400.0, 400.0],
                    if flipped {
                        ggez::graphics::Align::Right
                    } else {
                        ggez::graphics::Align::Left
                    },
                );
                *combo_text = Some(temp);
            }

            let hits_text = combo_text.as_ref().unwrap();

            let width = 400.0;

            ggez::graphics::set_transform(
                ctx,
                graphics::Matrix4::new_translation(&graphics::Vec3::new(
                    if flipped { -width + 290.0 } else { -290.0 },
                    -150.0,
                    0.0,
                )) * bottom_line
                    * graphics::Matrix4::new_nonuniform_scaling(&graphics::Vec3::new(
                        if flipped { -1.0 } else { 1.0 },
                        1.0,
                        1.0,
                    )),
            );
            ggez::graphics::apply_transformations(ctx)?;

            ggez::graphics::draw(
                ctx,
                hits_text,
                ggez::graphics::DrawParam::default().color(ggez::graphics::Color::new(
                    1.0,
                    1.0,
                    1.0,
                    *time as f32 / 30.0,
                )),
            )?;

            let _lock = ggez::graphics::use_shader(ctx, &assets.ui_shader);

            // TODO replace with constant, (LIMIT_BAR_PIXEL_SIZE)
            let limit_ratio = combo.available_limit.max(0) as f32 / 200.0;

            assets.ui_shader.send(
                ctx,
                UiProgress {
                    rate: limit_ratio,
                    value: 1.0,
                    alpha: (*time as f32 / 30.0),
                },
            )?;

            ui.player.limit_bar.draw_at_time(
                ctx,
                assets,
                0,
                bottom_line
                    * graphics::Matrix4::new_translation(&graphics::Vec3::new(-185.0, -25.0, 0.0)),
            )?;
        }

        ui.player
            .underlay
            .draw_at_time(ctx, assets, 0, bottom_line)?;

        {
            let _lock = ggez::graphics::use_shader(ctx, &assets.ui_shader);
            assets.ui_shader.send(
                ctx,
                UiProgress {
                    rate: self.state.health as f32 / self.data.properties.health as f32,
                    value: 1.0,
                    alpha: 1.0,
                },
            )?;

            ui.player.hp_bar.draw_at_time(
                ctx,
                assets,
                0,
                bottom_line
                    * graphics::Matrix4::new_translation(&graphics::Vec3::new(-35.0, -312.0, 0.0))
                    * graphics::Matrix4::new_nonuniform_scaling(&graphics::Vec3::new(
                        -1.0, 1.0, 1.0,
                    )),
            )?;

            assets.ui_shader.send(
                ctx,
                UiProgress {
                    rate: self.state.spirit_gauge as f32
                        / self.data.properties.max_spirit_gauge as f32,
                    value: 1.0,
                    alpha: 1.0,
                },
            )?;

            ui.player.spirit_bar.draw_at_time(
                ctx,
                assets,
                0,
                bottom_line
                    * graphics::Matrix4::new_translation(&graphics::Vec3::new(-140.0, 315.0, 0.0))
                    * graphics::Matrix4::new_nonuniform_scaling(&graphics::Vec3::new(
                        1.0, 1.0, 1.0,
                    )),
            )?;

            assets.ui_shader.send(
                ctx,
                UiProgress {
                    rate: self.state.meter as f32 / 200_00.0,
                    value: 1.0,
                    alpha: 1.0,
                },
            )?;

            ui.player.meter_bar.draw_at_time(
                ctx,
                assets,
                0,
                bottom_line
                    * graphics::Matrix4::new_translation(&graphics::Vec3::new(-170.0, 266.0, 0.0))
                    * graphics::Matrix4::new_nonuniform_scaling(&graphics::Vec3::new(
                        1.0, 1.0, 1.0,
                    )),
            )?;
        }
        ui.player
            .overlay
            .draw_at_time(ctx, assets, 0, bottom_line)?;

        // draw shield graphics

        ggez::graphics::set_transform(
            ctx,
            bottom_line
                * graphics::Matrix4::new_translation(&graphics::Vec3::new(-45.0, 230.0, 0.0)),
        );
        ggez::graphics::apply_transformations(ctx)?;
        let shield = if self.state.lockout > 0 {
            &ui.shield.disabled
        } else if self.state.meter >= 50_00 {
            &ui.shield.active
        } else {
            &ui.shield.passive
        };
        ggez::graphics::draw(ctx, shield, ggez::graphics::DrawParam::default())?;

        // draw round win markers

        for idx in 0..first_to {
            ggez::graphics::set_transform(
                ctx,
                bottom_line
                    * graphics::Matrix4::new_translation(&graphics::Vec3::new(
                        191.0 - idx as f32 * 25.0,
                        -279.0,
                        0.0,
                    )),
            );
            ggez::graphics::apply_transformations(ctx)?;
            ggez::graphics::draw(
                ctx,
                &ui.player.underlay_round_windicator,
                ggez::graphics::DrawParam::default(),
            )?;
            if idx < wins {
                ggez::graphics::set_transform(
                    ctx,
                    bottom_line
                        * graphics::Matrix4::new_translation(&graphics::Vec3::new(
                            191.0 - idx as f32 * 25.0,
                            -279.0,
                            0.0,
                        )),
                );
                ggez::graphics::apply_transformations(ctx)?;
                ggez::graphics::draw(
                    ctx,
                    &ui.player.round_windicator,
                    ggez::graphics::DrawParam::default(),
                )?;
            }
            ggez::graphics::set_transform(
                ctx,
                bottom_line
                    * graphics::Matrix4::new_translation(&graphics::Vec3::new(
                        190.0 - idx as f32 * 25.0,
                        -280.0,
                        0.0,
                    )),
            );
            ggez::graphics::apply_transformations(ctx)?;
            ggez::graphics::draw(
                ctx,
                &ui.player.overlay_round_windicator,
                ggez::graphics::DrawParam::default(),
            )?;
        }

        Ok(())
    }
}
