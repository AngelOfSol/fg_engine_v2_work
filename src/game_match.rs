mod player;

use crate::hitbox::PositionedHitbox;
use crate::input::InputState;
use crate::roster::generic_character::hit_info::HitType;
use crate::roster::generic_character::GenericCharacterBehaviour;
use crate::roster::{Yuyuko, YuyukoState};
use crate::stage::Stage;
use crate::typedefs::collision::IntoGraphical;
use crate::typedefs::graphics::{Matrix4, Vec3};
use crate::typedefs::player::PlayerData;
use gfx::{self, *};
use ggez::graphics::{self, Rect};
use ggez::{Context, GameResult};
use player::Player;
use std::path::PathBuf;
use std::rc::Rc;

pub struct PlayArea {
    pub width: i32,
}
gfx_defines! { constant Shadow { rate: f32 = "u_Rate", } }

pub struct Match {
    players: PlayerData<Player>,
    background: Stage,
    debug_text: graphics::Text,
    shader: graphics::Shader<Shadow>,
    play_area: PlayArea,
}

impl Match {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let background = Stage::new(ctx, "\\bg_14.png")?;
        let resources = Rc::new(Yuyuko::new_with_path(
            ctx,
            PathBuf::from(".\\resources\\yuyuko.json"),
        )?);
        let mut p1_state = YuyukoState::new(Rc::clone(&resources));
        let mut p2_state = YuyukoState::new(Rc::clone(&resources));
        p1_state.position.x = -100_00;
        p2_state.position.x = 100_00;
        Ok(Self {
            players: [Player { state: p1_state }, Player { state: p2_state }].into(),
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

impl Match {
    pub fn update(&mut self, input: PlayerData<&[InputState]>) -> GameResult<()> {
        for (player, input) in self.players.iter_mut().zip(input.iter()) {
            player.update(input, &self.play_area);
        }

        let (p1, p2) = self.players.both_mut();

        p1.handle_refacing(p2.position().x);
        p2.handle_refacing(p1.position().x);

        p1.apply_pushback(p2.get_pushback(&self.play_area));
        p2.apply_pushback(p1.get_pushback(&self.play_area));

        if p1.collision().overlaps(p2.collision()) {
            let (p1, p2) = if p1.collision().center.y > p2.collision().center.y {
                (p1, p2)
            } else {
                (p2, p1)
            };

            let (p1_mod, p2_mod) = p1.collision().fix_distances(
                p2.collision(),
                &self.play_area,
                (p1.state.velocity.x, p2.state.velocity.x),
                p1.state.facing,
            );
            p1.position_mut().x += p1_mod;
            p2.position_mut().x += p2_mod;
        }

        let (p1, p2) = self.players.both_mut();

        let touched = vec![
            PositionedHitbox::overlaps_any(&p2.hitboxes(), &p1.hurtboxes()),
            PositionedHitbox::overlaps_any(&p1.hitboxes(), &p2.hurtboxes()),
        ];

        let attack_data: Vec<_> = self
            .players
            .iter()
            .map(|player| player.get_attack_data())
            .collect();

        let hit_types: Vec<_> = self
            .players
            .iter()
            .zip(touched.into_iter())
            .zip(attack_data.into_iter().rev())
            .zip(input.iter())
            .map(|(((player, touched), attack_data), input)| {
                player.would_be_hit(input, touched, attack_data)
            })
            .collect();

        for (player, hit_type) in self.players.iter_mut().zip(hit_types.iter().rev()) {
            player.deal_hit(hit_type);
        }
        for (player, hit_type) in self.players.iter_mut().zip(hit_types.iter()) {
            player.take_hit(hit_type);
        }

        let (p1, p2) = self.players.both_mut();
        let (p1_context, p1_bullets) = p1.bullets_mut();
        let (p2_context, p2_bullets) = p2.bullets_mut();

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

        for player in self.players.iter_mut() {
            player.prune_bullets(&self.play_area);
        }

        fn handle_bullets(
            acting: &mut Player,
            reference: &mut Player,
            acting_input: &[InputState],
        ) -> Vec<HitType> {
            let (context, bullets) = reference.bullets_mut();
            bullets
                .iter_mut()
                .filter(|bullet| {
                    PositionedHitbox::overlaps_any(
                        &bullet.hitbox(context.bullets),
                        &acting.hurtboxes(),
                    )
                })
                .map(|bullet| {
                    let result = acting.would_be_hit(
                        acting_input,
                        true,
                        Some(bullet.attack_data(context.bullets, context.attacks)),
                    );
                    // side effect
                    bullet.on_touch(context.bullets, &result);

                    result
                })
                .collect()
        }

        let (p1, p2) = self.players.both_mut();
        let hit_info = vec![
            handle_bullets(p1, p2, input.p1()),
            handle_bullets(p2, p1, input.p2()),
        ];

        for (player, hit_info) in self.players.iter_mut().zip(hit_info.iter()) {
            for hit_info in hit_info.iter() {
                player.take_hit(&hit_info);
            }
        }

        for (player, hit_info) in self.players.iter_mut().zip(hit_info.iter().rev()) {
            for hit_info in hit_info.iter() {
                player.deal_hit(&hit_info);
            }
        }

        for player in self.players.iter_mut() {
            player.prune_bullets(&self.play_area);
        }

        Ok(())
    }
    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        crate::graphics::prepare_screen_for_game(ctx)?;

        let screen = Rect::new(0.0, 0.0, 1280.0, 720.0);

        let game_offset = Matrix4::new_translation(&Vec3::new(screen.w / 2.0, 660.0, 0.0));

        let p1_x = self.players.p1().position().x.into_graphical();
        let p2_x = self.players.p2().position().x.into_graphical();

        let center_point = (p1_x + p2_x) / 2.0;
        let dist = (p1_x - p2_x).abs();

        // min zoom level, determined by our camera size vs how big are image is
        // this is a number between 0 and 1 because the background will usually be greater
        // in width than the camera size, so to get it to render all in the camera (at min zoom out)
        // we need to make it smaller
        let min_scale = screen.w / self.background.width();
        // max allowed zoom level
        let max_scale = 2.0;

        // this is our scaling factor
        // it is in the range [min_scale, max_scale] so we don't zoom to far in or out
        // its relative to the distance between characters, and the size of the camera
        // we add a constant so the characters try to float in the inside edges
        // rather than right next to the edge of the screen
        let factor = screen.w / (dist + 140.0);
        let scaling = f32::min(f32::max(factor, min_scale), max_scale);

        // this is how much we can move the camera horizontally either way
        // we have to componensate the give from the camera size via the scaling
        // ie this is how much area between the edge of the camera if it was centered
        // and the edge of the background
        let give_factor = ((self.background.width() - screen.w / scaling) / 2.0).abs();
        // otherwise we just translate it by the center_point, so the player characters are centered
        let translate = f32::min(give_factor, f32::max(center_point, -give_factor));

        // we apply the scaling and then the translation
        let world = game_offset
            * Matrix4::new_scaling(scaling)
            * Matrix4::new_translation(&Vec3::new(-translate, 0.0, 0.0));

        self.background.draw(ctx, world)?;

        for player in self.players.iter() {
            player.draw(ctx, &self.shader, world)?;
        }

        for player in self.players.iter() {
            player.draw_particles(ctx, world)?;
        }

        for player in self.players.iter() {
            player.draw_bullets(ctx, world)?;
        }

        graphics::set_transform(ctx, Matrix4::identity());
        graphics::apply_transformations(ctx)?;

        graphics::set_blend_mode(ctx, graphics::BlendMode::Alpha)?;

        let show_combo = true;
        if show_combo {
            self.debug_text.fragments_mut()[0].text = format!(
                "{}, {}",
                self.players.p1().state.current_state.1,
                self.players.p2().state.current_state.1
            );
            graphics::draw(ctx, &self.debug_text, graphics::DrawParam::default())?;
        }

        self.players
            .p1_mut()
            .draw_ui(ctx, Matrix4::new_translation(&Vec3::new(30.0, 600.0, 0.0)))?;
        self.players.p2_mut().draw_ui(
            ctx,
            Matrix4::new_translation(&Vec3::new(1130.0, 600.0, 0.0)) * Matrix4::new_scaling(-1.0),
        )?;

        crate::graphics::prepare_screen_for_editor(ctx)?;

        Ok(())
    }
}
