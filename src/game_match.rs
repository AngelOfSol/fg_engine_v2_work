mod player;

use crate::hitbox::PositionedHitbox;
use crate::input::control_scheme::PadControlScheme;
use crate::input::InputBuffer;
use crate::roster::{Yuyuko, YuyukoState};
use crate::stage::Stage;
use crate::typedefs::collision::IntoGraphical;
use crate::typedefs::graphics::{Matrix4, Vec3};
use gfx::{self, *};
use ggez::event::EventHandler;
use ggez::graphics;
use ggez::timer;
use ggez::{Context, GameResult};
use gilrs::Gilrs;
use player::Player;
use std::path::PathBuf;

pub struct PlayArea {
    pub width: i32,
}
gfx_defines! { constant Shadow { rate: f32 = "u_Rate", } }
pub struct Match {
    p1: Player,
    p2: Player,
    pads_context: Gilrs,
    background: Stage,
    debug_text: graphics::Text,
    shader: graphics::Shader<Shadow>,
    play_area: PlayArea,
}

impl Match {
    pub fn new(ctx: &mut Context, p1: PadControlScheme, p2: PadControlScheme) -> GameResult<Self> {
        let background = Stage::new(ctx, "\\bg_14.png")?;
        let resources = Yuyuko::new_with_path(ctx, PathBuf::from(".\\resources\\yuyuko.json"))?;
        let mut p1_state = YuyukoState::new(&resources);
        let mut p2_state = YuyukoState::new(&resources);
        p1_state.position.x = -100_00;
        p2_state.position.x = 100_00;
        Ok(Self {
            p1: Player {
                state: p1_state,
                resources: resources.clone(),
                control_scheme: p1,
                input: InputBuffer::new(),
            },
            p2: Player {
                state: p2_state,
                resources,
                control_scheme: p2,
                input: InputBuffer::new(),
            },
            pads_context: Gilrs::new()?,
            debug_text: graphics::Text::new(""),
            play_area: PlayArea {
                width: background.width() as i32 * 100, //- 50_00,
            },
            background,
            shader: graphics::Shader::new(
                ctx,
                "/shaders/vertex.glslv",
                "/shaders/fragment.glslf",
                Shadow { rate: 1.0 },
                "Shadow",
                Some(&[graphics::BlendMode::Alpha]),
            )?,
        })
    }
}

impl EventHandler for Match {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while timer::check_update_time(ctx, 60) {
            let mut events = Vec::new();
            while let Some(event) = self.pads_context.next_event() {
                events.push(event);
            }
            let events = events;
            self.p1.update_input(events.iter());
            self.p2.update_input(events.iter());

            self.p1.update(&self.play_area);
            self.p2.update(&self.play_area);

            self.p1.handle_refacing(self.p2.position().x);
            self.p2.handle_refacing(self.p1.position().x);

            if self.p1.collision().overlaps(self.p2.collision()) {
                let (p1_mod, p2_mod) = self.p1.collision().fix_distances(self.p2.collision());
                self.p1.position_mut().x += p1_mod;
                self.p2.position_mut().x += p2_mod;
            }

            let p1_touched =
                PositionedHitbox::overlaps_any(&self.p2.hitboxes(), &self.p1.hurtboxes());
            let p2_touched =
                PositionedHitbox::overlaps_any(&self.p1.hitboxes(), &self.p2.hurtboxes());

            let p1_attack_data = self.p1.get_attack_data();
            let p2_attack_data = self.p2.get_attack_data();

            let p1_hit_type = self.p1.would_be_hit(p1_touched, p2_attack_data);
            let p2_hit_type = self.p2.would_be_hit(p2_touched, p1_attack_data);

            self.p1.deal_hit(&p2_hit_type);
            self.p2.deal_hit(&p1_hit_type);

            self.p1.take_hit(&p1_hit_type);
            self.p2.take_hit(&p2_hit_type);

            {
                let (p1_context, p1_bullets) = self.p1.bullets_mut();
                let (p2_context, p2_bullets) = self.p2.bullets_mut();

                for p1_bullet in p1_bullets.iter_mut() {
                    for p2_bullet in p2_bullets.iter_mut() {
                        if PositionedHitbox::overlaps_any(
                            &p1_bullet.hitbox(p1_context.bullets),
                            &p2_bullet.hitbox(p2_context.bullets),
                        ) {
                            // TODO, replace unit parameter with bullet tier/hp system
                            p1_bullet.on_touch_bullet(p1_context.bullets, ());
                            p2_bullet.on_touch_bullet(p2_context.bullets, ());
                        }
                    }
                }
            }

            self.p1.prune_bullets(&self.play_area);
            self.p2.prune_bullets(&self.play_area);

            let p1_hurtboxes = self.p1.hurtboxes();
            let p2_hurtboxes = self.p2.hurtboxes();

