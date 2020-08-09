macro_rules! impl_draw_ui {
    () => {
        fn draw_ui(
            &self,
            ctx: &mut Context,
            assets: &Assets,
            ui: &UiElements,
            bottom_line: graphics::Matrix4,
            flipped: bool,
            wins: usize,
            first_to: usize,
        ) -> GameResult<()> {
            if let Some((combo, timer)) = self.last_combo_state {
                let text = format!(
                    "{} hits\n{} damage\n{} limit",
                    combo.hits,
                    combo.total_damage,
                    combo.available_limit.max(0)
                );

                let mut combo_text = self.combo_text.borrow_mut();

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
                        (timer as f32 / 30.0),
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
                        alpha: (timer as f32 / 30.0),
                    },
                )?;

                ui.player.limit_bar.draw_at_time(
                    ctx,
                    assets,
                    0,
                    bottom_line
                        * graphics::Matrix4::new_translation(&graphics::Vec3::new(
                            -185.0, -25.0, 0.0,
                        )),
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
                        * graphics::Matrix4::new_translation(&graphics::Vec3::new(
                            -35.0, -312.0, 0.0,
                        ))
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
                        * graphics::Matrix4::new_translation(&graphics::Vec3::new(
                            -140.0, 315.0, 0.0,
                        ))
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
                        * graphics::Matrix4::new_translation(&graphics::Vec3::new(
                            -170.0, 266.0, 0.0,
                        ))
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
                //
            }

            Ok(())
        }
    };
}

macro_rules! impl_draw {
    () => {
        fn draw(
            &self,
            ctx: &mut Context,
            assets: &Assets,
            world: graphics::Matrix4,
        ) -> GameResult<()> {
            let (frame, move_id) = self.state.current_state;

            let collision = &self.data.states[&move_id].hitboxes.at_time(frame).collision;
            let position = world
                * graphics::Matrix4::new_translation(&graphics::up_dimension(
                    self.state.position.into_graphical(),
                ));

            self.data.states[&move_id].draw_at_time(
                ctx,
                assets,
                frame,
                position
                    * graphics::Matrix4::new_translation(&graphics::up_dimension(
                        self.state
                            .facing
                            .fix_graphics(-collision.center.into_graphical()),
                    ))
                    * graphics::Matrix4::new_nonuniform_scaling(&graphics::up_dimension(
                        self.state.facing.graphics_multiplier(),
                    )),
            )?;

            Ok(())
        }
    };
}

macro_rules! impl_draw_particles {
    () => {
        fn draw_particles(
            &self,
            ctx: &mut Context,
            assets: &Assets,
            world: graphics::Matrix4,
            global_particles: &HashMap<GlobalParticle, Particle>,
        ) -> GameResult<()> {
            for (frame, position, id) in &self.state.particles {
                let particle = id.get(&self.data.particles, global_particles);

                particle.draw_at_time(
                    ctx,
                    assets,
                    *frame,
                    world
                        * graphics::Matrix4::new_translation(&graphics::up_dimension(
                            position.into_graphical(),
                        )),
                )?;
            }

            Ok(())
        }
    };
}

macro_rules! impl_draw_bullets {
    () => {
        fn draw_bullets(
            &self,
            ctx: &mut Context,
            assets: &Assets,
            world: graphics::Matrix4,
        ) -> GameResult<()> {
            for bullet in &self.state.bullets {
                bullet.draw(ctx, &self.data, assets, world)?;
            }

            Ok(())
        }
    };
}

macro_rules! impl_draw_shadow {
    () => {
        fn draw_shadow(
            &self,
            ctx: &mut Context,
            assets: &Assets,
            world: graphics::Matrix4,
        ) -> GameResult<()> {
            let (frame, move_id) = self.state.current_state;

            let collision = &self.data.states[&move_id].hitboxes.at_time(frame).collision;
            let position = world
                * graphics::Matrix4::new_translation(&graphics::up_dimension(
                    self.state.position.into_graphical(),
                ));

            self.data.states[&move_id].draw_shadow_at_time(
                ctx,
                assets,
                frame,
                position
                    * graphics::Matrix4::new_translation(&graphics::up_dimension(
                        self.state
                            .facing
                            .fix_graphics(-collision.center.into_graphical()),
                    ))
                    * graphics::Matrix4::new_nonuniform_scaling(&graphics::up_dimension(
                        self.state.facing.graphics_multiplier(),
                    )),
            )?;
            Ok(())
        }
    };
}
