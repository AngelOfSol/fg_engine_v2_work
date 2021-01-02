mod flash;
mod match_settings;
mod noop_writer;
pub mod sounds;

use crate::input::Facing;
use crate::input::InputState;
use crate::netcode::{InputSet, RollbackableGameState};
use crate::roster::generic_character::GenericCharacterBehaviour;
use crate::roster::generic_character::OpaqueStateData;
use crate::roster::hit_info::Source;
use crate::roster::CharacterBehavior;
use crate::stage::Stage;
use crate::typedefs::collision::IntoGraphical;
use crate::typedefs::graphics::{Matrix4, Vec3};
use crate::typedefs::player::PlayerData;
use crate::{assets::ValueAlpha, roster::hit_info::HitSource};
use crate::{character::state::components::GlobalGraphic, roster::hit_info::OnHitEffect};
use crate::{graphics::animation_group::AnimationGroup, roster::hit_info::HitResultNew};
use crate::{hitbox::PositionedHitbox, roster::hit_info::OnHitType};
use flash::FlashOverlay;
pub use flash::FlashType;
use ggez::graphics::Image;
use ggez::graphics::{self, Rect};
use ggez::{Context, GameResult};
pub use match_settings::{
    load_global_graphics, FromMatchSettings, MatchSettings, MatchSettingsError, RuntimeData,
};
use noop_writer::NoopWriter;
use sounds::{GlobalSound, SoundList};
use std::io::Write;
use std::rc::Rc;

const FRAMES_PER_WS_SECOND: usize = 60;
const WS_SECONDS_PER_ROUND: usize = 99;
const FRAMES_PER_ROUND: usize = WS_SECONDS_PER_ROUND * FRAMES_PER_WS_SECOND;

#[derive(Clone)]
pub struct PlayArea {
    pub width: i32,
}

#[derive(Debug, Clone)]
pub struct GameState {
    current_frame: u32,
    flash: Option<FlashOverlay>,
    mode: UpdateMode,

    wins: PlayerData<usize>,
    round: usize,
    timer: usize,

    p1_install: bool,

    sound_state: sounds::PlayerSoundState<GlobalSound>,
}

#[derive(Clone)]
pub struct ShieldUi {
    pub active: Image,
    pub disabled: Image,
    pub passive: Image,
}

#[derive(Clone)]
pub struct RoundStartUi {
    pub action: AnimationGroup,
    pub gamestart: AnimationGroup,
    pub roundend: AnimationGroup,
    pub round: [AnimationGroup; 5],
}

#[derive(Clone)]
pub struct PlayerUi {
    pub limit_bar: AnimationGroup,
    pub overlay: AnimationGroup,
    pub underlay: AnimationGroup,
    pub hp_bar: AnimationGroup,
    pub spirit_bar: AnimationGroup,
    pub meter_bar: AnimationGroup,
    pub underlay_round_windicator: Image,
    pub overlay_round_windicator: Image,
    pub round_windicator: Image,
}

#[derive(Clone)]
pub struct UiElements {
    pub font: graphics::Font,
    pub shield: ShieldUi,
    pub roundstart: RoundStartUi,
    pub player: PlayerUi,
    pub timer_backdrop: Image,
    pub fade_out_overlay: Image,
}

pub struct Match<Writer> {
    pub players: PlayerData<CharacterBehavior>,
    game_state: GameState,

    game_over: Option<PlayerData<bool>>,

    runtime_data: Rc<RuntimeData>,

    pub settings: MatchSettings,

    writer: Writer,
    sound_renderer: sounds::SoundRenderer<sounds::GlobalSound>,

    text: GameText,
    scale_factor: f32,
}