            let (p1_hitby, p2_hitby) = {
                let p2_hitby: Vec<_> = {
                    let (p2, (p1_context, p1_bullets)) = (&self.p2, self.p1.bullets_mut());
                    p1_bullets
                        .iter_mut()
                        .filter(|bullet| {
                            PositionedHitbox::overlaps_any(
                                &bullet.hitbox(p1_context.bullets),
                                &p2_hurtboxes,
                            )
                        })
                        .map(|bullet| {
                            let result = p2.would_be_hit(
                                true,
                                Some(bullet.attack_data(p1_context.bullets, p1_context.attacks)),
                            );
                            // side effect
                            bullet.on_touch(p1_context.bullets, &result);

                            result
                        })
                        .collect()
                };

                let p1_hitby: Vec<_> = {
                    let (p1, (p2_context, p2_bullets)) = (&self.p1, self.p2.bullets_mut());
                    p2_bullets
                        .iter_mut()
                        .filter(|bullet| {
                            PositionedHitbox::overlaps_any(
                                &bullet.hitbox(p2_context.bullets),
                                &p1_hurtboxes,
                            )
                        })
                        .map(|bullet| {
                            let result = p1.would_be_hit(
                                true,
                                Some(bullet.attack_data(p2_context.bullets, p2_context.attacks)),
                            );

                            // side effect
                            bullet.on_touch(p2_context.bullets, &result);

                            result
                        })
                        .collect()
                };

                (p1_hitby, p2_hitby)
            };

            for attack_info in &p2_hitby {
                self.p2.take_hit(&attack_info);
            }
            for attack_info in &p1_hitby {
                self.p1.take_hit(&attack_info);
            }

            for attack_info in &p2_hitby {
                self.p2.deal_hit(&attack_info);
            }
            for attack_info in &p1_hitby {
                self.p1.deal_hit(&attack_info);
            }

            self.p1.prune_bullets(&self.play_area);
            self.p2.prune_bullets(&self.play_area);

            // compare bullets against opponents bullets
            // apply damage to each bullet
            // remove bullets that are dead

            // compare player against bullets
            //take_bullet_hit?take_hit?
            //deal_bullet_hit (pass an ID)
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        let game_offset =
            Matrix4::new_translation(&Vec3::new(graphics::drawable_size(ctx).0 / 2.0, 660.0, 0.0));

        let p1_x = self.p1.position().x.into_graphical();
        let p2_x = self.p2.position().x.into_graphical();

        let center_point = (p1_x + p2_x) / 2.0;
        let dist = (p1_x - p2_x).abs();

        // min zoom level, determined by our camera size vs how big are image is
        // this is a number between 0 and 1 because the background will usually be greater
        // in width than the camera size, so to get it to render all in the camera (at min zoom out)
        // we need to make it smaller
        let min_scale = graphics::drawable_size(ctx).0 / self.background.width();
        // max allowed zoom level
        let max_scale = 2.0;

        // this is our scaling factor
        // it is in the range [min_scale, max_scale] so we don't zoom to far in or out
        // its relative to the distance between characters, and the size of the camera
        // we add a constant so the characters try to float in the inside edges
        // rather than right next to the edge of the screen
        let factor = graphics::drawable_size(ctx).0 / (dist + 140.0);
        let scaling = f32::min(f32::max(factor, min_scale), max_scale);

        // this is how much we can move the camera horizontally either way
        // we have to componensate the give from the camera size via the scaling
        // ie this is how much area between the edge of the camera if it was centered
        // and the edge of the background
        let give_factor =
            ((self.background.width() - graphics::drawable_size(ctx).0 / scaling) / 2.0).abs();
        // otherwise we just translate it by the center_point, so the player characters are centered
        let translate = f32::min(give_factor, f32::max(center_point, -give_factor));

        // we apply the scaling and then the translation
        let world = game_offset
            * Matrix4::new_scaling(scaling)
            * Matrix4::new_translation(&Vec3::new(-translate, 0.0, 0.0));

        self.background.draw(ctx, world)?;

        self.p1.draw(ctx, &self.shader, world)?;
        self.p2.draw(ctx, &self.shader, world)?;

        self.p1.draw_particles(ctx, world)?;
        self.p2.draw_particles(ctx, world)?;

        self.p1.draw_bullets(ctx, world)?;
        self.p2.draw_bullets(ctx, world)?;

        graphics::set_transform(ctx, Matrix4::identity());
        graphics::apply_transformations(ctx)?;

        graphics::set_blend_mode(ctx, graphics::BlendMode::Alpha)?;

        self.debug_text.fragments_mut()[0].text = format!(
            "{}, {:?}, {:?}",
            self.p1.state.current_state.0, self.p1.state.current_state.1, self.p1.state.last_hit_by
        );
        graphics::draw(ctx, &self.debug_text, graphics::DrawParam::default())?;

        self.p1
            .draw_ui(ctx, Matrix4::new_translation(&Vec3::new(30.0, 600.0, 0.0)))?;
        self.p2.draw_ui(
            ctx,
            Matrix4::new_translation(&Vec3::new(1130.0, 600.0, 0.0)) * Matrix4::new_scaling(-1.0),
        )?;
        graphics::present(ctx)?;
        Ok(())
    }
}
