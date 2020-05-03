mod flash;
mod match_settings;
mod noop_writer;
pub mod sounds;

pub use flash::FlashType;
pub use match_settings::{MatchSettings, MatchSettingsBuilder, MatchSettingsError};

use crate::assets::Assets;
use crate::character::state::components::GlobalParticle;
use crate::graphics::particle::Particle;
use crate::hitbox::PositionedHitbox;
use crate::input::Facing;
use crate::input::InputState;
use crate::netcode::{InputSet, RollbackableGameState};
use crate::roster::generic_character::hit_info::{
    HitAction, HitEffect, HitEffectType, HitResult, HitSource,
};
use crate::roster::generic_character::GenericCharacterBehaviour;
use crate::roster::generic_character::OpaqueStateData;
use crate::roster::CharacterBehavior;
use crate::roster::{Yuyuko, YuyukoPlayer};
use crate::stage::Stage;
use crate::typedefs::collision::IntoGraphical;
use crate::typedefs::graphics::{Matrix4, Vec3};
use crate::typedefs::player::PlayerData;
use flash::FlashOverlay;
use gfx::{self, *};
use ggez::graphics::Image;
use ggez::graphics::{self, Rect};
use ggez::{Context, GameResult};
use noop_writer::NoopWriter;
use sounds::{GlobalSound, SoundList};
use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;
use strum::IntoEnumIterator;

#[derive(Clone)]
pub struct PlayArea {
    pub width: i32,
}
gfx_defines! { constant Shadow { rate: f32 = "u_Rate", } }
gfx_defines! { constant ValueAlpha { value: f32 = "u_Value", alpha: f32 = "u_Alpha", } }

pub type GameShader = graphics::Shader<ValueAlpha>;
pub type ShadowShader = graphics::Shader<Shadow>;

#[derive(Debug, Clone)]
pub struct GameState {
    current_frame: i16,
    flash: Option<FlashOverlay>,
    mode: UpdateMode,

    sound_state: sounds::PlayerSoundState<GlobalSound>,
}

pub struct ShieldUi {
    pub active: Image,
    pub disabled: Image,
    pub passive: Image,
}

pub struct RoundStartUi {
    pub action: Image,
    pub gamestart: Image,
    pub round: [Image; 3],
}

pub struct UiElements {
    pub shield: ShieldUi,
    pub roundstart: RoundStartUi,
}

pub struct Match<Writer> {
    players: PlayerData<CharacterBehavior>,
    game_state: GameState,

    #[allow(dead_code)]
    assets: Assets,

    ui: UiElements,
    background: Stage,
    play_area: PlayArea,
    writer: Writer,
    sounds: SoundList<sounds::GlobalSound>,
    sound_renderer: sounds::SoundRenderer<sounds::GlobalSound>,
    particles: HashMap<GlobalParticle, Particle>,
}

#[derive(Debug, Clone, Copy)]
enum UpdateMode {
    Normal,
    RoundStart { duration: i32 },
    GameStart { duration: i32 },
}

pub type NoLogMatch = Match<NoopWriter>;