struct GameText {
    pub timer: ggez::graphics::Text,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UpdateMode {
    Normal,
    RoundStart { duration: i32 },
    GameStart { duration: i32 },
    GameEnd,

    RoundEnd { duration: i32 },
    FadeOut { duration: i32 },
    FadeIn { duration: i32 },
}

pub type NoLogMatch = Match<NoopWriter>;

impl<Writer: Write> Match<Writer> {
    pub fn new(
        ctx: &mut Context,
        mut settings: MatchSettings,
        mut writer: Writer,
    ) -> GameResult<Self> {
        let background = Stage::new(ctx, "/bg_14.png")?;
        let play_area = PlayArea {
            width: background.width() as i32 * 100,
        };

        settings.load(ctx)?;

        let runtime_data = settings.runtime_data.as_ref().unwrap();

        let mut players = runtime_data
            .character_data
            .clone()
            .map(|item| item.make_character());

        players
            .p1_mut()
            .reset_to_position_gamestart(&play_area, -100_00, Facing::Right);
        players
            .p2_mut()
            .reset_to_position_gamestart(&play_area, 100_00, Facing::Left);

        let _ = bincode::serialize_into(&mut writer, &settings);

        let mut timer = ggez::graphics::Text::new("99");

        timer
            .set_bounds([1280.0, 80.0], graphics::Align::Center)
            .set_font(runtime_data.ui.font, graphics::Scale::uniform(38.0));

        Ok(Self {
            players,
            game_state: GameState {
                current_frame: 0,
                flash: None,
                mode: UpdateMode::GameStart { duration: 210 },
                sound_state: sounds::PlayerSoundState::new(),
                wins: [0; 2].into(),
                timer: FRAMES_PER_ROUND,
                round: 1,
                p1_install: false,
            },

            runtime_data: settings.runtime_data.clone().unwrap(),
            settings,
            sound_renderer: sounds::SoundRenderer::new(),

            game_over: None,
            writer,
            scale_factor: 3.6,
            text: GameText { timer },
        })
    }

    pub fn game_over(&self) -> Option<PlayerData<bool>> {
        if self.game_state.mode == UpdateMode::GameEnd {
            Some(
                self.game_state
                    .wins
                    .map(|wins| wins == self.settings.first_to),
            )
        } else {
            None
        }
    }

    pub fn current_frame(&self) -> u32 {
        self.game_state.current_frame
    }

    fn update_normal(&mut self, input: PlayerData<&[InputState]>) {
        self.game_state.timer = self.game_state.timer.saturating_sub(1);

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
            player.update_frame_mut(
                input,
                position,
                &self.runtime_data.play_area,
                &self.runtime_data.graphics,
            );
            let flash = self.game_state.flash.take();
            self.game_state.flash = player.get_flash().map(|item| item.into()).or(flash);
        }

        let (p1, p2) = self.players.both_mut();

        p1.handle_refacing(p2.position().x);
        p2.handle_refacing(p1.position().x);

        p1.apply_pushback(p2.get_pushback(&self.runtime_data.play_area));
        p2.apply_pushback(p1.get_pushback(&self.runtime_data.play_area));

        if p1.collision().overlaps(p2.collision()) {
            let (p1, p2) = if p1.collision().center.y > p2.collision().center.y {
                (p1, p2)
            } else {
                (p2, p1)
            };

            let (p1_mod, p2_mod) = p1.collision().fix_distances(
                p2.collision(),
                &self.runtime_data.play_area,
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
            .map(|player| player.get_attack_data_new())
            .collect();

        let facing: Vec<_> = self.players.iter().map(|item| item.facing()).collect();

        let (hit_effects, hit_types): (Vec<Option<OnHitEffect>>, Vec<Option<OnHitType>>) = self
            .players
            .iter()
            .zip(touched.iter())
            .zip(attack_data.into_iter().rev())
            .zip(facing.into_iter().rev())
            .zip(input.iter())
            .map(|((((player, touched), attack_data), facing), input)| {
                if let (true, Some(ref attack_data)) = (*touched, attack_data) {
                    player.would_be_hit_new(
                        input,
                        attack_data,
                        &Source {
                            source_type: HitSource::Character,
                            facing,
                        },
                        player.get_current_combo(),
                        None,
                    )
                } else {
                    HitResultNew::None
                }
            })
            .map(|item| item.split())
            .unzip();

        for (ref mut player, result) in self.players.iter_mut().zip(hit_types.iter().rev()) {
            if let Some(result) = result {
                player.deal_hit_new(result);
            }
        }

        /*
        let (p1, p2) = self.players.both_mut();
                        for mut p1_bullet in p1.bullets_mut() {
                            for mut p2_bullet in p2.bullets_mut() {
                                if PositionedHitbox::overlaps_any(&p1_bullet.hitboxes(), &p2_bullet.hitboxes()) {
                                    // TODO, replace unit parameter with bullet tier/hp system
                                    p1_bullet.on_touch_bullet();
                                    p2_bullet.on_touch_bullet();
                                }
                            }
                        }

                        for player in self.players.iter_mut() {
                            player.prune_bullets(&self.runtime_data.play_area);
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
        }*/

        for (player, effect) in self
            .players
            .iter_mut()
            .zip(hit_effects.into_iter())
            .flat_map(|(player, item)| item.map(|item| (player, item)))
        {
            if matches!(effect, OnHitEffect::GuardCrush(_)) {
                self.game_state.flash = Some(FlashType::GuardCrush.into())
            }
            player.take_hit_new(&effect, &self.runtime_data.play_area);
        }

        if self.players.iter().any(|player| player.is_dead()) {
            self.game_over = Some(
                self.players
                    .iter()
                    .map(GenericCharacterBehaviour::is_dead)
                    .collect(),
            );
        }

        for player in self.players.iter_mut() {
            player.prune_bullets(&self.runtime_data.play_area);
        }

        match self
            .players
            .p1()
            .draw_order_priority()
            .cmp(&self.players.p2().draw_order_priority())
        {
            std::cmp::Ordering::Greater => {
                self.game_state.p1_install = true;
            }
            std::cmp::Ordering::Less => {
                self.game_state.p1_install = false;
            }
            std::cmp::Ordering::Equal => (),
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
                player.update_cutscene(&self.runtime_data.play_area);
                self.game_state.flash = player
                    .get_flash()
                    .map(|item| item.into())
                    .or(self.game_state.flash);
            }
        } else {
            self.update_normal(input);
        }
    }

