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