impl<Writer: Write> Match<Writer> {
    pub fn new(ctx: &mut Context, settings: MatchSettings, mut writer: Writer) -> GameResult<Self> {
        let mut assets = Assets::new(ctx)?;
        let background = Stage::new(ctx, "\\bg_14.png")?;
        let play_area = PlayArea {
            width: background.width() as i32 * 100,
        };

        let resources =
            Yuyuko::new_with_path(ctx, &mut assets, PathBuf::from(".\\resources\\yuyuko.json"))?;
        let mut p1: CharacterBehavior = YuyukoPlayer::new(resources.clone(), Facing::Right).into();
        let mut p2: CharacterBehavior = YuyukoPlayer::new(resources.clone(), Facing::Left).into();

        p1.validate_position(&play_area);
        p2.validate_position(&play_area);

        p1.position_mut().x = -100_00;
        p2.position_mut().x = 100_00;

        let mut sounds = SoundList::new();

        for path in glob::glob(".\\resources\\global\\sounds\\**\\*.mp3")
            .unwrap()
            .filter_map(Result::ok)
        {
            let sound = path
                .file_stem()
                .and_then(|item| item.to_str())
                .and_then(|item| GlobalSound::from_str(item).ok());
            if let Some(sound) = sound {
                use rodio::source::Source;
                let source = rodio::decoder::Decoder::new(std::io::BufReader::new(
                    std::fs::File::open(&path)?,
                ))
                .unwrap();
                let source = rodio::buffer::SamplesBuffer::new(
                    source.channels(),
                    source.sample_rate(),
                    source.convert_samples().collect::<Vec<_>>(),
                )
                .buffered();

                sounds.data.insert(sound, source);
            }
        }

        let mut particles = HashMap::new();
        let mut path = PathBuf::from(".\\resources\\global\\particles");
        for particle in GlobalParticle::iter() {
            path.push(format!("{}.json", particle));

            particles.insert(
                particle,
                Particle::load_from_json(ctx, &mut assets, path.clone())?,
            );

            path.pop();
        }

        let _ = bincode::serialize_into(&mut writer, &settings);

        Ok(Self {
            players: [p1, p2].into(),
            game_state: GameState {
                current_frame: 0,
                flash: None,
                mode: UpdateMode::GameStart { duration: 210 },
                sound_state: sounds::PlayerSoundState::new(),
            },

            sound_renderer: sounds::SoundRenderer::new(),

            play_area,
            background,

            writer,
            sounds,
            particles,
            assets,

            ui: UiElements {
                shield: ShieldUi {
                    active: Image::new(ctx, "\\global\\ui\\lockout\\active_shield.png")?,
                    disabled: Image::new(ctx, "\\global\\ui\\lockout\\disabled_shield.png")?,
                    passive: Image::new(ctx, "\\global\\ui\\lockout\\passive_shield.png")?,
                },
                roundstart: RoundStartUi {
                    action: Image::new(ctx, "\\global\\ui\\roundstart\\riot.png")?,
                    gamestart: Image::new(ctx, "\\global\\ui\\roundstart\\rrr.png")?,
                    round: [
                        Image::new(ctx, "\\global\\ui\\roundstart\\rift1.png")?,
                        Image::new(ctx, "\\global\\ui\\roundstart\\rift2.png")?,
                        Image::new(ctx, "\\global\\ui\\roundstart\\rift3.png")?,
                    ],
                },
            },
        })
    }

    pub fn current_frame(&self) -> i16 {
        self.game_state.current_frame
    }

    fn update_normal(&mut self, input: PlayerData<&[InputState]>) {
        let positions: Vec<_> = self
            .players
            .iter()
            .rev()
            .map(GenericCharacterBehaviour::position)
            .collect();
        for ((player, input), position) in self
            .players
            .iter_mut()
            .zip(input.iter())
            .zip(positions.into_iter())
        {
            player.update_frame_mut(input, position, &self.play_area, &self.particles);
            self.game_state.flash = player
                .get_flash()
                .map(|item| item.into())
                .or(self.game_state.flash.take());
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
            match effect.hit_type {
                HitEffectType::GuardCrush => {
                    self.game_state.flash = Some(FlashType::GuardCrush.into())
                }
                _ => (),
            }
            player.take_hit(effect, &self.play_area);
        }

        for player in self.players.iter_mut() {
            player.prune_bullets(&self.play_area);
        }

        let lockouts: Vec<_> = self
            .players
            .iter()
            .map(|item| item.get_lockout())
            .rev()
            .collect();
        for (player, (timer, reset)) in self.players.iter_mut().zip(lockouts) {
            player.modify_lockout(timer, reset);
        }
    }

    fn update_midgame(&mut self, input: PlayerData<&[InputState]>) {
        if self.players.iter().any(|player| player.in_cutscene()) {
            for player in self.players.iter_mut() {
                player.update_cutscene(&self.play_area);
                self.game_state.flash = player
                    .get_flash()
                    .map(|item| item.into())
                    .or(self.game_state.flash.take());
            }
        } else {
            self.update_normal(input);
        }
    }

    fn update_pregame(&mut self) {
        let (p1, p2) = self.players.both_mut();

        p1.handle_refacing(p2.position().x);
        p2.handle_refacing(p1.position().x);

        for player in self.players.iter_mut() {
            player.update_roundstart();
        }
    }