    fn update_pregame(&mut self) {
        for player in self.players.iter_mut() {
            player.update_no_input(&self.runtime_data.play_area, &self.runtime_data.graphics);
            self.game_state.flash = player
                .get_flash()
                .map(|item| item.into())
                .or(self.game_state.flash);
        }

        let (p1, p2) = self.players.both_mut();
        p1.handle_refacing(p2.position().x);
        p2.handle_refacing(p1.position().x);
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
            UpdateMode::GameEnd => UpdateMode::GameEnd,
            UpdateMode::Normal => {
                self.update_midgame(input);

                if self.players.iter().any(|player| player.is_dead()) {
                    for (player, wins) in self
                        .players
                        .iter()
                        .zip(self.game_state.wins.iter_mut().rev())
                    {
                        if player.is_dead() {
                            *wins += 1;
                        }
                    }
                    self.game_state.round += 1;
                    UpdateMode::RoundEnd { duration: 120 }
                } else if self.game_state.timer == 0 {
                    match self.players.p1().health().cmp(&self.players.p2().health()) {
                        std::cmp::Ordering::Greater => {
                            *self.game_state.wins.p1_mut() += 1;
                        }
                        std::cmp::Ordering::Less => {
                            *self.game_state.wins.p2_mut() += 1;
                        }
                        std::cmp::Ordering::Equal => (),
                    }
                    self.game_state.round += 1;
                    UpdateMode::RoundEnd { duration: 120 }
                } else {
                    UpdateMode::Normal
                }
            }

            UpdateMode::RoundEnd { duration } => {
                self.update_pregame();

                if duration == 0 {
                    UpdateMode::FadeOut { duration: 30 }
                } else {
                    UpdateMode::RoundEnd {
                        duration: duration - 1,
                    }
                }
            }

            UpdateMode::FadeOut { duration } => {
                self.update_pregame();

                if duration == 0 {
                    if self
                        .game_state
                        .wins
                        .iter()
                        .any(|wins| *wins == self.settings.first_to)
                    {
                        UpdateMode::GameEnd
                    } else {
                        self.game_state.timer = FRAMES_PER_ROUND;
                        self.players.p1_mut().reset_to_position_roundstart(
                            &self.runtime_data.play_area,
                            -100_00,
                            Facing::Right,
                        );
                        self.players.p2_mut().reset_to_position_roundstart(
                            &self.runtime_data.play_area,
                            100_00,
                            Facing::Left,
                        );
                        UpdateMode::FadeIn { duration: 30 }
                    }
                } else {
                    UpdateMode::FadeOut {
                        duration: duration - 1,
                    }
                }
            }

            UpdateMode::FadeIn { duration } => {
                self.update_pregame();

                if duration == 0 {
                    UpdateMode::RoundStart { duration: 120 }
                } else {
                    UpdateMode::FadeIn {
                        duration: duration - 1,
                    }
                }
            }

            UpdateMode::GameStart { duration } => {
                self.update_pregame();
                if duration == 110 {
                    self.game_state.sound_state.play_sound(
                        sounds::ChannelName::Announcer,
                        sounds::GlobalSound::GameStart,
                    );
                }

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

                if duration == 120 {
                    let sound = match self.game_state.round {
                        x if x as usize == self.settings.first_to * 2 - 1 => {
                            sounds::GlobalSound::RoundLast
                        }
                        1 => sounds::GlobalSound::Round1,
                        2 => sounds::GlobalSound::Round2,
                        3 => sounds::GlobalSound::Round3,
                        4 => sounds::GlobalSound::Round4,
                        _ => sounds::GlobalSound::RoundLast,
                    };

                    self.game_state
                        .sound_state
                        .play_sound(sounds::ChannelName::Announcer, sound);
                } else if duration == 50 {
                    self.game_state.sound_state.play_sound(
                        sounds::ChannelName::Announcer,
                        sounds::GlobalSound::RoundStart,
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

        let assets = &self.runtime_data.assets;

        let screen = Rect::new(0.0, 0.0, 1280.0, 720.0);

        let game_offset = Matrix4::new_translation(&Vec3::new(screen.w / 2.0, 610.0, 0.0));

        let p1_graphical = self.players.p1().position().into_graphical();
        let p2_graphical = self.players.p2().position().into_graphical();

        let center_point = (p1_graphical.x + p2_graphical.x) / 2.0;
        let dist = (p1_graphical - p2_graphical).abs();

        // min zoom level, determined by our camera size vs how big are image is
        // this is a number between 0 and 1 because the background will usually be greater
        // in width than the camera size, so to get it to render all in the camera (at min zoom out)
        // we need to make it smaller
        let min_scale = screen.w / self.runtime_data.background.width();
        // max allowed zoom level
        let max_scale = 2.0;

        // this is our scaling factor
        // it is in the range [min_scale, max_scale] so we don't zoom to far in or out
        // its relative to the distance between characters, and the size of the camera
        // we add a constant so the characters try to float in the inside edges
        // rather than right next to the edge of the screen
        let factor = (screen.w / (dist.x + 140.0)).min(screen.h / (dist.y + 200.0));
        let iir = 0.1;
        self.scale_factor = self.scale_factor * (1.0 - iir) + factor * iir;
        let scaling = f32::min(f32::max(self.scale_factor, min_scale), max_scale);

        // this is how much we can move the camera horizontally either way
        // we have to componensate the give from the camera size via the scaling
        // ie this is how much area between the edge of the camera if it was centered
        // and the edge of the background
        let give_factor = ((self.runtime_data.background.width() - screen.w / scaling) / 2.0).abs();
        // otherwise we just translate it by the center_point, so the player characters are centered
        let translate = f32::min(give_factor, f32::max(center_point, -give_factor));

        // we apply the scaling and then the translation
        let world = game_offset
            * Matrix4::new_scaling(scaling)
            * Matrix4::new_translation(&Vec3::new(-translate, 0.0, 0.0));

        {
            let _lock = graphics::use_shader(ctx, &assets.shader);
            graphics::set_blend_mode(ctx, graphics::BlendMode::Alpha)?;
            self.runtime_data.background.draw(ctx, world)?;

            if let Some(flash) = &self.game_state.flash {
                let overlay = graphics::Image::solid(ctx, 1280, flash.color())?;

                graphics::set_transform(ctx, Matrix4::identity());
                graphics::apply_transformations(ctx)?;
                graphics::draw(ctx, &overlay, graphics::DrawParam::default())?;
            }

            if self.game_state.p1_install {
                for player in self.players.iter().rev() {
                    {
                        let _lock = graphics::use_shader(ctx, &assets.shadow_shader);
                        let skew = Matrix4::new(
                            1.0, -0.7, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
                            0.0, 1.0,
                        );
                        let world = world
                            * skew
                            * Matrix4::new_nonuniform_scaling(&Vec3::new(1.0, -0.3, 1.0));

                        player.draw_shadow(ctx, &assets, world)?;
                    }

                    player.draw(ctx, &assets, world)?;

                    graphics::set_transform(ctx, Matrix4::identity());
                    graphics::apply_transformations(ctx)?;

                    graphics::set_blend_mode(ctx, graphics::BlendMode::Alpha)?;
                }
            } else {
                for player in self.players.iter() {
                    {
                        let _lock = graphics::use_shader(ctx, &assets.shadow_shader);
                        let skew = Matrix4::new(
                            1.0, -0.7, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
                            0.0, 1.0,
                        );
                        let world = world
                            * skew
                            * Matrix4::new_nonuniform_scaling(&Vec3::new(1.0, -0.3, 1.0));

                        player.draw_shadow(ctx, &assets, world)?;
                    }

                    player.draw(ctx, &assets, world)?;

                    graphics::set_transform(ctx, Matrix4::identity());
                    graphics::apply_transformations(ctx)?;

                    graphics::set_blend_mode(ctx, graphics::BlendMode::Alpha)?;
                }
            }

            for player in self.players.iter() {
                player.draw_objects(ctx, &assets, world, &self.runtime_data.graphics)?;
            }
        }

        graphics::set_transform(ctx, Matrix4::identity());
        graphics::apply_transformations(ctx)?;

        graphics::set_blend_mode(ctx, graphics::BlendMode::Alpha)?;

        let combos = self
            .players
            .as_ref()
            .map(|player| player.get_last_combo_state().clone());
        self.players.p1_mut().draw_ui(
            ctx,
            &assets,
            &self.runtime_data.ui,
            Matrix4::new_translation(&Vec3::new(320.0, 360.0, 0.0)),
            false,
            *self.game_state.wins.p1(),
            self.settings.first_to,
            combos.p2(),
        )?;
        self.players.p2_mut().draw_ui(
            ctx,
            &assets,
            &self.runtime_data.ui,
            Matrix4::new_translation(&Vec3::new(960.0, 360.0, 0.0))
                * Matrix4::new_nonuniform_scaling(&Vec3::new(-1.0, 1.0, 1.0)),
            true,
            *self.game_state.wins.p2(),
            self.settings.first_to,
            combos.p1(),
        )?;

        graphics::set_blend_mode(ctx, graphics::BlendMode::Alpha)?;

        ggez::graphics::set_transform(ctx, Matrix4::new_translation(&Vec3::new(600.0, 10.0, 0.0)));
        ggez::graphics::apply_transformations(ctx)?;

        ggez::graphics::draw(
            ctx,
            &self.runtime_data.ui.timer_backdrop,
            ggez::graphics::DrawParam::default(),
        )?;

        let text = format!("{}", self.game_state.timer / FRAMES_PER_WS_SECOND);

        if self.text.timer.fragments()[0].text != text {
            self.text.timer.fragments_mut()[0] = ggez::graphics::TextFragment::new(text);
        }

        ggez::graphics::set_transform(ctx, Matrix4::new_translation(&Vec3::new(0.0, 30.0, 0.0)));
        ggez::graphics::apply_transformations(ctx)?;

        ggez::graphics::draw(ctx, &self.text.timer, ggez::graphics::DrawParam::default())?;

        let _lock = graphics::use_shader(ctx, &assets.shader);

        if self.game_state.mode == UpdateMode::GameEnd {
            assets.shader.send(
                ctx,
                ValueAlpha {
                    value: 1.0,
                    alpha: 1.0,
                },
            )?;

            graphics::set_transform(ctx, Matrix4::identity());
            graphics::apply_transformations(ctx)?;
            graphics::draw(
                ctx,
                &self.runtime_data.ui.fade_out_overlay,
                graphics::DrawParam::default(),
            )?;
        }

        match &self.game_state.mode {
            UpdateMode::FadeOut { duration } => {
                use crate::graphics::keyframe::*;
                let alpha_keyframes = Keyframes::with_data(
                    vec![
                        (
                            0,
                            Keyframe {
                                value: 1.0,
                                function: EaseType::EaseIn,
                            },
                        ),
                        (
                            30,
                            Keyframe {
                                value: 0.0,
                                function: EaseType::Constant,
                            },
                        ),
                    ],
                    60,
                )
                .unwrap();
                assets.shader.send(
                    ctx,
                    ValueAlpha {
                        value: 1.0,
                        alpha: alpha_keyframes.get_eased(*duration as usize).unwrap_or(1.0),
                    },
                )?;

                graphics::set_transform(ctx, Matrix4::identity());
                graphics::apply_transformations(ctx)?;
                graphics::draw(
                    ctx,
                    &self.runtime_data.ui.fade_out_overlay,
                    graphics::DrawParam::default(),
                )?;
            }
            UpdateMode::FadeIn { duration } => {
                use crate::graphics::keyframe::*;
                let alpha_keyframes = Keyframes::with_data(
                    vec![
                        (
                            0,
                            Keyframe {
                                value: 0.0,
                                function: EaseType::EaseOut,
                            },
                        ),
                        (
                            30,
                            Keyframe {
                                value: 1.0,
                                function: EaseType::Constant,
                            },
                        ),
                    ],
                    60,
                )
                .unwrap();
                assets.shader.send(
                    ctx,
                    ValueAlpha {
                        value: 1.0,
                        alpha: alpha_keyframes.get_eased(*duration as usize).unwrap_or(0.0),
                    },
                )?;

                graphics::set_transform(ctx, Matrix4::identity());
                graphics::apply_transformations(ctx)?;
                graphics::draw(
                    ctx,
                    &self.runtime_data.ui.fade_out_overlay,
                    graphics::DrawParam::default(),
                )?;
            }
            _ => {}
        }

        if let Some((image, duration)) = match &self.game_state.mode {
            UpdateMode::GameEnd
            | UpdateMode::Normal
            | UpdateMode::FadeOut { .. }
            | UpdateMode::FadeIn { .. } => None,
            UpdateMode::RoundEnd { duration } => {
                const ROUNDEND_DURATION: usize = 120;
                let duration = *duration as usize;
                if duration < ROUNDEND_DURATION {
                    Some((
                        &self.runtime_data.ui.roundstart.roundend,
                        ROUNDEND_DURATION - duration,
                    ))
                } else {
                    None
                }
            }
            UpdateMode::GameStart { duration } => {
                let duration = *duration as usize;
                let animation = &self.runtime_data.ui.roundstart.gamestart;

                if duration < animation.duration() {
                    Some((
                        &self.runtime_data.ui.roundstart.gamestart,
                        animation.duration() - duration,
                    ))
                } else {
                    None
                }
            }
            UpdateMode::RoundStart { duration } => {
                let duration = *duration as usize;
                let idx = match self.game_state.round {
                    x if x as usize >= self.settings.first_to * 2 - 1 => 0,
                    x => x,
                }
                .min(self.runtime_data.ui.roundstart.round.len() - 1);
                let pre_duration = self.runtime_data.ui.roundstart.action.duration()
                    + self.runtime_data.ui.roundstart.round[idx].duration();
                if duration < self.runtime_data.ui.roundstart.action.duration() {
                    Some((
                        &self.runtime_data.ui.roundstart.action,
                        self.runtime_data.ui.roundstart.action.duration() - duration,
                    ))
                } else if duration < pre_duration {
                    Some((
                        &self.runtime_data.ui.roundstart.round[idx],
                        pre_duration - duration,
                    ))
                } else {
                    None
                }
            }
        } {
            image.draw_at_time(
                ctx,
                assets,
                duration,
                Matrix4::new_translation(&Vec3::new(640.0, 360.0, 0.0)),
            )?;
        }

        assets.shader.send(
            ctx,
            ValueAlpha {
                value: 1.0,
                alpha: 1.0,
            },
        )?;

        crate::graphics::prepare_screen_for_editor(ctx)?;

        Ok(())
    }

    pub fn render_sounds(&mut self, fps: u32, audio_device: &rodio::Device) -> GameResult<()> {
        for player in self.players.iter_mut() {
            player.render_sound(&audio_device, &self.runtime_data.sounds, fps);
        }
        self.sound_renderer.render_frame(
            &audio_device,
            &self.runtime_data.sounds.data,
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
        for (player, new_state) in self.players.iter_mut().zip(players.iter().cloned()) {
            player.load(new_state).unwrap();
            // TODO log load error
        }
        self.game_state = game_state;
    }
}
