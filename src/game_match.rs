mod match_settings;
mod noop_writer;
pub mod sounds;

pub use match_settings::{MatchSettings, MatchSettingsBuilder, MatchSettingsError};

use crate::hitbox::PositionedHitbox;
use crate::input::InputState;
use crate::netcode::{InputSet, RollbackableGameState};
use crate::roster::generic_character::hit_info::{HitAction, HitEffect, HitResult, HitSource};
use crate::roster::generic_character::GenericCharacterBehaviour;
use crate::roster::CharacterBehavior;
use crate::roster::{Yuyuko, YuyukoPlayer};
use crate::stage::Stage;
use crate::typedefs::collision::IntoGraphical;
use crate::typedefs::graphics::{Matrix4, Vec3};
use crate::typedefs::player::PlayerData;
use gfx::{self, *};
use ggez::graphics::{self, Rect};
use ggez::{Context, GameResult};
use noop_writer::NoopWriter;
use sounds::{GlobalSound, SoundList};
use std::io::Write;
use std::path::PathBuf;
use std::rc::Rc;
use strum::IntoEnumIterator;

#[derive(Clone)]
pub struct PlayArea {
    pub width: i32,
}
gfx_defines! { constant Shadow { rate: f32 = "u_Rate", } }

#[derive(Debug, Clone)]
pub struct GameState {
    current_frame: i16,
}

pub struct Match<Writer> {
    players: PlayerData<CharacterBehavior>,
    game_state: GameState,

    background: Stage,
    shader: graphics::Shader<Shadow>,
    play_area: PlayArea,
    writer: Writer,
    sounds: SoundList<sounds::GlobalSound>,
}

pub type NoLogMatch = Match<NoopWriter>;

impl<Writer: Write> Match<Writer> {
    pub fn new(ctx: &mut Context, settings: MatchSettings, mut writer: Writer) -> GameResult<Self> {
        let background = Stage::new(ctx, "\\bg_14.png")?;
        let resources = Rc::new(Yuyuko::new_with_path(
            ctx,
            PathBuf::from(".\\resources\\yuyuko.json"),
        )?);
        let mut p1: CharacterBehavior = YuyukoPlayer::new(Rc::clone(&resources)).into();
        let mut p2: CharacterBehavior = YuyukoPlayer::new(Rc::clone(&resources)).into();
        p1.position_mut().x = -100_00;
        p2.position_mut().x = 100_00;

        let mut sounds = SoundList::new();
        let mut path = PathBuf::from(".\\resources\\sounds");
        for sound in GlobalSound::iter() {
            path.push(format!("{}.mp3", sound));
            use rodio::source::Source;
            let source =
                rodio::decoder::Decoder::new(std::io::BufReader::new(std::fs::File::open(&path)?))
                    .unwrap();
            let source = rodio::buffer::SamplesBuffer::new(
                source.channels(),
                source.sample_rate(),
                source.convert_samples().collect::<Vec<_>>(),
            )
            .buffered();

            sounds.data.insert(sound, source);
            path.pop();
        }

        let _ = bincode::serialize_into(&mut writer, &settings);

        Ok(Self {
            players: [p1, p2].into(),
            game_state: GameState { current_frame: 0 },
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
            writer,
            sounds,
        })
    }

    pub fn current_frame(&self) -> i16 {
        self.game_state.current_frame
    }

    pub fn update(&mut self, input: PlayerData<&[InputState]>) {
        let _ = bincode::serialize_into(&mut self.writer, &self.game_state.current_frame);

        for (player, input) in self.players.iter_mut().zip(input.iter()) {
            if let Some(last_input) = &input.last() {
                let _ = bincode::serialize_into(&mut self.writer, &last_input);
                player.update_frame_mut(input, &self.play_area);
            } else {
                dbg!("skipped a frame");
            }
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
                (p1.velocity().x, p2.velocity().x),
                p1.facing(),
            );
            p1.position_mut().x += p1_mod;
            p2.position_mut().x += p2_mod;
        }

        let (p1, p2) = self.players.both_mut();

        let touched = [
            PositionedHitbox::overlaps_any(&p2.hitboxes(), &p1.hurtboxes()),
            PositionedHitbox::overlaps_any(&p1.hitboxes(), &p2.hurtboxes()),
        ];

        let attack_data: Vec<_> = self
            .players
            .iter()
            .map(|player| player.get_attack_data())
            .collect();

