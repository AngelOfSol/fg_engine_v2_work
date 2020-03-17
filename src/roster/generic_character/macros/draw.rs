macro_rules! impl_draw_ui {
    () => {
        fn draw_ui(&self, ctx: &mut Context, bottom_line: graphics::Matrix4) -> GameResult<()> {
            ggez::graphics::set_transform(ctx, bottom_line);
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

            // draw HP bar

            ggez::graphics::set_transform(
                ctx,
                graphics::Matrix4::new_translation(&graphics::Vec3::new(0.0, -400.0, 0.0))
                    * bottom_line,
            );
            ggez::graphics::apply_transformations(ctx)?;

            let hp_length = 300.0;
            let hp_current = ggez::graphics::Rect::new(
                0.0,
                0.0,
                hp_length * self.state.health as f32 / self.data.properties.health as f32,
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

            Ok(())
        }
    };
}

macro_rules! impl_draw {
    () => {
        fn draw(&self, ctx: &mut Context, world: graphics::Matrix4) -> GameResult<()> {
            let (frame, move_id) = self.state.current_state;

            let collision = &self.data.states[&move_id].hitboxes.at_time(frame).collision;
            let position = world
                * graphics::Matrix4::new_translation(&graphics::up_dimension(
                    self.state.position.into_graphical(),
                ));

            self.data.states[&move_id].draw_at_time(
                ctx,
                &self.data.assets,
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
        fn draw_particles(&self, ctx: &mut Context, world: graphics::Matrix4) -> GameResult<()> {
            for (frame, position, id) in &self.state.particles {
                self.data.particles[&id].draw_at_time(
                    ctx,
                    &self.data.assets,
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
        fn draw_bullets(&self, ctx: &mut Context, world: graphics::Matrix4) -> GameResult<()> {
            for bullet in &self.state.bullets {
                bullet.draw(ctx, &self.data, &self.data.assets, world)?;
            }

            Ok(())
        }
    };
}

macro_rules! impl_draw_shadow {
    () => {
        fn draw_shadow(&self, ctx: &mut Context, world: graphics::Matrix4) -> GameResult<()> {
            let (frame, move_id) = self.state.current_state;

            let collision = &self.data.states[&move_id].hitboxes.at_time(frame).collision;
            let position = world
                * graphics::Matrix4::new_translation(&graphics::up_dimension(
                    self.state.position.into_graphical(),
                ));

            self.data.states[&move_id].draw_shadow_at_time(
                ctx,
                &self.data.assets,
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