    pub fn update(&mut self, input: PlayerData<&[InputState]>) {
        if input.iter().any(|input| input.is_empty()) {
            return;
        }

        let _ = bincode::serialize_into(&mut self.writer, &self.game_state.current_frame);
        for input in input.iter() {
            let _ = bincode::serialize_into(&mut self.writer, &input.last().unwrap());
        }

        self.game_state.mode = match self.game_state.mode {
            UpdateMode::Normal => {
                self.update_midgame(input);

                UpdateMode::Normal
            }
            UpdateMode::GameStart { duration } => {
                self.update_pregame();
                if duration == 0 {
                    UpdateMode::RoundStart { duration: 120 }
                } else {
                    UpdateMode::GameStart {
                        duration: duration - 1,
                    }
                }
            }
            UpdateMode::RoundStart { duration } => {
                if duration <= 15 {
                    self.update_midgame(input);
                } else {
                    self.update_pregame();
                }

                if duration % 10 == 0 {
                    self.game_state.sound_state.play_sound(
                        sounds::ChannelName::Announcer,
                        sounds::GlobalSound::CounterHit,
                    );
                }

                if duration == 0 {
                    UpdateMode::Normal
                } else {
                    UpdateMode::RoundStart {
                        duration: duration - 1,
                    }
                }
            }
        };

        self.game_state.flash = self.game_state.flash.take().and_then(|item| item.update());
        self.game_state.sound_state.update();

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

        let lock = graphics::use_shader(ctx, &self.assets.shader);

        graphics::set_blend_mode(ctx, graphics::BlendMode::Alpha)?;
        self.background.draw(ctx, world)?;

        if let Some(flash) = &self.game_state.flash {
            let overlay = graphics::Image::solid(ctx, 1280, flash.color())?;

            graphics::set_transform(ctx, Matrix4::identity());
            graphics::apply_transformations(ctx)?;
            graphics::draw(ctx, &overlay, graphics::DrawParam::default())?;
        }

        for player in self.players.iter() {
            {
                let _lock = graphics::use_shader(ctx, &self.assets.shadow_shader);
                let skew = Matrix4::new(
                    1.0, -0.7, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
                );
                let world =
                    world * skew * Matrix4::new_nonuniform_scaling(&Vec3::new(1.0, -0.3, 1.0));

                player.draw_shadow(ctx, &self.assets, world)?;
            }

            player.draw(ctx, &self.assets, world)?;

            graphics::set_transform(ctx, Matrix4::identity());
            graphics::apply_transformations(ctx)?;

            graphics::set_blend_mode(ctx, graphics::BlendMode::Alpha)?;
        }

        for player in self.players.iter() {
            player.draw_particles(ctx, &self.assets, world, &self.particles)?;
        }

        for player in self.players.iter() {
            player.draw_bullets(ctx, &self.assets, world)?;
        }

        graphics::set_transform(ctx, Matrix4::identity());
        graphics::apply_transformations(ctx)?;

        drop(lock);

        graphics::set_blend_mode(ctx, graphics::BlendMode::Alpha)?;

        self.players.p1_mut().draw_ui(
            ctx,
            &self.ui,
            Matrix4::new_translation(&Vec3::new(30.0, 720.0, 0.0)),
            false,
        )?;
        self.players.p2_mut().draw_ui(
            ctx,
            &self.ui,
            Matrix4::new_translation(&Vec3::new(1250.0, 720.0, 0.0))
                * Matrix4::new_nonuniform_scaling(&Vec3::new(-1.0, 1.0, 1.0)),
            true,
        )?;

        let _lock = graphics::use_shader(ctx, &self.assets.shader);

        match &self.game_state.mode {
            UpdateMode::Normal => {}
            UpdateMode::GameStart { duration } => {
                use crate::graphics::keyframe::*;

                let animation_duration = 90;

                let alpha_keyframes = Keyframes {
                    frames: vec![
                        Keyframe {
                            frame: 0,
                            value: 0.0,
                            function: EaseType::EaseIn,
                        },
                        Keyframe {
                            frame: 10,
                            value: 1.0,
                            function: EaseType::Constant,
                        },
                        Keyframe {
                            frame: animation_duration - 10,
                            value: 1.0,
                            function: EaseType::EaseOut,
                        },
                        Keyframe {
                            frame: animation_duration,
                            value: 0.0,
                            function: EaseType::EaseOut,
                        },
                    ],
                };

                let image = if *duration < animation_duration as i32 {
                    Some((
                        &self.ui.roundstart.gamestart,
                        animation_duration - *duration as usize,
                    ))
                } else {
                    None
                };
                if let Some((image, duration)) = image {
                    self.assets.shader.send(
                        ctx,
                        ValueAlpha {
                            value: 1.0,
                            alpha: alpha_keyframes.at_time(duration).unwrap_or(0.0),
                        },
                    )?;
                    ggez::graphics::set_transform(
                        ctx,
                        Matrix4::new_translation(&Vec3::new(
                            640.0 - image.width() as f32 / 2.0,
                            360.0 - image.height() as f32 / 2.0,
                            0.0,
                        )),
                    );
                    ggez::graphics::apply_transformations(ctx)?;
                    ggez::graphics::draw(ctx, image, ggez::graphics::DrawParam::default())?;
                }
                self.assets.shader.send(
                    ctx,
                    ValueAlpha {
                        value: 1.0,
                        alpha: 1.0,
                    },
                )?;
            }

            UpdateMode::RoundStart { duration } => {
                use crate::graphics::keyframe::*;

                let animation_duration = 60;

                let alpha_keyframes = Keyframes {
                    frames: vec![
                        Keyframe {
                            frame: 0,
                            value: 0.0,
                            function: EaseType::EaseIn,
                        },
                        Keyframe {
                            frame: 10,
                            value: 1.0,
                            function: EaseType::Constant,
                        },
                        Keyframe {
                            frame: animation_duration - 10,
                            value: 1.0,
                            function: EaseType::EaseOut,
                        },
                        Keyframe {
                            frame: animation_duration,
                            value: 0.0,
                            function: EaseType::EaseOut,
                        },
                    ],
                };

                let image = if *duration < animation_duration as i32 {
                    Some((
                        &self.ui.roundstart.action,
                        animation_duration - *duration as usize,
                    ))
                } else if *duration < animation_duration as i32 * 2 {
                    Some((
                        &self.ui.roundstart.round[0],
                        animation_duration * 2 - *duration as usize,
                    ))
                } else {
                    None
                };
                if let Some((image, duration)) = image {
                    self.assets.shader.send(
                        ctx,
                        ValueAlpha {
                            value: 1.0,
                            alpha: alpha_keyframes.at_time(duration).unwrap_or(0.0),
                        },
                    )?;
                    ggez::graphics::set_transform(
                        ctx,
                        Matrix4::new_translation(&Vec3::new(
                            640.0 - image.width() as f32 / 2.0,
                            360.0 - image.height() as f32 / 2.0,
                            0.0,
                        )),
                    );
                    ggez::graphics::apply_transformations(ctx)?;
                    ggez::graphics::draw(ctx, image, ggez::graphics::DrawParam::default())?;
                }
                self.assets.shader.send(
                    ctx,
                    ValueAlpha {
                        value: 1.0,
                        alpha: 1.0,
                    },
                )?;
            }
        }

        /*
        let test =
            ggez::graphics::Text::new(format!("current_frame: {}", self.game_state.current_frame));

        ggez::graphics::set_transform(ctx, Matrix4::new_translation(&Vec3::new(600.0, 100.0, 0.0)));
        ggez::graphics::apply_transformations(ctx)?;

        ggez::graphics::draw(ctx, &test, ggez::graphics::DrawParam::default());

        */

        crate::graphics::prepare_screen_for_editor(ctx)?;

        Ok(())
    }

    pub fn render_sounds(&mut self, fps: u32, audio_device: &rodio::Device) -> GameResult<()> {
        for player in self.players.iter_mut() {
            player.render_sound(&audio_device, &self.sounds, fps);
        }
        self.sound_renderer.render_frame(
            &audio_device,
            &self.sounds.data,
            &self.game_state.sound_state,
            fps,
        );
        Ok(())
    }
}

impl<Writer: Write> RollbackableGameState for Match<Writer> {
    type Input = InputState;
    type SavedState = (PlayerData<OpaqueStateData>, GameState);

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
        for (player, new_state) in self.players.iter_mut().zip(players.into_iter().cloned()) {
            player.load(new_state).unwrap();
            // TODO log load error
        }
        self.game_state = game_state;
    }
}
