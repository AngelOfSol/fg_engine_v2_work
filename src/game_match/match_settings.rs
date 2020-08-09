use crate::assets::Assets;
use crate::game_match::{
    GlobalParticle, GlobalSound, PlayArea, PlayerUi, RoundStartUi, ShieldUi, SoundList, Stage,
    UiElements,
};
use crate::graphics::particle::Particle;
use crate::player_list::PlayerList;
use crate::roster::Character;
use crate::roster::CharacterData;
use crate::typedefs::player::PlayerData;
use ggez::{graphics, Context, GameResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;
use std::str::FromStr;
use strum::IntoEnumIterator;

#[derive(Clone, Serialize, Deserialize)]
pub struct MatchSettings {
    replay_version: usize,
    pub first_to: usize,
    pub characters: PlayerData<Character>,
    #[serde(skip)]
    pub runtime_data: Option<Rc<RuntimeData>>,
}

#[derive(Clone)]
pub struct RuntimeData {
    pub character_data: PlayerData<CharacterData>,
    pub assets: Assets,
    pub sounds: SoundList<GlobalSound>,
    pub particles: HashMap<GlobalParticle, Particle>,
    pub ui: UiElements,
    pub background: Stage,
    pub play_area: PlayArea,
}

pub enum MatchSettingsError {
    ReplayVersionMismatch,
    DeserializeError(bincode::Error),
}

impl From<bincode::Error> for MatchSettingsError {
    fn from(value: bincode::Error) -> MatchSettingsError {
        MatchSettingsError::DeserializeError(value)
    }
}

impl MatchSettings {
    pub fn new() -> MatchSettings {
        MatchSettings {
            first_to: 2,
            characters: [Character::default(); 2].into(),
            replay_version: crate::typedefs::REPLAY_VERSION,
            runtime_data: None,
        }
    }

    pub fn validate(&self) -> Result<(), MatchSettingsError> {
        if self.replay_version != crate::typedefs::REPLAY_VERSION {
            return Err(MatchSettingsError::ReplayVersionMismatch);
        }

        Ok(())
    }

    pub fn load(&mut self, ctx: &mut Context) -> GameResult<()> {
        match self.runtime_data {
            Some(ref mut data) => {
                let data = Rc::make_mut(data);
                data.character_data = self
                    .characters
                    .iter()
                    .map(|chara| {
                        data.character_data
                            .iter()
                            .find(|data| data.is_for(*chara))
                            .cloned()
                            .map(|item| Ok(item))
                            .unwrap_or_else(|| chara.load_data(ctx, &mut data.assets))
                    })
                    .collect::<GameResult<PlayerData<_>>>()?;
            }
            None => {
                let mut assets = Assets::new(ctx)?;

                let mut sounds = SoundList::new();

                for path in glob::glob("./resources/global/sounds/**/*.mp3")
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

                let background = Stage::new(ctx, "/bg_14.png")?;
                let play_area = PlayArea {
                    width: background.width() as i32 * 100,
                };

                let mut particles = HashMap::new();
                let mut path = PathBuf::from("./resources/global/particles");
                for particle in GlobalParticle::iter() {
                    path.push(format!("{}.json", particle));

                    particles.insert(
                        particle,
                        Particle::load_from_json(ctx, &mut assets, path.clone())?,
                    );

                    path.pop();
                }

                let ui = UiElements {
                    font: graphics::Font::default(), //graphics::Font::new(ctx, "/font.ttf")?,
                    shield: ShieldUi {
                        active: graphics::Image::new(ctx, "/global/ui/lockout/active_shield.png")?,
                        disabled: graphics::Image::new(
                            ctx,
                            "/global/ui/lockout/disabled_shield.png",
                        )?,
                        passive: graphics::Image::new(
                            ctx,
                            "/global/ui/lockout/passive_shield.png",
                        )?,
                    },
                    roundstart: RoundStartUi {
                        action: graphics::Image::new(ctx, "/global/ui/roundstart/riot.png")?,
                        gamestart: graphics::Image::new(ctx, "/global/ui/roundstart/rrr.png")?,
                        roundend: graphics::Image::new(ctx, "/global/ui/roundstart/ruin.png")?,
                        round: [
                            graphics::Image::new(ctx, "/global/ui/roundstart/lastrift.png")?,
                            graphics::Image::new(ctx, "/global/ui/roundstart/rift1.png")?,
                            graphics::Image::new(ctx, "/global/ui/roundstart/rift2.png")?,
                            graphics::Image::new(ctx, "/global/ui/roundstart/rift3.png")?,
                            graphics::Image::new(ctx, "/global/ui/roundstart/rift3.png")?,
                        ],
                    },
                    player: PlayerUi {
                        limit_bar: Particle::load_from_json(
                            ctx,
                            &mut assets,
                            "./resources/global/ui/limit_bar.json".into(),
                        )?,
                        overlay: Particle::load_from_json(
                            ctx,
                            &mut assets,
                            "./resources/global/ui/player_overlay.json".into(),
                        )?,
                        underlay: Particle::load_from_json(
                            ctx,
                            &mut assets,
                            "./resources/global/ui/underlay.json".into(),
                        )?,
                        hp_bar: Particle::load_from_json(
                            ctx,
                            &mut assets,
                            "./resources/global/ui/hp_bar.json".into(),
                        )?,
                        spirit_bar: Particle::load_from_json(
                            ctx,
                            &mut assets,
                            "./resources/global/ui/spirit_bar.json".into(),
                        )?,
                        meter_bar: Particle::load_from_json(
                            ctx,
                            &mut assets,
                            "./resources/global/ui/meter_bar.json".into(),
                        )?,
                        underlay_round_windicator: graphics::Image::new(
                            ctx,
                            "/global/ui/underlay_windicator.png",
                        )?,
                        overlay_round_windicator: graphics::Image::new(
                            ctx,
                            "/global/ui/overlay_windicator.png",
                        )?,
                        round_windicator: graphics::Image::new(ctx, "/global/ui/windicator.png")?,
                    },
                    timer_backdrop: graphics::Image::solid(ctx, 80, graphics::BLACK)?,
                    fade_out_overlay: graphics::Image::solid(ctx, 1280, graphics::BLACK)?,
                };

                self.runtime_data = Some(Rc::new(RuntimeData {
                    character_data: self
                        .characters
                        .clone()
                        .map(|item| item.load_data(ctx, &mut assets))
                        .transpose()?,
                    assets,
                    particles,
                    sounds,
                    ui,
                    background,
                    play_area,
                }));
            }
        }

        Ok(())
    }
}

pub trait FromMatchSettings {
    fn from_settings(
        ctx: &mut Context,
        player_list: PlayerList,
        settings: MatchSettings,
    ) -> GameResult<Box<Self>>;
}