        let (hit_effects, hit_results): (Vec<_>, Vec<_>) = self
            .players
            .iter()
            .zip(touched.iter())
            .zip(attack_data.into_iter().rev())
            .zip(input.iter())
            .map(|(((player, touched), attack_data), input)| {
                if *touched && attack_data.is_some() {
                    player.would_be_hit(input, attack_data.unwrap(), None)
                } else {
                    (None, None)
                }
            })
            .unzip();

        for (ref mut player, ref result) in
            self.players.iter_mut().zip(hit_results.into_iter().rev())
        {
            if let Some(result) = result {
                player.deal_hit(result);
            }
        }

        let (p1, p2) = self.players.both_mut();

        use crate::roster::generic_character::BulletMut;

        for mut p1_bullet in p1.bullets_mut() {
            for mut p2_bullet in p2.bullets_mut() {
                if PositionedHitbox::overlaps_any(&p1_bullet.hitboxes(), &p2_bullet.hitboxes()) {
                    // TODO, replace unit parameter with bullet tier/hp system
                    p1_bullet.on_touch_bullet(());
                    p2_bullet.on_touch_bullet(());
                }
            }
        }

        for player in self.players.iter_mut() {
            player.prune_bullets(&self.play_area);
        }

        fn handle_bullets(
            acting: &mut CharacterBehavior,
            reference: &mut CharacterBehavior,
            acting_input: &[InputState],
            mut effect: Option<HitEffect>,
        ) -> (Option<HitEffect>, Vec<HitResult>) {
            let mut results = Vec::new();
            for mut bullet in reference.bullets_mut().filter(|bullet| {
                PositionedHitbox::overlaps_any(&bullet.hitboxes(), &acting.hurtboxes())
            }) {
                let result = acting.would_be_hit(
                    acting_input,
                    HitAction {
                        attack_info: bullet.attack_data(),
                        hash: bullet.hash(),
                        facing: bullet.facing(),
                        source: HitSource::Object,
                    },
                    effect,
                );
                if let Some(result) = result.1 {
                    bullet.deal_hit(&result);
                    results.push(result);
                }
                effect = result.0;
            }
            (effect, results)
        }

        let (p1, p2) = self.players.both_mut();
        let (hit_effects, hit_results): (Vec<_>, Vec<_>) = vec![
            handle_bullets(p1, p2, input.p1(), hit_effects[0]),
            handle_bullets(p2, p1, input.p2(), hit_effects[1]),
        ]
        .into_iter()
        .unzip();

        for (player, results) in self.players.iter_mut().zip(hit_results.into_iter().rev()) {
            for result in results.into_iter() {
                player.deal_hit(&result);
            }
        }

        for (player, effect) in self
            .players
            .iter_mut()
            .zip(hit_effects.into_iter())
            .flat_map(|(player, item)| item.map(|item| (player, item)))
        {
            player.take_hit(effect);
        }

        for player in self.players.iter_mut() {
            player.prune_bullets(&self.play_area);
        }

        self.game_state.current_frame += 1;
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
            {
                let _lock = graphics::use_shader(ctx, &self.shader);
                let skew = Matrix4::new(
                    1.0, -0.7, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
                );
                let world =
                    world * skew * Matrix4::new_nonuniform_scaling(&Vec3::new(1.0, -0.3, 1.0));

                player.draw_shadow(ctx, world)?;
            }

            player.draw(ctx, world)?;

            graphics::set_transform(ctx, Matrix4::identity());
            graphics::apply_transformations(ctx)?;

            graphics::set_blend_mode(ctx, graphics::BlendMode::Alpha)?;
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

    pub fn render_sounds(&mut self, fps: u32) -> GameResult<()> {
        let audio_device = rodio::default_output_device().unwrap();
        for player in self.players.iter_mut() {
            player.render_sound(&audio_device, &self.sounds, fps);
        }
        Ok(())
    }
}

impl<Writer: Write> RollbackableGameState for Match<Writer> {
    type Input = InputState;
    type SavedState = (PlayerData<Vec<u8>>, GameState);

    fn advance_frame(&mut self, input: InputSet<'_, Self::Input>) {
        self.update([input.inputs[0], input.inputs[1]].into())
    }

    fn save_state(&self) -> Self::SavedState {
        (
            self.players.as_ref().map(|player| player.save().unwrap()),
            self.game_state.clone(),
        )
    }

    fn load_state(&mut self, (players, game_state): Self::SavedState) {
        for (player, new_state) in self.players.iter_mut().zip(players.iter().cloned()) {
            let _ = player.load(&new_state);
            // TODO log load error
        }
        self.game_state = game_state;
    }
}
