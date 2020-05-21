macro_rules! impl_draw_ui {
    () => {
        fn draw_ui(
            &self,
            ctx: &mut Context,
            ui: &UiElements,
            bottom_line: graphics::Matrix4,
            flipped: bool,
            wins: usize,
            first_to: usize,
        ) -> GameResult<()> {
            ggez::graphics::set_transform(
                ctx,
                bottom_line
                    * graphics::Matrix4::new_translation(&graphics::Vec3::new(0.0, -50.0, 0.0)),
            );
            ggez::graphics::apply_transformations(ctx)?;
            ggez::graphics::set_blend_mode(ctx, ggez::graphics::BlendMode::Alpha)?;

            let spirit_current = ggez::graphics::Rect::new(
                0.0,
                0.0,
                100.0 * self.state.spirit_gauge as f32
                    / self.data.properties.max_spirit_gauge as f32,
                20.0,
            );
            let spirit_backdrop = ggez::graphics::Rect::new(0.0, 0.0, 100.0, 20.0);
            let spirit_max = ggez::graphics::Rect::new(-5.0, -5.0, 110.0, 30.0);

            let rect = ggez::graphics::Mesh::new_rectangle(
                ctx,
                ggez::graphics::DrawMode::Fill(ggez::graphics::FillOptions::default()),
                spirit_max,
                ggez::graphics::Color::new(0.0, 0.0, 0.0, 1.0),
            )?;

            ggez::graphics::draw(ctx, &rect, ggez::graphics::DrawParam::default())?;

            let rect = ggez::graphics::Mesh::new_rectangle(
                ctx,
                ggez::graphics::DrawMode::Fill(ggez::graphics::FillOptions::default()),
                spirit_backdrop,
                ggez::graphics::Color::new(1.0, 1.0, 1.0, 1.0),
            )?;

            ggez::graphics::draw(ctx, &rect, ggez::graphics::DrawParam::default())?;

            let rect = ggez::graphics::Mesh::new_rectangle(
                ctx,
                ggez::graphics::DrawMode::Fill(ggez::graphics::FillOptions::default()),
                spirit_current,
                ggez::graphics::Color::new(0.0, 0.0, 1.0, 1.0),
            )?;

            ggez::graphics::draw(ctx, &rect, ggez::graphics::DrawParam::default())?;

            // draw shield graphics

            ggez::graphics::set_transform(
                ctx,
                bottom_line
                    * graphics::Matrix4::new_translation(&graphics::Vec3::new(0.0, -95.0, 0.0))
                    * graphics::Matrix4::new_translation(&graphics::Vec3::new(110.0, 0.0, 0.0)),
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

            // draw meter

            ggez::graphics::set_transform(
                ctx,
                bottom_line
                    * graphics::Matrix4::new_translation(&graphics::Vec3::new(0.0, -100.0, 0.0)),
            );
            ggez::graphics::apply_transformations(ctx)?;

            let meter_current = ggez::graphics::Rect::new(
                0.0,
                0.0,
                100.0 * self.state.meter as f32 / 200_00.0,
                20.0,
            );
            let meter_backdrop = ggez::graphics::Rect::new(0.0, 0.0, 100.0, 20.0);
            let meter_max = ggez::graphics::Rect::new(-5.0, -5.0, 110.0, 30.0);

            let rect = ggez::graphics::Mesh::new_rectangle(
                ctx,
                ggez::graphics::DrawMode::Fill(ggez::graphics::FillOptions::default()),
                meter_max,
                ggez::graphics::Color::new(0.0, 0.0, 0.0, 1.0),
            )?;

            ggez::graphics::draw(ctx, &rect, ggez::graphics::DrawParam::default())?;

            let rect = ggez::graphics::Mesh::new_rectangle(
                ctx,
                ggez::graphics::DrawMode::Fill(ggez::graphics::FillOptions::default()),
                meter_backdrop,
                ggez::graphics::Color::new(1.0, 1.0, 1.0, 1.0),
            )?;

            ggez::graphics::draw(ctx, &rect, ggez::graphics::DrawParam::default())?;

            let rect = ggez::graphics::Mesh::new_rectangle(
                ctx,
                ggez::graphics::DrawMode::Fill(ggez::graphics::FillOptions::default()),
                meter_current,
                ggez::graphics::Color::new(1.0, 0.0, 0.0, 1.0),
            )?;

            ggez::graphics::draw(ctx, &rect, ggez::graphics::DrawParam::default())?;

            // draw round win markers

            let win_box = ggez::graphics::Image::solid(ctx, 20, ggez::graphics::BLACK)?;
            let tick_win_box = ggez::graphics::Image::solid(ctx, 15, ggez::graphics::WHITE)?;

            for idx in 0..first_to {
                ggez::graphics::set_transform(
                    ctx,
                    bottom_line
                        * graphics::Matrix4::new_translation(&graphics::Vec3::new(
                            320.0 + idx as f32 * 25.0,
                            -700.0,
                            0.0,
                        )),
                );
                ggez::graphics::apply_transformations(ctx)?;
                ggez::graphics::draw(ctx, &win_box, ggez::graphics::DrawParam::default())?;
                if idx < wins {
                    ggez::graphics::set_transform(
                        ctx,
                        bottom_line
                            * graphics::Matrix4::new_translation(&graphics::Vec3::new(
                                322.5 + idx as f32 * 20.0,
                                -697.5,
                                0.0,
                            )),
                    );
                    ggez::graphics::apply_transformations(ctx)?;
                    ggez::graphics::draw(ctx, &tick_win_box, ggez::graphics::DrawParam::default())?;
                }
                //
            }

            // draw HP bar

            ggez::graphics::set_transform(
                ctx,
                bottom_line
                    * graphics::Matrix4::new_translation(&graphics::Vec3::new(0.0, -700.0, 0.0)),
            );
            ggez::graphics::apply_transformations(ctx)?;

            let hp_length = 300.0;
            let hp_current = ggez::graphics::Rect::new(
                0.0,
                0.0,
                (hp_length * self.state.health as f32 / self.data.properties.health as f32)
                    .max(0.0),
                20.0,
            );
            let hp_backdrop = ggez::graphics::Rect::new(0.0, 0.0, hp_length, 20.0);
            let hp_max = ggez::graphics::Rect::new(-5.0, -5.0, hp_length + 10.0, 30.0);

            let rect = ggez::graphics::Mesh::new_rectangle(
                ctx,
                ggez::graphics::DrawMode::Fill(ggez::graphics::FillOptions::default()),
                hp_max,
                ggez::graphics::Color::new(0.0, 0.0, 0.0, 1.0),
            )?;

            ggez::graphics::draw(ctx, &rect, ggez::graphics::DrawParam::default())?;

            let rect = ggez::graphics::Mesh::new_rectangle(
                ctx,
                ggez::graphics::DrawMode::Fill(ggez::graphics::FillOptions::default()),
                hp_backdrop,
                ggez::graphics::Color::new(1.0, 1.0, 1.0, 1.0),
            )?;

            ggez::graphics::draw(ctx, &rect, ggez::graphics::DrawParam::default())?;

            let rect = ggez::graphics::Mesh::new_rectangle(
                ctx,
                ggez::graphics::DrawMode::Fill(ggez::graphics::FillOptions::default()),
                hp_current,
                ggez::graphics::Color::new(0.0, 1.0, 0.0, 1.0),
            )?;

            ggez::graphics::draw(ctx, &rect, ggez::graphics::DrawParam::default())?;

            // draw meter text

            let meter_text = ggez::graphics::Text::new(format!("{}", self.state.meter / 100));
            let width = meter_text.width(ctx) as f32;

            ggez::graphics::set_transform(
                ctx,
                graphics::Matrix4::new_translation(&graphics::Vec3::new(
                    if flipped { -width - 40.0 } else { 40.0 },
                    -97.5,
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
                &meter_text,
                ggez::graphics::DrawParam::default()
                    .color(ggez::graphics::Color::new(0.0, 0.0, 0.0, 1.0)),
            )?;

            if let Some((combo, timer)) = self.last_combo_state {
                let mut hits_text = ggez::graphics::Text::new(format!(
                    "{} hits\n{} damage\n{} limit",
                    combo.hits,
                    combo.total_damage,
                    combo.available_limit.max(0)
                ));
                hits_text.set_font(ui.font, ggez::graphics::Scale::uniform(30.0));
                hits_text.set_bounds(
                    [hits_text.width(ctx) as f32, 400.0],
                    if flipped {
                        ggez::graphics::Align::Right
                    } else {
                        ggez::graphics::Align::Left
                    },
                );
                let width = hits_text.width(ctx) as f32;

                ggez::graphics::set_transform(
                    ctx,
                    graphics::Matrix4::new_translation(&graphics::Vec3::new(
                        if flipped { -width - 10.0 } else { 10.0 },
                        -500.0,
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
                    &hits_text,
                    ggez::graphics::DrawParam::default().color(ggez::graphics::Color::new(
                        1.0,
                        1.0,
                        1.0,
                        (timer as f32 / 30.0),
                    )),
                )?;

                ggez::graphics::set_transform(
                    ctx,
                    bottom_line
                        * graphics::Matrix4::new_translation(&graphics::Vec3::new(
                            if flipped { -10.0 } else { 10.0 },
                            -370.0,
                            0.0,
                        )),
                );
                ggez::graphics::apply_transformations(ctx)?;

                let limit_current =
                    ggez::graphics::Rect::new(0.0, 0.0, combo.available_limit.max(0) as f32, 4.0);

                let rect = ggez::graphics::Mesh::new_rectangle(
                    ctx,
                    ggez::graphics::DrawMode::Fill(ggez::graphics::FillOptions::default()),
                    limit_current,
                    ggez::graphics::Color::new(1.0, 1.0, 1.0, (timer as f32 / 30.0)),
                )?;

                ggez::graphics::draw(ctx, &rect, ggez::graphics::DrawParam::default())?;
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

            /*
            let test = ggez::graphics::Text::new(format!(
                "{}, {}",
                self.state.current_state.0, self.state.current_state.1
            ));

            ggez::graphics::set_transform(
                ctx,
                position
                    * graphics::Matrix4::new_translation(&graphics::Vec3::new(-25.0, -100.0, 0.0)),
            );
            ggez::graphics::apply_transformations(ctx)?;

            ggez::graphics::draw(ctx, &test, ggez::graphics::DrawParam::default());
            */

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
